# voicepeak-cli

**[日本語](README_ja.md) | English**

A command-line interface wrapper for VOICEPEAK text-to-speech software with preset management and automatic audio playback.

## What's Different from the Original VOICEPEAK Command?

This wrapper enhances the original VOICEPEAK CLI with several powerful features:

- 🎵 **Auto-play with mpv** - Automatically plays generated audio when no output file is specified
- 📝 **Voice presets** - Save and reuse combinations of narrator, emotions, and pitch settings
- 📜 **Long text support** - Automatically splits texts longer than 140 characters and merges audio chunks
- 🔧 **Advanced playback modes** - Choose between batch (generate all → merge → play) or sequential (generate → play one by one)
- 🔄 **Pipe input support** - Accept text from stdin: `echo "text" | vp`
- 🔇 **Clean output** - Suppresses technical output by default (use `--verbose` to see debug info)
- ⚙️ **Configuration file** - Store your preferred settings in `~/.config/vp/config.toml`

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
vp "こんにちは、世界！"

# With explicit narrator
vp "こんにちは、世界！" --narrator "夏色花梨"

# Save to file instead of auto-play
vp "こんにちは、世界！" --narrator "夏色花梨" -o output.wav

# Read from file
vp -t input.txt --narrator "夏色花梨"

# Pipe input
echo "こんにちは、世界！" | vp --narrator "夏色花梨"
cat document.txt | vp -p karin-happy
```

### Using Presets

```bash
# List available presets
vp --list-presets

# Use a preset
vp "こんにちは、世界！" -p karin-happy

# Override preset settings
vp "こんにちは、世界！" -p karin-normal --emotion "happy=50"
```

### Voice Controls

```bash
# Control speech parameters
vp "こんにちは、世界！" --narrator "夏色花梨" --speed 120 --pitch 50

# List available narrators
vp --list-narrator

# List emotions for a specific narrator
vp --list-emotion "夏色花梨"
```

### Text Length Handling

```bash
# Allow automatic text splitting (default)
vp "very long text..."

# Strict mode: reject texts longer than 140 characters
vp "text" --strict-length
```

### Playback Modes

```bash
# Batch mode: generate all chunks first, merge, then play (default)
vp "long text" --playback-mode batch

# Sequential mode: generate and play chunks one by one
vp "long text" --playback-mode sequential

# Long text file output (uses ffmpeg to merge chunks)
vp "very long text" -o output.wav

# For sequential playback without ffmpeg
vp "long text" --playback-mode sequential
```

## Configuration

Configuration is stored in `~/.config/vp/config.toml`. The file is automatically created on first run.

### Example Configuration

```toml
default_preset = "karin-custom"

[[presets]]
name = "karin-custom"
narrator = "夏色花梨"
emotions = [
    { name = "hightension", value = 10 },
    { name = "sasayaki", value = 20 },
]
pitch = 30

[[presets]]
name = "karin-normal"
narrator = "夏色花梨"
emotions = []

[[presets]]
name = "karin-happy"
narrator = "夏色花梨"
emotions = [{ name = "hightension", value = 50 }]
```

### Configuration Fields

- `default_preset`: Optional. Preset to use when no `-p` option is specified
- `presets`: Array of voice presets

#### Preset Fields

- `name`: Unique preset identifier
- `narrator`: Voice narrator name
- `emotions`: Array of emotion parameters with `name` and `value`
- `pitch`: Optional pitch adjustment (-300 to 300)

## Command-Line Options

```
Usage: vp [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to say

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
  -h, --help                     Print help
  -V, --version                  Print version
```

## Parameter Priority

When multiple sources specify the same parameter, the priority order is:

1. Command-line options (highest priority)
2. Preset values
3. Default values / none (lowest priority)

For example:
- `vp "text" -p my-preset --pitch 100` uses pitch=100 (CLI override)
- `vp "text" -p my-preset` uses preset's pitch value
- `vp "text" --narrator "voice"` uses no pitch adjustment

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on how to contribute to this project.
