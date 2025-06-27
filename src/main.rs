use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use tempfile::NamedTempFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VoicePreset {
    name: String,
    narrator: String,
    emotion: String,
}

impl VoicePreset {
    fn new(name: &str, narrator: &str, emotion: &str) -> Self {
        Self {
            name: name.to_string(),
            narrator: narrator.to_string(),
            emotion: emotion.to_string(),
        }
    }
}

fn get_default_presets() -> HashMap<String, VoicePreset> {
    let mut presets = HashMap::new();
    
    presets.insert("karin-normal".to_string(), VoicePreset::new("karin-normal", "夏色花梨", ""));
    presets.insert("karin-happy".to_string(), VoicePreset::new("karin-happy", "夏色花梨", "hightension=50"));
    presets.insert("karin-angry".to_string(), VoicePreset::new("karin-angry", "夏色花梨", "buchigire=50"));
    presets.insert("karin-sad".to_string(), VoicePreset::new("karin-sad", "夏色花梨", "nageki=50"));
    presets.insert("karin-whisper".to_string(), VoicePreset::new("karin-whisper", "夏色花梨", "sasayaki=50"));
    
    presets
}

fn main() {
    let matches = Command::new("voicepeak-cli")
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
                .conflicts_with_all(&["narrator", "emotion"]),
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
        .get_matches();

    let presets = get_default_presets();

    if matches.get_flag("list-narrator") {
        list_narrator();
        return;
    }

    if let Some(narrator) = matches.get_one::<String>("list-emotion") {
        list_emotion(narrator);
        return;
    }

    if matches.get_flag("list-presets") {
        list_presets(&presets);
        return;
    }

    if let Err(e) = run_voicepeak(&matches, &presets) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn list_narrator() {
    let output = ProcessCommand::new("/Applications/voicepeak.app/Contents/MacOS/voicepeak")
        .arg("--list-narrator")
        .output();

    match output {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            eprintln!("Failed to execute voicepeak: {}", e);
        }
    }
}

fn list_emotion(narrator: &str) {
    let output = ProcessCommand::new("/Applications/voicepeak.app/Contents/MacOS/voicepeak")
        .arg("--list-emotion")
        .arg(narrator)
        .output();

    match output {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            eprintln!("Failed to execute voicepeak: {}", e);
        }
    }
}

fn list_presets(presets: &HashMap<String, VoicePreset>) {
    println!("Available presets:");
    for (name, preset) in presets {
        println!("  {} - {} ({})", name, preset.narrator, 
                 if preset.emotion.is_empty() { "normal" } else { &preset.emotion });
    }
}

fn play_audio_and_cleanup(file_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let status = ProcessCommand::new("mpv")
        .arg("--no-video")
        .arg("--really-quiet")
        .arg(file_path)
        .status()?;

    if !status.success() {
        return Err("mpv failed to play audio".into());
    }

    std::fs::remove_file(file_path)?;
    Ok(())
}

fn run_voicepeak(matches: &clap::ArgMatches, presets: &HashMap<String, VoicePreset>) 
    -> Result<(), Box<dyn std::error::Error>> {
    
    let mut cmd = ProcessCommand::new("/Applications/voicepeak.app/Contents/MacOS/voicepeak");

    if let Some(text) = matches.get_one::<String>("say") {
        cmd.arg("-s").arg(text);
    } else if let Some(file) = matches.get_one::<String>("text") {
        cmd.arg("-t").arg(file);
    } else {
        return Err("Either --say or --text must be specified".into());
    }

    let (narrator, emotion) = if let Some(preset_name) = matches.get_one::<String>("preset") {
        let preset = presets.get(preset_name)
            .ok_or_else(|| format!("Unknown preset: {}", preset_name))?;
        (preset.narrator.clone(), preset.emotion.clone())
    } else {
        let narrator = matches.get_one::<String>("narrator")
            .map(|s| s.clone())
            .unwrap_or_else(|| "夏色花梨".to_string());
        let emotion = matches.get_one::<String>("emotion")
            .map(|s| s.clone())
            .unwrap_or_default();
        (narrator, emotion)
    };

    cmd.arg("-n").arg(&narrator);
    if !emotion.is_empty() {
        cmd.arg("-e").arg(&emotion);
    }

    if let Some(speed) = matches.get_one::<String>("speed") {
        cmd.arg("--speed").arg(speed);
    }

    if let Some(pitch) = matches.get_one::<String>("pitch") {
        cmd.arg("--pitch").arg(pitch);
    }

    let should_play = matches.get_one::<String>("out").is_none();
    let output_path = if should_play {
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().with_extension("wav");
        temp_file.persist(&temp_path)?;
        temp_path
    } else {
        PathBuf::from(matches.get_one::<String>("out").unwrap())
    };

    cmd.arg("-o").arg(&output_path);

    let status = cmd.status()?;
    if !status.success() {
        return Err("voicepeak command failed".into());
    }

    if should_play {
        play_audio_and_cleanup(&output_path)?;
    }

    Ok(())
}
