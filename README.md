# Lumine

## Usage

### Record and Transcribe Audio

By default, Lumine records audio from your input device and transcribes it:

```bash
lumine
```

### Transcribe Existing Audio File

You can also transcribe an existing audio file directly:

```bash
lumine transcribe --file path/to/audio.wav
```

### Record an Audio File

You can also record an audio file directly:

```bash
lumine record
```

### Reset Configuration

You can reset the configuration to default values:

```bash
lumine reset-config
```

## Requirements

### MacOS

- FFmpeg (required for audio recording and format conversion)

### Linux

- FFmpeg (required for audio recording and format conversion)
- PulseAudio (required for audio capture)

## Build from Source

```bash
git clone https://github.com/MahanRahmati/lumine.git
cd lumine
cargo build --release
```

The compiled binary will be available at `target/release/lumine`.

## Configuration

Lumine is configured using a toml configuration file.

The configuration file is loaded from XDG configuration directory.

- `$XDG_CONFIG_HOME/lumine/config.toml`

### Default Configuration

```config.toml
[whisper]
# Use local Whisper model (true) or remote service (false)
use_local = true
# URL for remote Whisper service (used when use_local = false)
url = "http://127.0.0.1:9090"
# Path to local Whisper model file (used when use_local = true)
model_path = ""
# Path to VAD model for speech filtering (optional, used when use_local = true)
vad_model_path = ""

[recorder]
# Directory for audio recordings (auto-created if empty)
recordings_directory = ""
# Seconds of silence before stopping recording
silence_limit = 2
# Noise threshold in dB for silence detection
silence_detect_noise = 40
# Preferred audio input device name
preferred_audio_input_device = ""
# Maximum recording duration in seconds (0 = unlimited)
max_recording_duration = 60

[general]
# Remove audio files after successful transcription
remove_after_transcript = true
# Enable verbose output for debugging
verbose = false
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues, please file an [issue](https://github.com/MahanRahmati/lumine/issues) on the GitHub repository.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and updates.
