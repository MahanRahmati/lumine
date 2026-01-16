# Lumine

## Configuration

Lumine is configured using a toml configuration file.

The configuration file is loaded from XDG configuration directory.

`$XDG_CONFIG_HOME/lumine/config.toml`

```toml
[whisper]
url = "http://127.0.0.1:9090"

[ffmpeg]
recordings_directory = "recordings"
silence_limit = 2
silence_detect_noise = 40
preferred_audio_input_device = ""

[general]
remove_after_transcript = true
verbose = true
```
