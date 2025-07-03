use wm_common::KeybindingConfig;

use crate::{platform_impl::MouseMoveEvent, NativeWindow};

pub enum DisplayEvent {
  DisplaySettingsChanged,
}

pub enum KeyboardEventType {
  KeybindingTriggered,
}

pub enum KeyboardEvent {
  KeybindingTriggered(KeybindingConfig),
}

pub enum MouseEventType {
  MouseMove,
}

pub enum MouseEvent {
  MouseMove(MouseMoveEvent),
}

pub enum WindowEventType {
  WindowDestroyed,
  WindowFocused,
  WindowHidden,
  WindowLocationChanged,
  WindowMinimized,
  WindowMinimizeEnded,
  WindowMovedOrResizedEnd,
  WindowMovedOrResizedStart,
  WindowShown,
  WindowTitleChanged,
}

pub enum WindowEvent {
  WindowDestroyed(NativeWindow),
  WindowFocused(NativeWindow),
  WindowHidden(NativeWindow),
  WindowLocationChanged(NativeWindow),
  WindowMinimized(NativeWindow),
  WindowMinimizeEnded(NativeWindow),
  WindowMovedOrResizedEnd(NativeWindow),
  WindowMovedOrResizedStart(NativeWindow),
  WindowShown(NativeWindow),
  WindowTitleChanged(NativeWindow),
}
