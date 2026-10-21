# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Breaking:** Renamed the command from `vp` to `vpsay` to avoid conflicting with Vite+ (`vp`)
- Config moved from `~/.config/vp/config.toml` to `~/.config/vpsay/config.toml`; an existing config at the old path is copied automatically on first run
- Lock file moved to `vpsay/vpsay.lock` in the config directory

## [0.8.1] - 2026-10-18

### Fixed
- Playback cutting off at the beginning and end: mpv's coreaudio output failed with mono audio on some devices and fell back to avfoundation; audio is now upmixed to stereo
- Silence inserted between merged chunks now matches the input WAV format (VOICEPEAK outputs 48kHz), so it is exactly 1 second

### Changed
- Docs use the English narrator name `Natsuki Karin`; Japanese narrator names crash VOICEPEAK 1.2.23
- Retry wait now uses exponential backoff (0.5s, 1s, 2s, 4s, then 5s) instead of a fixed 5 seconds
- Removed `pkill -f voicepeak` before retries, which could kill unrelated processes such as the VOICEPEAK editor; timed-out processes are still killed by PID

## [0.8.0] - 2026-02-17

### Added
- `--bg` flag for background execution using fork() + setsid()
- Shell returns control immediately while audio generates and plays in the background
- All existing flags and options work with `--bg`
- stdout/stderr suppressed in background mode

### Changed
- Added `libc` dependency for process forking

## [0.7.0] - 2025-10-23

### Added
- File locking mechanism to prevent concurrent VOICEPEAK execution
- Exclusive lock using `~/.config/vp/vp.lock` ensures safe parallel command execution
- Multiple `vp` commands can now be run simultaneously without interference

### Fixed
- Parallel execution no longer causes process conflicts and failures
- Eliminates race conditions when multiple instances try to use VOICEPEAK

### Changed
- Added `fs2` dependency for file locking functionality

## [0.6.1] - 2025-08-12

### Changed
- Improved preset help to reference --list-presets

## [0.6.0] - 2025-08-12

### Added
- Robust retry logic (10 attempts with 5-second wait between attempts)
- 15-second timeout for VOICEPEAK command execution
- Automatic cleanup of hung VOICEPEAK processes before retry

## [0.5.0] - 2025-08-10

### Added
- Speed parameter support in presets and CLI
- Enhanced documentation for new options and stdin support
