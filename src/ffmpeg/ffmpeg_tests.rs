use crate::config::*;
use crate::ffmpeg::*;

#[test]
fn test_check_ffmpeg() {
  // TODO: Add the test.
}

#[test]
fn test_ffmpeg_without_ffmpeg_installed() {
  // TODO: Add the test.
}

#[test]
fn test_get_audio_input_devices() {
  // TODO: Add the test.
}

#[test]
fn test_select_audio_input_device() {
  let config = Config::default();
  let ffmpeg = FFMPEG::new(
    config.get_recordings_directory(),
    config.get_silence_limit(),
    config.get_silence_detect_noise(),
    String::from("USB Microphone"),
    config.get_verbose(),
  );

  let devices: AudioInputDevices = vec![
    AudioInputDevice::new(
      String::from("0"),
      String::from("Built-in Microphone"),
    ),
    AudioInputDevice::new(String::from("1"), String::from("USB Microphone")),
    AudioInputDevice::new(String::from("2"), String::from("Headset Mic")),
  ];

  let selected_device = ffmpeg.select_audio_input_device(devices);
  assert_eq!(selected_device.get_name(), "USB Microphone");
  assert_eq!(selected_device.get_index(), "1");
}

#[test]
fn test_select_audio_input_device_with_empty_strings() {
  let config = Config::default();
  let ffmpeg = FFMPEG::new(
    config.get_recordings_directory(),
    config.get_silence_limit(),
    config.get_silence_detect_noise(),
    String::new(),
    config.get_verbose(),
  );

  let devices: AudioInputDevices = vec![
    AudioInputDevice::new(
      String::from("0"),
      String::from("Built-in Microphone"),
    ),
    AudioInputDevice::new(String::from("1"), String::from("USB Microphone")),
  ];

  let selected_device = ffmpeg.select_audio_input_device(devices);
  assert_eq!(selected_device.get_name(), "default");
  assert_eq!(selected_device.get_index(), "default");
}

#[test]
fn test_select_audio_input_device_with_special_characters() {
  let config = Config::default();
  let ffmpeg = FFMPEG::new(
    config.get_recordings_directory(),
    config.get_silence_limit(),
    config.get_silence_detect_noise(),
    String::from("Built-in"),
    config.get_verbose(),
  );

  let devices: AudioInputDevices = vec![
    AudioInputDevice::new(
      String::from("0"),
      String::from("Built-in Microphone"),
    ),
    AudioInputDevice::new(
      String::from("1"),
      String::from("USB Microphone [HD]"),
    ),
  ];

  let selected_device = ffmpeg.select_audio_input_device(devices);
  assert_eq!(selected_device.get_name(), "Built-in Microphone");
  assert_eq!(selected_device.get_index(), "0");
}

#[test]
fn test_select_audio_input_device_with_multiple_matches() {
  let config = Config::default();
  let ffmpeg = FFMPEG::new(
    config.get_recordings_directory(),
    config.get_silence_limit(),
    config.get_silence_detect_noise(),
    String::from("Mic"),
    config.get_verbose(),
  );

  let devices: AudioInputDevices = vec![
    AudioInputDevice::new(
      String::from("0"),
      String::from("Built-in Microphone"),
    ),
    AudioInputDevice::new(String::from("1"), String::from("USB Microphone")),
    AudioInputDevice::new(String::from("2"), String::from("Headset Mic")),
  ];

  let selected_device = ffmpeg.select_audio_input_device(devices);
  assert_eq!(selected_device.get_name(), "Built-in Microphone");
  assert_eq!(selected_device.get_index(), "0");
}

#[test]
fn test_record_audio_with_device() {
  // TODO: Add the test.
}

#[test]
fn test_record_audio_with_device_with_invalid_directory() {
  // TODO: Add the test.
}
