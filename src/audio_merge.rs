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

struct WavFormat {
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
}

/// Reads the format of a PCM WAV file by scanning its RIFF chunks for `fmt `.
fn read_wav_format(path: &Path) -> Result<WavFormat, Box<dyn std::error::Error>> {
    let data = std::fs::read(path)?;
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return Err(format!("Not a WAV file: {}", path.display()).into());
    }

    let mut pos = 12;
    while pos + 8 <= data.len() {
        let id = &data[pos..pos + 4];
        let size = u32::from_le_bytes(data[pos + 4..pos + 8].try_into()?) as usize;
        let body = pos + 8;
        if id == b"fmt " && size >= 16 && body + 16 <= data.len() {
            return Ok(WavFormat {
                channels: u16::from_le_bytes(data[body + 2..body + 4].try_into()?),
                sample_rate: u32::from_le_bytes(data[body + 4..body + 8].try_into()?),
                bits_per_sample: u16::from_le_bytes(data[body + 14..body + 16].try_into()?),
            });
        }
        // Chunks are padded to an even size
        pos = body + size + (size & 1);
    }

    Err(format!("No fmt chunk found in WAV file: {}", path.display()).into())
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

    // Create 1-second silence audio file matching the input format,
    // since stream copy requires all concatenated files to share it
    let format = read_wav_format(input_files[0])?;
    let silence_path = temp_dir.path().join("silence.wav");
    let silence_status = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-i")
        .arg(format!(
            "anullsrc=channel_layout={}c:sample_rate={}",
            format.channels, format.sample_rate
        ))
        .arg("-t")
        .arg("1") // 1 second
        .arg("-c:a")
        .arg(format!("pcm_s{}le", format.bits_per_sample))
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
