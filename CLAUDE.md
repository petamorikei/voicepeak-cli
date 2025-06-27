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

The project is structured as both a library and binary crate using Rust 2018 module system (no mod.rs):

- `src/lib.rs` - Library crate entry point, re-exports all modules
- `src/main.rs` - Binary crate entry point, uses the library
- `src/presets.rs` - Voice preset definitions and management
- `src/voicepeak.rs` - VOICEPEAK command execution and wrapper
- `src/audio.rs` - Audio playback and temporary file management  
- `src/cli.rs` - CLI argument parsing and command handling
- `src/env_check.rs` - Environment validation (macOS, VOICEPEAK, mpv)
- `src/text_splitter.rs` - Text splitting for 140-character limit handling
- `Cargo.toml` - Project configuration with dependencies: clap, tempfile, serde, serde_json

Key components:
- `VoicePreset` struct combines narrator and emotion settings
- `VoicepeakCommand` provides a builder pattern for VOICEPEAK execution
- Default presets for "夏色花梨" with various emotions (normal, happy, angry, sad, whisper)
- Automatic temporary file creation and cleanup for audio playback
- Integration with system mpv for audio playback
- Environment validation ensures macOS with VOICEPEAK and mpv installed
- Intelligent text splitting for inputs longer than 140 characters with natural break points

## System Requirements

This application requires:
- macOS operating system
- VOICEPEAK installed at `/Applications/voicepeak.app/Contents/MacOS/voicepeak`
- mpv installed and available in PATH (`brew install mpv`)

The application performs environment checks on startup and will display helpful error messages if requirements are not met.

This structure allows the functionality to be used both as a CLI application and as a library in other Rust projects.

## Usage Examples

```bash
# Basic text-to-speech with auto-play
./target/debug/vp -s "Hello world"

# Use preset with emotion
./target/debug/vp -s "こんにちは" -p karin-happy

# Save to file (no auto-play)
./target/debug/vp -s "Hello" -o output.wav

# List available presets
./target/debug/vp --list-presets

# Long text with automatic splitting (default behavior)
./target/debug/vp -s "Very long text..." -p karin-normal

# Reject long text (strict mode)
./target/debug/vp -s "Very long text..." --strict-length -p karin-normal
# Error: Input text is too long (183 characters). Maximum allowed is 140 characters.
```