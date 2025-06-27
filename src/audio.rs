use std::path::Path;
use std::process::Command as ProcessCommand;
use tempfile::NamedTempFile;

pub fn play_audio_and_cleanup(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_temp_audio_file() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().with_extension("wav");
    temp_file.persist(&temp_path)?;
    Ok(temp_path)
}
