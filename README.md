# voicepeak-cli

**[日本語](README_ja.md) | English**

A command-line interface wrapper for VOICEPEAK text-to-speech software with preset management and automatic audio playback.

## What's Different from the Original VOICEPEAK Command?

This wrapper enhances the original VOICEPEAK CLI with several powerful features:

- 🎵 **Auto-play with mpv** - Automatically plays generated audio when no output file is specified
- 📝 **Voice presets** - Save and reuse combinations of narrator, emotions, and pitch settings
- 📜 **Long text support** - Automatically splits texts longer than 140 characters and merges audio chunks
- 🔧 **Advanced playback modes** - Choose between batch (generate all → merge → play) or sequential (generate → play one by one)
- 🔄 **Pipe input support** - Accept text from stdin: `echo "text" | vpsay`
- 🚀 **Background execution** - Run with `--bg` to return shell control immediately while audio generates and plays
- 🔇 **Clean output** - Suppresses technical output by default (use `--verbose` to see debug info)
- ⚙️ **Configuration file** - Store your preferred settings in `~/.config/vpsay/config.toml`

## Key Benefits

- **Enhanced Workflow**: No need to manually save and play audio files - just run and listen
- **Batch Processing**: Handle long documents without worrying about character limits  
- **Flexible Input**: Works with direct text, files, or piped input from other commands
- **Personalization**: Save your favorite voice configurations for consistent results
- **Professional Output**: Clean interface with optional verbose mode for debugging

## Requirements

- macOS
- [VOICEPEAK](https://www.ai-j.jp/voicepeak/) installed at `/Applications/voicepeak.app/`
- [mpv](https://mpv.io/) for audio playback (install via Homebrew: `brew install mpv`)
- [ffmpeg](https://ffmpeg.org/) for batch mode and multi-chunk file output (install via Homebrew: `brew install ffmpeg`)

## Installation

### From crates.io (Recommended)

```bash
cargo install voicepeak-cli
```

### From source

1. Clone this repository
2. Build and install:
   ```bash
   cargo install --path .
   ```

## Usage

### Basic Usage

```bash
# Simple text-to-speech (requires preset or --narrator)
vpsay "こんにちは、世界！"

# With explicit narrator
vpsay "こんにちは、世界！" --narrator "Natsuki Karin"

# Save to file instead of auto-play
vpsay "こんにちは、世界！" --narrator "Natsuki Karin" -o output.wav

# Read from file
vpsay -t input.txt --narrator "Natsuki Karin"

# Pipe input
echo "こんにちは、世界！" | vpsay --narrator "Natsuki Karin"
cat document.txt | vpsay -p karin-happy
```

### Using Presets

```bash
# List available presets
vpsay --list-presets

# Use a preset
vpsay "こんにちは、世界！" -p karin-happy

# Override preset settings
vpsay "こんにちは、世界！" -p karin-normal --emotion "happy=50"
```

### Voice Controls

```bash
# Control speech parameters
vpsay "こんにちは、世界！" --narrator "Natsuki Karin" --speed 120 --pitch 50

# List available narrators
vpsay --list-narrator

# List emotions for a specific narrator
vpsay --list-emotion "Natsuki Karin"
```

### Text Length Handling

```bash
# Allow automatic text splitting (default)
vpsay "very long text..."

# Strict mode: reject texts longer than 140 characters
vpsay "text" --strict-length
```

### Background Execution

```bash
# Run in background (returns shell control immediately)
vpsay --bg "こんにちは、世界！"

# Combine with other options
vpsay --bg -p karin-happy "こんにちは、世界！"
vpsay --bg -o output.wav "こんにちは、世界！"
echo "こんにちは" | vpsay --bg
```

### Playback Modes

```bash
# Batch mode: generate all chunks first, merge, then play (default)
vpsay "long text" --playback-mode batch

# Sequential mode: generate and play chunks one by one
vpsay "long text" --playback-mode sequential

# Long text file output (uses ffmpeg to merge chunks)
vpsay "very long text" -o output.wav

# For sequential playback without ffmpeg
vpsay "long text" --playback-mode sequential
```

## Configuration

Configuration is stored in `~/.config/vpsay/config.toml`. The file is automatically created on first run.

> **Upgrading from v0.8.x or earlier:** The command was renamed from `vp` to `vpsay` in v0.9.0 to avoid conflicting with Vite+ (`vp`). If `~/.config/vpsay/config.toml` does not exist, your old `~/.config/vp/config.toml` is copied there automatically on first run.

### Example Configuration

```toml
default_preset = "karin-custom"

[[presets]]
name = "karin-custom"
narrator = "Natsuki Karin"
emotions = [
    { name = "hightension", value = 10 },
    { name = "sasayaki", value = 20 },
]
pitch = 30
speed = 120

[[presets]]
name = "karin-normal"
narrator = "Natsuki Karin"
emotions = []

[[presets]]
name = "karin-happy"
narrator = "Natsuki Karin"
emotions = [{ name = "hightension", value = 50 }]
```

### Configuration Fields

- `default_preset`: Optional. Preset to use when no `-p` option is specified
- `presets`: Array of voice presets

#### Preset Fields

- `name`: Unique preset identifier
- `narrator`: Voice narrator name. Use the name shown by `vpsay --list-narrator` (e.g. `Natsuki Karin`). Japanese names such as `夏色花梨` crash VOICEPEAK 1.2.23
- `emotions`: Array of emotion parameters with `name` and `value`
- `pitch`: Optional pitch adjustment (-300 to 300)
- `speed`: Optional speed adjustment (50 to 200)

## Command-Line Options

```
Usage: vpsay [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to say (or pipe from stdin)

Options:
  -t, --text <FILE>              Text file to say
  -o, --out <FILE>               Path of output file (optional - will play with mpv if not specified)
  -n, --narrator <NAME>          Name of voice
  -e, --emotion <EXPR>           Emotion expression (e.g., happy=50,sad=50)
  -p, --preset <NAME>            Use voice preset
      --list-narrator            Print voice list
      --list-emotion <NARRATOR>  Print emotion list for given voice
      --list-presets             Print available presets
      --speed <VALUE>            Speed (50 - 200)
      --pitch <VALUE>            Pitch (-300 - 300)
      --strict-length            Reject input longer than 140 characters (default: false, allows splitting)
      --playback-mode <MODE>     Playback mode: sequential or batch (default: batch)
      --bg                       Run in background (return immediately)
  -v, --verbose                  Enable verbose output (show VOICEPEAK debug messages)
  -h, --help                     Print help
  -V, --version                  Print version
```

## Parameter Priority

When multiple sources specify the same parameter, the priority order is:

1. Command-line options (highest priority)
2. Preset values
3. Default values / none (lowest priority)

For example:
- `vpsay "text" -p my-preset --pitch 100` uses pitch=100 (CLI override)
- `vpsay "text" -p my-preset` uses preset's pitch value
- `vpsay "text" --narrator "voice"` uses no pitch adjustment

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to contribute to this project.
