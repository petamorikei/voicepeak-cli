use fs2::FileExt;
use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Output, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

const VOICEPEAK_PATH: &str = "/Applications/voicepeak.app/Contents/MacOS/voicepeak";

fn get_lock_file() -> Result<File, Box<dyn std::error::Error>> {
    let lock_path = get_lock_file_path()?;

    // Ensure parent directory exists
    if let Some(parent) = lock_path.parent() {
        create_dir_all(parent)?;
    }

    // Open or create lock file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&lock_path)?;

    Ok(file)
}

fn get_lock_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("Could not determine config directory")?;
    Ok(config_dir.join("vp").join("vp.lock"))
}

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

#[derive(Debug, Clone)]
struct CommandArgs {
    text: Option<String>,
    narrator: Option<String>,
    emotion: Option<String>,
    output: Option<std::path::PathBuf>,
    speed: Option<String>,
    pitch: Option<String>,
}

pub struct VoicepeakCommand {
    args: CommandArgs,
}

fn execute_command_with_timeout(
    mut command: ProcessCommand,
    timeout_secs: u64,
) -> Result<Output, Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let child_id = Arc::new(Mutex::new(None::<u32>));
    let child_id_clone = Arc::clone(&child_id);

    thread::spawn(move || {
        let child = match command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                let _ = tx.send(Err(e));
                return;
            }
        };

        // Store child process ID for potential killing
        {
            let mut id_guard = child_id_clone.lock().unwrap();
            *id_guard = Some(child.id());
        }

        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(result) => result.map_err(|e| e.into()),
        Err(_) => {
            // Kill the process on timeout
            if let Ok(id_guard) = child_id.lock() {
                if let Some(pid) = *id_guard {
                    kill_process_by_id(pid);
                }
            }
            Err("Command timed out after 15 seconds".into())
        }
    }
}

fn kill_process_by_id(pid: u32) {
    use std::process::Command;
    let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
}

fn kill_all_voicepeak_processes() {
    use std::process::Command;
    let _ = Command::new("pkill").arg("-f").arg("voicepeak").output();
}

impl VoicepeakCommand {
    pub fn new() -> Self {
        Self {
            args: CommandArgs {
                text: None,
                narrator: None,
                emotion: None,
                output: None,
                speed: None,
                pitch: None,
            },
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.args.text = Some(text.to_string());
        self
    }

    pub fn narrator(mut self, narrator: &str) -> Self {
        self.args.narrator = Some(narrator.to_string());
        self
    }

    pub fn emotion(mut self, emotion: &str) -> Self {
        if !emotion.is_empty() {
            self.args.emotion = Some(emotion.to_string());
        }
        self
    }

    pub fn output(mut self, path: &std::path::Path) -> Self {
        self.args.output = Some(path.to_path_buf());
        self
    }

    pub fn speed(mut self, speed: &str) -> Self {
        self.args.speed = Some(speed.to_string());
        self
    }

    pub fn pitch(mut self, pitch: &str) -> Self {
        self.args.pitch = Some(pitch.to_string());
        self
    }

    fn build_command(&self) -> ProcessCommand {
        let mut command = ProcessCommand::new(VOICEPEAK_PATH);

        if let Some(ref text) = self.args.text {
            command.arg("-s").arg(text);
        }
        if let Some(ref narrator) = self.args.narrator {
            command.arg("-n").arg(narrator);
        }
        if let Some(ref emotion) = self.args.emotion {
            command.arg("-e").arg(emotion);
        }
        if let Some(ref output) = self.args.output {
            command.arg("-o").arg(output);
        }
        if let Some(ref speed) = self.args.speed {
            command.arg("--speed").arg(speed);
        }
        if let Some(ref pitch) = self.args.pitch {
            command.arg("--pitch").arg(pitch);
        }

        command
    }

    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        self.execute_with_verbose(false)
    }

    pub fn execute_with_verbose(self, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.execute_with_retry(verbose, 10)
    }

    fn execute_with_retry(
        &self,
        verbose: bool,
        max_retries: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Acquire exclusive lock to prevent concurrent VOICEPEAK execution
        let lock_file = get_lock_file()?;
        lock_file.lock_exclusive()?;
        // Lock will be automatically released when lock_file is dropped

        let mut last_error: Option<Box<dyn std::error::Error>> = None;

        for attempt in 1..=max_retries {
            let command = self.build_command();

            let output = execute_command_with_timeout(command, 15);
            let result = match output {
                Ok(output) => {
                    if verbose {
                        // Print stdout and stderr in verbose mode
                        if !output.stdout.is_empty() {
                            print!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            eprint!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    if output.status.success() {
                        Ok(())
                    } else {
                        Err("voicepeak command failed".into())
                    }
                }
                Err(e) => Err(e),
            };

            match result {
                Ok(()) => return Ok(()),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        eprintln!(
                            "VOICEPEAK command failed (attempt {}/{}), retrying in 5 seconds...",
                            attempt, max_retries
                        );
                        // Kill any remaining VOICEPEAK processes before retry
                        kill_all_voicepeak_processes();
                        thread::sleep(Duration::from_secs(5));
                    }
                }
            }
        }

        Err(format!(
            "VOICEPEAK command failed after {} attempts: {}",
            max_retries,
            last_error.unwrap_or_else(|| "unknown error".into())
        )
        .into())
    }
}

impl Default for VoicepeakCommand {
    fn default() -> Self {
        Self::new()
    }
}
