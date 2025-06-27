use std::path::Path;
use std::process::Command;

const VOICEPEAK_PATH: &str = "/Applications/voicepeak.app/Contents/MacOS/voicepeak";

#[derive(Debug)]
pub enum EnvironmentError {
    NotMacOS,
    VoicepeakNotInstalled,
    MpvNotInstalled,
}

impl std::fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvironmentError::NotMacOS => {
                write!(f, "This application is only supported on macOS")
            }
            EnvironmentError::VoicepeakNotInstalled => {
                write!(f, "VOICEPEAK is not installed. Please install VOICEPEAK from the official website.\nExpected path: {}", VOICEPEAK_PATH)
            }
            EnvironmentError::MpvNotInstalled => {
                write!(
                    f,
                    "mpv is not installed. Please install mpv using Homebrew:\n  brew install mpv"
                )
            }
        }
    }
}

impl std::error::Error for EnvironmentError {}

pub fn check_environment() -> Result<(), EnvironmentError> {
    check_macos()?;
    check_voicepeak_installed()?;
    check_mpv_installed()?;
    Ok(())
}

fn check_macos() -> Result<(), EnvironmentError> {
    if !cfg!(target_os = "macos") {
        return Err(EnvironmentError::NotMacOS);
    }
    Ok(())
}

fn check_voicepeak_installed() -> Result<(), EnvironmentError> {
    if !Path::new(VOICEPEAK_PATH).exists() {
        return Err(EnvironmentError::VoicepeakNotInstalled);
    }
    Ok(())
}

fn check_mpv_installed() -> Result<(), EnvironmentError> {
    match Command::new("mpv").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(EnvironmentError::MpvNotInstalled)
            }
        }
        Err(_) => Err(EnvironmentError::MpvNotInstalled),
    }
}
