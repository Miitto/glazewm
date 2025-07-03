use std::cell::RefCell;

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::warn;
use windows::Win32::{
  Foundation::HWND,
  UI::{
    Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK},
    WindowsAndMessaging::{
      EVENT_OBJECT_CLOAKED, EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE,
      EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_NAMECHANGE,
      EVENT_OBJECT_SHOW, EVENT_OBJECT_UNCLOAKED, EVENT_SYSTEM_FOREGROUND,
      EVENT_SYSTEM_MINIMIZEEND, EVENT_SYSTEM_MINIMIZESTART,
      EVENT_SYSTEM_MOVESIZEEND, EVENT_SYSTEM_MOVESIZESTART, OBJID_WINDOW,
      WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS,
    },
  },
};

use crate::{platform_impl::Installable, WindowEvent, WindowEventType};

thread_local! {
  static WINDOW_EVENTS: RefCell<Option<WindowEventHandles>> = RefCell::new(None);
}

struct WindowEventHandles {
  event_tx: mpsc::UnboundedSender<WindowEvent>,
  hook_handles: Vec<HWINEVENTHOOK>,
}

#[derive(Debug)]
pub struct WindowEventHook {
  event_rx: mpsc::UnboundedReceiver<WindowEvent>,
}

impl WindowEventHook {
  /// Creates an instance of `WindowEventHook`.
  pub fn new(
    event_types: &'static [WindowEventType],
  ) -> anyhow::Result<(
    Self,
    Installable<
      impl FnOnce() -> anyhow::Result<()> + Send + 'static,
      impl FnOnce() -> anyhow::Result<()> + Send + 'static,
    >,
  )> {
    let (tx, rx) = mpsc::unbounded_channel::<WindowEvent>();

    let install = move || {
      let mut ids: Vec<u32> =
        event_types.iter().map(|event| event.id()).collect();
      ids.sort();

      let mut iter = ids.iter();
      let mut ranges = vec![];
      while let Some(id) = iter.next() {
        let mut max = *id;
        while let Some(next) = iter.next() {
          if *next == id + 1 {
            max = *next;
          }
        }

        ranges.push((*id, max));
      }
      let handles: anyhow::Result<Vec<HWINEVENTHOOK>> =
        ranges.iter().try_fold(Vec::new(), |mut handles, range| {
          let hook_handle: anyhow::Result<HWINEVENTHOOK> =
            Self::hook_win_event(range.0, range.1);
          handles.push(hook_handle?);
          Ok(handles)
        });

      let event_handles = WindowEventHandles {
        event_tx: tx,
        hook_handles: handles?,
      };

      WINDOW_EVENTS
        .with(|w| w.replace(Some(event_handles)))
        .ok_or(anyhow::anyhow!(
          "Failed to set thread local window hooks"
        ))?;
      Ok(())
    };

    let stop = move || {
      WINDOW_EVENTS.with(|handles| {
        let handles = if let Some(handles) = handles.replace(None) {
          handles.hook_handles
        } else {
          return;
        };
        for hook_handle in handles {
          unsafe { UnhookWinEvent(hook_handle) };
        }
      });

      Ok(())
    };

    let installer = Installable {
      installer: install,
      stop,
    };
    let win_event_hook = Self { event_rx: rx };

    Ok((win_event_hook, installer))
  }

  /// Creates several window event hooks via `SetWinEventHook`.
  fn hook_win_events() -> Result<Vec<HWINEVENTHOOK>> {
    let event_ranges = [
      (EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_LOCATIONCHANGE),
      (EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE),
      (EVENT_SYSTEM_MINIMIZESTART, EVENT_SYSTEM_MINIMIZEEND),
      (EVENT_SYSTEM_MOVESIZEEND, EVENT_SYSTEM_MOVESIZEEND),
      (EVENT_SYSTEM_MOVESIZESTART, EVENT_SYSTEM_MOVESIZESTART),
      (EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND),
      (EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_NAMECHANGE),
      (EVENT_OBJECT_CLOAKED, EVENT_OBJECT_UNCLOAKED),
    ];

    // Create separate hooks for each event range. This is more performant
    // than creating a single hook for all events and filtering them.
    event_ranges
      .iter()
      .try_fold(Vec::new(), |mut handles, event_range| {
        let hook_handle =
          Self::hook_win_event(event_range.0, event_range.1)?;
        handles.push(hook_handle);
        Ok(handles)
      })
  }

  /// Creates a window hook for the specified event range.
  fn hook_win_event(
    event_min: u32,
    event_max: u32,
  ) -> Result<HWINEVENTHOOK> {
    let hook_handle = unsafe {
      SetWinEventHook(
        event_min,
        event_max,
        None,
        Some(window_event_hook_proc),
        0,
        0,
        WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
      )
    };

    if hook_handle.is_invalid() {
      Err(anyhow::anyhow!("Failed to set window event hook."))
    } else {
      Ok(hook_handle)
    }
  }

  /// Stops the window event hook and unhooks from all window events.
  ///
  /// # Panics
  ///
  /// If the internal mutex is poisoned.
  pub fn stop(&self) -> anyhow::Result<()> {
    Ok(())
  }
}

/// Callback passed to `SetWinEventHook` to handle window events.
///
/// This function is called on selected window events, and forwards them
/// through an MPSC channel for the WM to process.
extern "system" fn window_event_hook_proc(
  _hook: HWINEVENTHOOK,
  event_type: u32,
  handle: HWND,
  id_object: i32,
  id_child: i32,
  _event_thread: u32,
  _event_time: u32,
) {
  let is_window_event =
    id_object == OBJID_WINDOW.0 && id_child == 0 && handle != HWND(0);

  // Check whether the event is associated with a window object instead
  // of a UI control.
  if !is_window_event {
    return;
  }

  let window = crate::NativeWindow::new(handle.0);

  let window_event = match event_type {
    EVENT_OBJECT_DESTROY => WindowEvent::WindowDestroyed(window),
    EVENT_SYSTEM_FOREGROUND => WindowEvent::WindowFocused(window),
    EVENT_OBJECT_HIDE | EVENT_OBJECT_CLOAKED => {
      WindowEvent::WindowHidden(window)
    }
    EVENT_OBJECT_LOCATIONCHANGE => {
      WindowEvent::WindowLocationChanged(window)
    }
    EVENT_SYSTEM_MINIMIZESTART => WindowEvent::WindowMinimized(window),
    EVENT_SYSTEM_MINIMIZEEND => WindowEvent::WindowMinimizeEnded(window),
    EVENT_SYSTEM_MOVESIZEEND => {
      WindowEvent::WindowMovedOrResizedEnd(window)
    }
    EVENT_SYSTEM_MOVESIZESTART => {
      WindowEvent::WindowMovedOrResizedStart(window)
    }
    EVENT_OBJECT_SHOW | EVENT_OBJECT_UNCLOAKED => {
      WindowEvent::WindowShown(window)
    }
    EVENT_OBJECT_NAMECHANGE => WindowEvent::WindowTitleChanged(window),
    _ => return,
  };

  WINDOW_EVENTS.with(|hooks| {
    if let Some(ref hooks) = *hooks.borrow() {
      if let Err(err) = hooks.event_tx.send(window_event) {
        warn!("Failed to send platform event '{}'.", err);
      }
    }
  })
}
