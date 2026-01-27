#[derive(Debug, Clone)]
pub struct AudioInputDevice {
  index: String,
  name: String,
}

impl AudioInputDevice {
  pub fn new(index: String, name: String) -> Self {
    return AudioInputDevice { index, name };
  }

  pub fn get_index(&self) -> &String {
    return &self.index;
  }
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

pub type AudioInputDevices = Vec<AudioInputDevice>;
