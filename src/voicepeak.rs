use std::process::Command as ProcessCommand;

const VOICEPEAK_PATH: &str = "/Applications/voicepeak.app/Contents/MacOS/voicepeak";

pub fn list_narrator() {
    let output = ProcessCommand::new(VOICEPEAK_PATH)
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

pub fn list_emotion(narrator: &str) {
    let output = ProcessCommand::new(VOICEPEAK_PATH)
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

pub struct VoicepeakCommand {
    command: ProcessCommand,
}

impl VoicepeakCommand {
    pub fn new() -> Self {
        Self {
            command: ProcessCommand::new(VOICEPEAK_PATH),
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.command.arg("-s").arg(text);
        self
    }

    pub fn text_file(mut self, file: &str) -> Self {
        self.command.arg("-t").arg(file);
        self
    }

    pub fn narrator(mut self, narrator: &str) -> Self {
        self.command.arg("-n").arg(narrator);
        self
    }

    pub fn emotion(mut self, emotion: &str) -> Self {
        if !emotion.is_empty() {
            self.command.arg("-e").arg(emotion);
        }
        self
    }

    pub fn output(mut self, path: &std::path::Path) -> Self {
        self.command.arg("-o").arg(path);
        self
    }

    pub fn speed(mut self, speed: &str) -> Self {
        self.command.arg("--speed").arg(speed);
        self
    }

    pub fn pitch(mut self, pitch: &str) -> Self {
        self.command.arg("--pitch").arg(pitch);
        self
    }

    pub fn execute(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let status = self.command.status()?;
        if !status.success() {
            return Err("voicepeak command failed".into());
        }
        Ok(())
    }
}

impl Default for VoicepeakCommand {
    fn default() -> Self {
        Self::new()
    }
}