use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionParam {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicePreset {
    pub name: String,
    pub narrator: String,
    pub emotions: Vec<EmotionParam>,
    pub pitch: Option<i32>,
}

impl VoicePreset {


    pub fn get_emotion_string(&self) -> String {
        if self.emotions.is_empty() {
            String::new()
        } else {
            self.emotions
                .iter()
                .map(|e| format!("{}={}", e.name, e.value))
                .collect::<Vec<_>>()
                .join(",")
        }
    }
}

impl EmotionParam {
    pub fn new(name: &str, value: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub default_preset: Option<String>,
    pub presets: Vec<VoicePreset>,
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    let config_dir = home_dir.join(".config").join("vp");
    
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }
    
    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }
    
    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let content = toml::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}

pub fn get_presets_map(config: &Config) -> HashMap<String, VoicePreset> {
    config.presets.iter()
        .map(|preset| (preset.name.clone(), preset.clone()))
        .collect()
}

pub fn list_presets(config: &Config) {
    println!("Available presets:");
    for preset in &config.presets {
        let marker = if Some(&preset.name) == config.default_preset.as_ref() {
            " (default)"
        } else {
            ""
        };
        let emotion_display = if preset.emotions.is_empty() {
            "normal".to_string()
        } else {
            preset.get_emotion_string()
        };
        let pitch_display = preset.pitch
            .map(|p| format!(", pitch={}", p))
            .unwrap_or_default();
        println!("  {} - {} ({}{}){}", 
                 preset.name, 
                 preset.narrator, 
                 emotion_display,
                 pitch_display,
                 marker);
    }
    
    if let Some(ref default) = config.default_preset {
        println!("\nDefault preset: {}", default);
    } else {
        println!("\nNo default preset set");
    }
}