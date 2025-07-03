use std::{
  sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, OnceLock,
  },
  thread::{self, JoinHandle},
  time::SystemTime,
};

use tokio::sync::mpsc;
use tracing::{info, warn};
use windows::Win32::{
  Devices::HumanInterfaceDevice::{
    HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC,
  },
  Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM},
  UI::{
    Input::{
      GetRawInputData, RegisterRawInputDevices, HRAWINPUT, RAWINPUT,
      RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT,
      RIM_TYPEMOUSE,
    },
    WindowsAndMessaging::{
      DefWindowProcW, DestroyWindow, GetCursorPos, DBT_DEVNODES_CHANGED,
      PBT_APMRESUMEAUTOMATIC, PBT_APMRESUMESUSPEND, PBT_APMSUSPEND,
      RI_MOUSE_LEFT_BUTTON_DOWN, RI_MOUSE_LEFT_BUTTON_UP,
      RI_MOUSE_RIGHT_BUTTON_DOWN, RI_MOUSE_RIGHT_BUTTON_UP,
      SPI_ICONVERTICALSPACING, SPI_SETWORKAREA, WM_DEVICECHANGE,
      WM_DISPLAYCHANGE, WM_INPUT, WM_POWERBROADCAST, WM_SETTINGCHANGE,
    },
  },
};
use wm_common::{KeybindingConfig, Point};

use super::{
  KeyboardHook, MouseMoveEvent, Platform, PlatformEvent, WindowEventHook,
  FOREGROUND_INPUT_IDENTIFIER,
};

/// Global instance of sender for platform events.
///
/// For use with window procedure.
static PLATFORM_EVENT_TX: OnceLock<mpsc::UnboundedSender<PlatformEvent>> =
  OnceLock::new();



#[derive(Debug)]
pub struct EventWindow {
  keyboard_hook: Arc<KeyboardHook>,
  window_thread: Option<JoinHandle<anyhow::Result<()>>>,
}

impl EventWindow {
  /// Creates an instance of `EventWindow`. Spawns a hidden window and
  /// emits platform events.
  ///
  /// Uses global state (e.g. `PLATFORM_EVENT_TX`) and should thus only
  /// ever be instantiated once in the application's lifetime.
  pub fn new(
    event_tx: &mpsc::UnboundedSender<PlatformEvent>,
    keybindings: &Vec<KeybindingConfig>,
    enable_mouse_events: bool,
  ) -> anyhow::Result<Self> {
    let keyboard_hook = KeyboardHook::new(keybindings, event_tx.clone())?;
    let window_event_hook = WindowEventHook::new(event_tx.clone())?;
    let keyboard_hook_clone = keyboard_hook.clone();

    // Add the sender for platform events to global state.
    PLATFORM_EVENT_TX.set(event_tx.clone()).map_err(|_| {
      anyhow::anyhow!("Platform event sender already set.")
    })?;

    ENABLE_MOUSE_EVENTS.store(enable_mouse_events, Ordering::Relaxed);

    let window_thread = thread::spawn(move || {
      // Start hooks for listening to platform events.
      keyboard_hook_clone.start()?;
      window_event_hook.start()?;

      // Create a hidden window with a message loop on the current thread.
      let handle =
        Platform::create_message_window(Some(event_window_proc))?;

      let rid = RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_MOUSE,
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: HWND(handle),
      };

      // Register our window to receive mouse events.
      unsafe {
        #[allow(clippy::cast_possible_truncation)]
        RegisterRawInputDevices(
          &[rid],
          std::mem::size_of::<RAWINPUTDEVICE>() as u32,
        )
      }?;

      Platform::run_message_loop();

      // Clean-up on message loop exit.
      unsafe { DestroyWindow(HWND(handle)) }?;
      keyboard_hook_clone.stop()?;
      window_event_hook.stop()?;

      Ok(())
    });

    Ok(Self {
      keyboard_hook,
      window_thread: Some(window_thread),
    })
  }

  pub fn update(
    &mut self,
    keybindings: &Vec<KeybindingConfig>,
    enable_mouse_events: bool,
  ) {
    self.keyboard_hook.update(keybindings);
    ENABLE_MOUSE_EVENTS.store(enable_mouse_events, Ordering::Relaxed);
  }
}
