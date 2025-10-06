use clap::{Arg, Command};
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;

use crate::audio::{create_temp_audio_file, play_audio_and_cleanup};
use crate::audio_merge::{check_ffmpeg_available, merge_audio_files};
use crate::config::{get_presets_map, list_presets, load_config, Config};
use crate::text_splitter::{check_text_length, split_text, MAX_CHARS};
use crate::voicepeak::{list_emotion, list_narrator, VoicepeakCommand};

pub fn build_cli() -> Command {
    Command::new("voicepeak-cli")
        .version("0.6.0")
        .about("VOICEPEAK CLI wrapper with presets and auto-play")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Text to say (or pipe from stdin)")
                .index(1),
        )
        .arg(
            Arg::new("file")
                .short('t')
                .long("text")
                .value_name("FILE")
                .help("Text file to say")
                .conflicts_with("text"),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .long("out")
                .value_name("FILE")
                .help("Path of output file (optional - will play with mpv if not specified)"),
        )
        .arg(
            Arg::new("narrator")
                .short('n')
                .long("narrator")
                .value_name("NAME")
                .help("Name of voice"),
        )
        .arg(
            Arg::new("emotion")
                .short('e')
                .long("emotion")
                .value_name("EXPR")
                .help("Emotion expression (e.g., happy=50,sad=50)"),
        )
        .arg(
            Arg::new("preset")
                .short('p')
                .long("preset")
                .value_name("NAME")
                .help("Use voice preset (use --list-presets to see available presets)")
                .conflicts_with_all(["narrator", "emotion"]),
        )
        .arg(
            Arg::new("list-narrator")
                .long("list-narrator")
                .help("Print voice list")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list-emotion")
                .long("list-emotion")
                .value_name("NARRATOR")
                .help("Print emotion list for given voice"),
        )
        .arg(
            Arg::new("list-presets")
                .long("list-presets")
                .help("Print available presets")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("speed")
                .long("speed")
                .value_name("VALUE")
                .help("Speed (50 - 200)"),
        )
        .arg(
            Arg::new("pitch")
                .long("pitch")
                .value_name("VALUE")
                .help("Pitch (-300 - 300)"),
        )
        .arg(
            Arg::new("strict-length")
                .long("strict-length")
                .help("Reject input longer than 140 characters (default: false, allows splitting)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("playback-mode")
                .long("playback-mode")
                .value_name("MODE")
                .help("Playback mode: sequential or batch (default: batch)")
                .value_parser(["sequential", "batch"])
                .default_value("batch"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Enable verbose output (show VOICEPEAK debug messages)")
                .action(clap::ArgAction::SetTrue),
        )
}

pub fn handle_matches(matches: clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    if matches.get_flag("list-narrator") {
        list_narrator();
        return Ok(());
    }

    if let Some(narrator) = matches.get_one::<String>("list-emotion") {
        list_emotion(narrator);
        return Ok(());
    }

    if matches.get_flag("list-presets") {
        list_presets(&config);
        return Ok(());
    }

    run_voicepeak(&matches, &config)
}

fn run_voicepeak(
    matches: &clap::ArgMatches,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_text = if let Some(text) = matches.get_one::<String>("text") {
        text.clone()
    } else if let Some(file_path) = matches.get_one::<String>("file") {
        std::fs::read_to_string(file_path)?
    } else if !io::stdin().is_terminal() {
        // Read from stdin if available (pipe input)
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    } else {
        return Err("Either text argument, --text file, or pipe input must be specified".into());
    };

    let presets_map = get_presets_map(config);

    let (narrator, emotion, preset_pitch, preset_speed) =
        if let Some(preset_name) = matches.get_one::<String>("preset") {
            // Explicit preset specified via -p option
            let preset = presets_map
                .get(preset_name)
                .ok_or_else(|| format!("Unknown preset: {}", preset_name))?;
            (
                preset.narrator.clone(),
                preset.get_emotion_string(),
                preset.pitch,
                preset.speed,
            )
        } else if let Some(default_preset_name) = &config.default_preset {
            // No preset specified, but default_preset exists in config
            if let Some(default_preset) = presets_map.get(default_preset_name) {
                // Use default preset, but allow individual overrides
                let narrator = matches
                    .get_one::<String>("narrator")
                    .cloned()
                    .unwrap_or_else(|| default_preset.narrator.clone());
                let emotion = matches
                    .get_one::<String>("emotion")
                    .cloned()
                    .unwrap_or_else(|| default_preset.get_emotion_string());
                let preset_pitch = if matches.get_one::<String>("emotion").is_some() {
                    None // If emotion is overridden, don't use preset pitch
                } else {
                    default_preset.pitch
                };
                let preset_speed = default_preset.speed;
                (narrator, emotion, preset_pitch, preset_speed)
            } else {
                // Default preset not found, fallback to manual settings
                let narrator = matches
                    .get_one::<String>("narrator")
                    .cloned()
                    .ok_or("No narrator specified. Use --narrator option or configure a preset.")?;
                let emotion = matches
                    .get_one::<String>("emotion")
                    .cloned()
                    .unwrap_or_default();
                (narrator, emotion, None, None)
            }
        } else {
            // No preset and no default_preset, use manual settings only
            let narrator = matches
                .get_one::<String>("narrator")
                .cloned()
                .ok_or("No narrator specified. Use --narrator option or configure a preset.")?;
            let emotion = matches
                .get_one::<String>("emotion")
                .cloned()
                .unwrap_or_default();
            (narrator, emotion, None, None)
        };

    let speed = matches
        .get_one::<String>("speed")
        .cloned()
        .or_else(|| preset_speed.map(|s| s.to_string()));
    let pitch = matches
        .get_one::<String>("pitch")
        .cloned()
        .or_else(|| preset_pitch.map(|p| p.to_string()));
    let should_play = matches.get_one::<String>("out").is_none();
    let output_path = matches.get_one::<String>("out").map(PathBuf::from);
    let strict_length = matches.get_flag("strict-length");
    let playback_mode = matches.get_one::<String>("playback-mode").unwrap();
    let verbose = matches.get_flag("verbose");

    if strict_length && !check_text_length(&input_text) {
        return Err(format!(
            "Input text is too long ({} characters). Maximum allowed is {} characters.\nUse without --strict-length to enable automatic splitting.",
            input_text.chars().count(),
            MAX_CHARS
        ).into());
    }

    let text_chunks = split_text(&input_text);

    if text_chunks.len() > 1 {
        println!(
            "Text is too long, splitting into {} parts...",
            text_chunks.len()
        );
    }

    // Check ffmpeg availability for batch mode
    if (playback_mode == "batch" || (!should_play && text_chunks.len() > 1))
        && !check_ffmpeg_available()
    {
        return Err(
            "ffmpeg is required for batch mode and multi-chunk file output.\n\
            Please install ffmpeg or use --playback-mode sequential for auto-play mode.\n\
            Install ffmpeg: https://ffmpeg.org/download.html"
                .into(),
        );
    }

    if should_play {
        // Auto-play mode
        if playback_mode == "sequential" {
            // Sequential mode: generate and play one by one
            for (i, chunk) in text_chunks.iter().enumerate() {
                if text_chunks.len() > 1 {
                    println!("Playing part {}/{}", i + 1, text_chunks.len());
                }

                let temp_path = create_temp_audio_file()?;

                let mut cmd = VoicepeakCommand::new()
                    .text(chunk)
                    .narrator(&narrator)
                    .emotion(&emotion)
                    .output(&temp_path);

                if let Some(speed) = &speed {
                    cmd = cmd.speed(&speed);
                }
                if let Some(pitch) = &pitch {
                    cmd = cmd.pitch(pitch);
                }

                cmd.execute_with_verbose(verbose)?;
                play_audio_and_cleanup(&temp_path)?;
            }
        } else {
            // Batch mode: generate all, merge, then play
            let mut temp_files = Vec::new();

            for (i, chunk) in text_chunks.iter().enumerate() {
                if text_chunks.len() > 1 {
                    println!("Generating part {}/{}", i + 1, text_chunks.len());
                }

                let temp_path = create_temp_audio_file()?;

                let mut cmd = VoicepeakCommand::new()
                    .text(chunk)
                    .narrator(&narrator)
                    .emotion(&emotion)
                    .output(&temp_path);

                if let Some(speed) = &speed {
                    cmd = cmd.speed(&speed);
                }
                if let Some(pitch) = &pitch {
                    cmd = cmd.pitch(pitch);
                }

                cmd.execute_with_verbose(verbose)?;
                temp_files.push(temp_path);
            }

            // Merge and play
            let final_temp = create_temp_audio_file()?;
            let temp_paths: Vec<&std::path::Path> =
                temp_files.iter().map(|p| p.as_path()).collect();

            if text_chunks.len() > 1 {
                println!("Merging audio files...");
                merge_audio_files(&temp_paths, &final_temp)?;
                println!("Merge complete. Playing audio...");
            } else {
                merge_audio_files(&temp_paths, &final_temp)?;
            }

            // Cleanup individual temp files
            for temp_file in temp_files {
                let _ = std::fs::remove_file(temp_file);
            }

            play_audio_and_cleanup(&final_temp)?;
        }
    } else if let Some(output_path) = output_path {
        // File output mode
        let mut temp_files = Vec::new();

        for (i, chunk) in text_chunks.iter().enumerate() {
            if text_chunks.len() > 1 {
                println!("Generating part {}/{}", i + 1, text_chunks.len());
            }

            let temp_path = create_temp_audio_file()?;

            let mut cmd = VoicepeakCommand::new()
                .text(chunk)
                .narrator(&narrator)
                .emotion(&emotion)
                .output(&temp_path);

            if let Some(speed) = &speed {
                cmd = cmd.speed(&speed);
            }
            if let Some(pitch) = &pitch {
                cmd = cmd.pitch(pitch);
            }

            cmd.execute_with_verbose(verbose)?;
            temp_files.push(temp_path);
        }

        // Merge to final output
        let temp_paths: Vec<&std::path::Path> = temp_files.iter().map(|p| p.as_path()).collect();

        if text_chunks.len() > 1 {
            println!("Merging audio files...");
            merge_audio_files(&temp_paths, &output_path)?;
            println!("Merge complete.");
        } else {
            merge_audio_files(&temp_paths, &output_path)?;
        }

        // Cleanup temp files
        for temp_file in temp_files {
            let _ = std::fs::remove_file(temp_file);
        }

        println!("Audio saved to: {}", output_path.display());
    }

    Ok(())
}
