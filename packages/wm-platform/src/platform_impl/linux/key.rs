use smithay::input::keyboard::keysyms::*;

impl crate::IsKeyDownRaw for LinuxKey {
  fn is_down_raw(&self) -> bool {
    false
  }
}

impl crate::IsKeyDownRaw for u16 {
  fn is_down_raw(&self) -> bool {
    false
  }
}

#[derive(
  wm_macros::TryToDiscriminant, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(u32)]
pub enum LinuxKey {
  A = KEY_A,

  #[other]
  Other(u32),
}

pub type NativeKey = LinuxKey;
pub type NativeKeyCode = u32;
