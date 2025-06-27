# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust CLI application called `voicepeak-cli` that serves as a wrapper for the VOICEPEAK text-to-speech software. It provides enhanced functionality including:

- Voice presets combining narrator and emotion settings
- Automatic audio playback with mpv when no output file is specified
- Temporary file management for audio playback
- All original VOICEPEAK CLI options are supported

The application wraps `/Applications/voicepeak.app/Contents/MacOS/voicepeak` and requires mpv for audio playback.

## Development Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run` - Build and run the application
- `cargo check` - Check for compilation errors without building

### Testing and Quality
- `cargo test` - Run tests
- `cargo clippy` - Run the Rust linter
- `cargo fmt` - Format code according to Rust standards

### Release
- `cargo build --release` - Build optimized release version

## Architecture

The project structure:
- `src/main.rs` - Main application with CLI argument parsing, voice presets, and VOICEPEAK integration
- `Cargo.toml` - Project configuration with dependencies: clap, tempfile, serde, serde_json

Key components:
- `VoicePreset` struct combines narrator and emotion settings
- Default presets for "夏色花梨" with various emotions (normal, happy, angry, sad, whisper)
- Automatic temporary file creation and cleanup for audio playback
- Integration with system mpv for audio playback

## Usage Examples

```bash
# Basic text-to-speech with auto-play
./target/debug/voicepeak-cli -s "Hello world"

# Use preset with emotion
./target/debug/voicepeak-cli -s "こんにちは" -p karin-happy

# Save to file (no auto-play)
./target/debug/voicepeak-cli -s "Hello" -o output.wav

# List available presets
./target/debug/voicepeak-cli --list-presets
```