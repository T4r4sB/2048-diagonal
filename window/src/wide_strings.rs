use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

#[derive(Default)]
pub struct WideStringManager {
  memory: Vec<Vec<u16>>,
}

impl WideStringManager {
  pub fn new() -> Self {
    Default::default()
  }

  fn to_vec(str: &str) -> Vec<u16> {
    return OsStr::new(str)
      .encode_wide()
      .chain(Some(0).into_iter())
      .collect();
  }

  pub fn from(&mut self, str: &str) -> *const u16 {
    self.memory.push(Self::to_vec(str));
    return self.memory.last().unwrap().as_ptr();
  }
}