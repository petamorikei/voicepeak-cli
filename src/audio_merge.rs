use std::path::Path;
use std::process::Command;

pub fn check_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .arg("-loglevel")
        .arg("error") // Suppress version output
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

    // Create 1-second silence audio file
    let silence_path = temp_dir.path().join("silence.wav");
    let silence_status = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg("anullsrc=channel_layout=mono:sample_rate=44100")
        .arg("-t")
        .arg("1") // 1 second
        .arg("-y")
        .arg("-loglevel")
        .arg("error")
        .arg(&silence_path)
        .status()?;

    if !silence_status.success() {
        return Err("Failed to create silence audio".into());
    }

    // Create new concat file with silence between audio files
    let mut new_concat_content = String::new();
    for (i, file) in input_files.iter().enumerate() {
        new_concat_content.push_str(&format!("file '{}'\n", file.display()));
        // Add silence between files (but not after the last file)
        if i < input_files.len() - 1 {
            new_concat_content.push_str(&format!("file '{}'\n", silence_path.display()));
        }
    }
    std::fs::write(&concat_file, new_concat_content)?;

    // Run ffmpeg to concatenate files (suppress output)
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
        .arg("-loglevel")
        .arg("error") // Only show errors
        .arg(output_path)
        .status()?;

    if !status.success() {
        return Err("ffmpeg failed to merge audio files".into());
    }

    Ok(())
}