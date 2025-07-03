use windows::Win32::UI::WindowsAndMessaging::{
  EVENT_OBJECT_CLOAKED, EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE,
  EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_NAMECHANGE, EVENT_OBJECT_SHOW,
  EVENT_OBJECT_UNCLOAKED, EVENT_SYSTEM_FOREGROUND,
  EVENT_SYSTEM_MINIMIZEEND, EVENT_SYSTEM_MINIMIZESTART,
  EVENT_SYSTEM_MOVESIZEEND, EVENT_SYSTEM_MOVESIZESTART,
};
use wm_common::{KeybindingConfig, Point};

use crate::NativeWindow;

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

#[derive(Debug, Clone)]
pub struct MouseMoveEvent {
  /// Location of mouse with 0,0 being the top-left corner of the primary
  /// monitor.
  pub point: Point,

  /// Whether either left or right-click is currently pressed.
  pub is_mouse_down: bool,
}

pub enum MouseEvent {
  MouseMove(MouseMoveEvent),
}

pub enum WindowEventType {
  WindowDestroyed,
  WindowFocused,
  WindowHidden,
  WindowCloaked,
  WindowLocationChanged,
  WindowMinimized,
  WindowMinimizeEnded,
  WindowMovedOrResizedEnd,
  WindowMovedOrResizedStart,
  WindowShown,
  WindowUncloaked,
  WindowTitleChanged,
}

impl WindowEventType {
  pub fn id(&self) -> u32 {
    match self {
      WindowEventType::WindowDestroyed => EVENT_OBJECT_DESTROY,
      WindowEventType::WindowFocused => EVENT_SYSTEM_FOREGROUND,
      WindowEventType::WindowHidden => EVENT_OBJECT_HIDE,
      WindowEventType::WindowCloaked => EVENT_OBJECT_CLOAKED,
      WindowEventType::WindowLocationChanged => {
        EVENT_OBJECT_LOCATIONCHANGE
      }
      WindowEventType::WindowMinimized => EVENT_SYSTEM_MINIMIZESTART,
      WindowEventType::WindowMinimizeEnded => EVENT_SYSTEM_MINIMIZEEND,
      WindowEventType::WindowMovedOrResizedEnd => EVENT_SYSTEM_MOVESIZEEND,
      WindowEventType::WindowMovedOrResizedStart => {
        EVENT_SYSTEM_MOVESIZESTART
      }
      WindowEventType::WindowShown => EVENT_OBJECT_SHOW,
      WindowEventType::WindowUncloaked => EVENT_OBJECT_UNCLOAKED,
      WindowEventType::WindowTitleChanged => EVENT_OBJECT_NAMECHANGE,
    }
  }
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
