pub use u16 as WindowHandle;
pub use u16 as MonitorHandle;

pub use crate::raw::linux::{
  event_source::EventSource, key::LinuxKey as RawKey,
  keyboard_hook::Hook as RawHook, native_monitor::NativeMonitor,
  native_window::NativeWindow, platform::Platform,
  single_instance::SingleInstance,
};

impl super::IsKeyDownRaw for RawKey {
  fn is_down_raw(&self) -> bool {
    // FIXME: Implement the actual key state check for Linux
    false
  }
}

impl super::IsKeyDownRaw for u16 {
  fn is_down_raw(&self) -> bool {
    // FIXME: Implement the actual key state check for Linux
    false
  }
}
