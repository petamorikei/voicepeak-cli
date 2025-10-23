# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
