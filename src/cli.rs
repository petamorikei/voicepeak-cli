use clap::{Arg, Command};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::presets::{VoicePreset, get_default_presets, list_presets};
use crate::voicepeak::{list_narrator, list_emotion, VoicepeakCommand};
use crate::audio::{play_audio_and_cleanup, create_temp_audio_file};
use crate::text_splitter::{split_text, check_text_length, MAX_CHARS};

pub fn build_cli() -> Command {
    Command::new("voicepeak-cli")
        .version("0.1.0")
        .about("VOICEPEAK CLI wrapper with presets and auto-play")
        .arg(
            Arg::new("say")
                .short('s')
                .long("say")
                .value_name("TEXT")
                .help("Text to say")
                .conflicts_with("text"),
        )
        .arg(
            Arg::new("text")
                .short('t')
                .long("text")
                .value_name("FILE")
                .help("Text file to say")
                .conflicts_with("say"),
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
                .help("Use voice preset (karin-normal, karin-happy, karin-angry, karin-sad, karin-whisper)")
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
}

pub fn handle_matches(matches: clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let presets = get_default_presets();

    if matches.get_flag("list-narrator") {
        list_narrator();
        return Ok(());
    }

    if let Some(narrator) = matches.get_one::<String>("list-emotion") {
        list_emotion(narrator);
        return Ok(());
    }

    if matches.get_flag("list-presets") {
        list_presets(&presets);
        return Ok(());
    }

    run_voicepeak(&matches, &presets)
}

fn run_voicepeak(matches: &clap::ArgMatches, presets: &HashMap<String, VoicePreset>) 
    -> Result<(), Box<dyn std::error::Error>> {
    
    let input_text = if let Some(text) = matches.get_one::<String>("say") {
        text.clone()
    } else if let Some(file_path) = matches.get_one::<String>("text") {
        std::fs::read_to_string(file_path)?
    } else {
        return Err("Either --say or --text must be specified".into());
    };

    let (narrator, emotion) = if let Some(preset_name) = matches.get_one::<String>("preset") {
        let preset = presets.get(preset_name)
            .ok_or_else(|| format!("Unknown preset: {}", preset_name))?;
        (preset.narrator.clone(), preset.emotion.clone())
    } else {
        let narrator = matches.get_one::<String>("narrator")
            .cloned()
            .unwrap_or_else(|| "夏色花梨".to_string());
        let emotion = matches.get_one::<String>("emotion")
            .cloned()
            .unwrap_or_default();
        (narrator, emotion)
    };

    let speed = matches.get_one::<String>("speed");
    let pitch = matches.get_one::<String>("pitch");
    let should_play = matches.get_one::<String>("out").is_none();
    let output_path = matches.get_one::<String>("out").map(PathBuf::from);
    let strict_length = matches.get_flag("strict-length");

    if strict_length && !check_text_length(&input_text) {
        return Err(format!(
            "Input text is too long ({} characters). Maximum allowed is {} characters.\nUse without --strict-length to enable automatic splitting.",
            input_text.chars().count(),
            MAX_CHARS
        ).into());
    }

    let text_chunks = split_text(&input_text);
    
    if text_chunks.len() > 1 {
        println!("Text is too long, splitting into {} parts...", text_chunks.len());
    }

    if should_play {
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
            
            if let Some(speed) = speed {
                cmd = cmd.speed(speed);
            }
            if let Some(pitch) = pitch {
                cmd = cmd.pitch(pitch);
            }
            
            cmd.execute()?;
            play_audio_and_cleanup(&temp_path)?;
        }
    } else if let Some(output_path) = output_path {
        if text_chunks.len() == 1 {
            let mut cmd = VoicepeakCommand::new()
                .text(&text_chunks[0])
                .narrator(&narrator)
                .emotion(&emotion)
                .output(&output_path);
            
            if let Some(speed) = speed {
                cmd = cmd.speed(speed);
            }
            if let Some(pitch) = pitch {
                cmd = cmd.pitch(pitch);
            }
            
            cmd.execute()?;
        } else {
            return Err("Cannot save multiple chunks to a single file. Use auto-play mode for long texts.".into());
        }
    }

    Ok(())
}