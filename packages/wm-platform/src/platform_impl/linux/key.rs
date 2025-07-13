use smithay::input::keyboard::keysyms as k;

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
  A = k::KEY_A,
  B = k::KEY_B,
  C = k::KEY_C,
  D = k::KEY_D,
  E = k::KEY_E,
  F = k::KEY_F,
  G = k::KEY_G,
  H = k::KEY_H,
  I = k::KEY_I,
  J = k::KEY_J,
  K = k::KEY_K,
  L = k::KEY_L,
  M = k::KEY_M,
  N = k::KEY_N,
  O = k::KEY_O,
  P = k::KEY_P,
  Q = k::KEY_Q,
  R = k::KEY_R,
  S = k::KEY_S,
  T = k::KEY_T,
  U = k::KEY_U,
  V = k::KEY_V,
  W = k::KEY_W,
  X = k::KEY_X,
  Y = k::KEY_Y,
  Z = k::KEY_Z,

  #[other]
  Other(u32) = 0,
}

impl LinuxKey {
  pub fn raw(&self) -> NativeKeyCode {
    // Saftey: The enum is repr(u32) and only used u32 values
    unsafe { *std::ptr::from_ref::<Self>(self).cast::<u32>() }
  }
}

pub type NativeKeyCode = u32;
