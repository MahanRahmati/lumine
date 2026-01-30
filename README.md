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

### macOS
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
use_local = true
url = "http://127.0.0.1:9090"
model_path = ""
vad_model_path = ""

[recorder]
recordings_directory = ""
silence_limit = 2
silence_detect_noise = 40
preferred_audio_input_device = ""
max_recording_duration = 60

[general]
remove_after_transcript = true
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
