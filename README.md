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
lumine transcribe --file samples/jfk.wav
```

## Configuration

Lumine is configured using a toml configuration file.

The configuration file is loaded from XDG configuration directory.

- `$XDG_CONFIG_HOME/lumine/config.toml`

### Default Configuration

```config.toml
[whisper]
url = "http://127.0.0.1:9090"

[ffmpeg]
recordings_directory = ""
silence_limit = 2
silence_detect_noise = 40
preferred_audio_input_device = ""

[general]
remove_after_transcript = true
verbose = false
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues, please file an issue on the GitHub repository.
