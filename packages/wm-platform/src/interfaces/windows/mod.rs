pub use isize as WindowHandle;
pub use isize as MonitorHandle;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState;

pub use crate::raw::windows::{
  event_window::EventWindow as EventSource, key::WinKey as RawKey,
  keyboard_hook::Hook as RawHook, native_monitor::NativeMonitor,
  native_window::NativeWindow, platform::Platform,
  single_instance::SingleInstance,
};

impl super::IsKeyDownRaw for RawKey {
  fn is_down_raw(&self) -> bool {
    unsafe { (GetKeyState(*self as i32) & 0x80) == 0x80 }
  }
}

impl super::IsKeyDownRaw for u16 {
  fn is_down_raw(&self) -> bool {
    unsafe { (GetKeyState(i32::from(*self)) & 0x80) == 0x80 }
  }
}

pub use crate::raw::windows::keyboard_hook::Hook;
