pub use crate::raw::windows::key::WinKey as RawKey;

impl super::IsKeyDownRaw for RawKey {
  fn is_down_raw(&self) -> bool {
    unsafe { (GetKeyState(*self as u16) & 0x80) == 0x80 }
  }
}

impl super::IsKeyDownRaw for u16 {
  fn is_down_raw(&self) -> bool {
    unsafe { (GetKeyState(*self) & 0x80) == 0x80 }
  }
}

pub use crate::raw::windows::keyboard_hook::Hook;
