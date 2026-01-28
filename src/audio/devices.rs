/// Represents an available audio input device.
///
/// Contains the device index and human-readable name for audio input selection.
#[derive(Debug, Clone)]
pub struct AudioInputDevice {
  index: String,
  name: String,
}

impl AudioInputDevice {
  /// Creates a new AudioInputDevice instance.
  ///
  /// # Arguments
  ///
  /// * `index` - The device index identifier
  /// * `name` - The human-readable device name
  ///
  /// # Returns
  ///
  /// A new `AudioInputDevice` instance.
  pub fn new(index: String, name: String) -> Self {
    return AudioInputDevice { index, name };
  }

  /// Gets the device index.
  ///
  /// Returns the index identifier used by the audio system.
  ///
  /// # Returns
  ///
  /// A reference to the device index string.
  pub fn get_index(&self) -> &String {
    return &self.index;
  }
  /// Gets the device name.
  ///
  /// Returns the human-readable name of the audio device.
  ///
  /// # Returns
  ///
  /// A reference to the device name string.
  pub fn get_name(&self) -> &String {
    return &self.name;
  }
}

impl Default for AudioInputDevice {
  fn default() -> Self {
    return AudioInputDevice::new(
      String::from("default"),
      String::from("default"),
    );
  }
}

/// Type alias for a collection of audio input devices.
pub type AudioInputDevices = Vec<AudioInputDevice>;
