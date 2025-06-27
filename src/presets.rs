use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoicePreset {
    pub name: String,
    pub narrator: String,
    pub emotion: String,
}

impl VoicePreset {
    pub fn new(name: &str, narrator: &str, emotion: &str) -> Self {
        Self {
            name: name.to_string(),
            narrator: narrator.to_string(),
            emotion: emotion.to_string(),
        }
    }
}

pub fn get_default_presets() -> HashMap<String, VoicePreset> {
    let mut presets = HashMap::new();
    
    presets.insert("karin-normal".to_string(), VoicePreset::new("karin-normal", "夏色花梨", ""));
    presets.insert("karin-happy".to_string(), VoicePreset::new("karin-happy", "夏色花梨", "hightension=50"));
    presets.insert("karin-angry".to_string(), VoicePreset::new("karin-angry", "夏色花梨", "buchigire=50"));
    presets.insert("karin-sad".to_string(), VoicePreset::new("karin-sad", "夏色花梨", "nageki=50"));
    presets.insert("karin-whisper".to_string(), VoicePreset::new("karin-whisper", "夏色花梨", "sasayaki=50"));
    
    presets
}

pub fn list_presets(presets: &HashMap<String, VoicePreset>) {
    println!("Available presets:");
    for (name, preset) in presets {
        println!("  {} - {} ({})", name, preset.narrator, 
                 if preset.emotion.is_empty() { "normal" } else { &preset.emotion });
    }
}