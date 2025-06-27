use std::path::Path;
use std::process::Command;

pub fn check_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn merge_audio_files(
    input_files: &[&Path],
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if input_files.is_empty() {
        return Err("No input files provided".into());
    }

    if input_files.len() == 1 {
        // Single file, just copy
        std::fs::copy(input_files[0], output_path)?;
        return Ok(());
    }

    // Create concat file for ffmpeg
    let temp_dir = tempfile::tempdir()?;
    let concat_file = temp_dir.path().join("concat_list.txt");
    
    let mut concat_content = String::new();
    for file in input_files {
        concat_content.push_str(&format!("file '{}'\n", file.display()));
    }
    std::fs::write(&concat_file, concat_content)?;

    // Run ffmpeg to concatenate files
    let status = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(&concat_file)
        .arg("-c")
        .arg("copy")
        .arg("-y") // Overwrite output file
        .arg(output_path)
        .status()?;

    if !status.success() {
        return Err("ffmpeg failed to merge audio files".into());
    }

    Ok(())
}