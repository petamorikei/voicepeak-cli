// This module is deprecated - functionality moved to config.rs
// Keeping for backward compatibility

use crate::config::{get_presets_map, load_config};
use std::collections::HashMap;

pub use crate::config::VoicePreset;

#[deprecated(note = "Use load_config() instead")]
pub fn get_default_presets() -> HashMap<String, VoicePreset> {
    match load_config() {
        Ok(config) => get_presets_map(&config),
        Err(_) => HashMap::new(),
    }
}

#[deprecated(note = "Use config::list_presets() instead")]
pub fn list_presets(presets: &HashMap<String, VoicePreset>) {
    println!("Available presets:");
    for (name, preset) in presets {
        let emotion_display = if preset.emotions.is_empty() {
            "normal".to_string()
        } else {
            preset.get_emotion_string()
        };
        println!("  {} - {} ({})", name, preset.narrator, emotion_display);
    }
}
