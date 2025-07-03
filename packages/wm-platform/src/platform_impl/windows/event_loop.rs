use std::{
  cell::RefCell,
  sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self},
    Arc,
  },
  thread::{self, JoinHandle},
};

use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  System::Threading::GetCurrentThreadId,
  UI::WindowsAndMessaging::{
    DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW,
    PostMessageW, PostThreadMessageW, TranslateMessage, MSG,
    PBT_APMRESUMEAUTOMATIC, PBT_APMRESUMESUSPEND, PBT_APMSUSPEND,
    WM_DEVICECHANGE, WM_DISPLAYCHANGE, WM_INPUT, WM_POWERBROADCAST,
    WM_QUIT, WM_SETTINGCHANGE, WM_USER,
  },
};

use crate::platform_impl::{DisplayHook, MouseHook};

// Custom message ID for our dispatch mechanism
const WM_DISPATCH_CALLBACK: u32 = WM_USER + 1;

static ENABLE_MOUSE_EVENTS: AtomicBool = AtomicBool::new(false);
static IS_SYSTEM_SUSPENDED: AtomicBool = AtomicBool::new(false);

thread_local! {
  static CLEANUP_FUNCTIONS: RefCell<Vec<Box<dyn FnOnce() -> anyhow::Result<()>>>> = RefCell::new(Vec::new());
}

// Type alias for the callback function
type EventCallback = Box<dyn FnOnce() + Send + 'static>;

pub struct Installable<I, S>
where
  I: FnOnce() -> anyhow::Result<()> + Send + 'static,
  S: FnOnce() -> anyhow::Result<()> + Send + 'static,
{
  pub installer: I,
  pub stop: S,
}

pub struct EventLoop {
  message_window_handle: crate::this::WindowHandle,
  thread_handle: Option<JoinHandle<anyhow::Result<()>>>,
  thread_id: u32,
  cleanup:
    Vec<Arc<dyn FnOnce() -> anyhow::Result<()> + Send + 'static + Sync>>,
}

impl EventLoop {
  /// Creates a new Win32 EventLoop and starts the message loop in a
  /// separate thread
  pub fn new() -> anyhow::Result<Self> {
    let (sender, receiver) =
      mpsc::channel::<(crate::this::WindowHandle, u32)>();

    let thread_handle = thread::spawn(move || -> anyhow::Result<()> {
      let hwnd =
        super::Platform::create_message_window(Some(Self::window_proc))?;
      let thread_id = unsafe { GetCurrentThreadId() };

      // Send the window handle and thread ID back to the main thread
      sender.send((hwnd, thread_id))?;

      // Run the message loop
      Self::run_message_loop()?;

      let fns = CLEANUP_FUNCTIONS.with(|fns| fns.replace(vec![]));
      for cleanup_fn in fns {
        if let Err(err) = cleanup_fn() {
          eprintln!("Cleanup function failed: {}", err);
        }
      }

      unsafe { DestroyWindow(HWND(hwnd)) }?;

      Ok(())
    });

    // Wait for the window handle and thread ID
    let (hwnd, thread_id) = receiver.recv()?;

    Ok(EventLoop {
      message_window_handle: hwnd,
      thread_handle: Some(thread_handle),
      thread_id,
      cleanup: Vec::new(),
    })
  }

  /// Dispatches a callback to be executed on the Win32 message loop thread
  ///
  /// # Arguments
  /// * `callback` - A closure that will be executed on the message loop
  ///   thread
  pub fn dispatch<F>(&self, callback: F) -> anyhow::Result<()>
  where
    F: FnOnce() + Send + 'static,
  {
    // Box the callback and convert to raw pointer
    let boxed_callback = Box::new(callback);
    let callback_ptr = Box::into_raw(boxed_callback) as isize;

    // Post a message with the callback pointer as lParam
    unsafe {
      if PostMessageW(
        HWND(self.message_window_handle),
        WM_DISPATCH_CALLBACK,
        WPARAM(0),
        LPARAM(callback_ptr),
      )
      .is_ok()
      {
        Ok(())
      } else {
        // If PostMessage fails, we need to clean up the callback
        let _ = Box::from_raw(callback_ptr as *mut EventCallback);
        Err(anyhow::anyhow!("Failed to post message"))
      }
    }
  }

  pub fn install<I, S>(
    &mut self,
    installable: Installable<I, S>,
  ) -> anyhow::Result<()>
  where
    I: FnOnce() -> anyhow::Result<()> + Send + 'static,
    S: FnOnce() -> anyhow::Result<()> + Send + 'static + Sync,
  {
    // Dispatch the installer function to the message loop thread
    self.dispatch_and_wait(move || -> anyhow::Result<()> {
      (installable.installer)()?;

      CLEANUP_FUNCTIONS.with(|fns| {
        fns.borrow_mut().push(Box::new(installable.stop));
      });

      Ok(())
    })??;

    Ok(())
  }

  /// Shuts down the event loop gracefully
  pub fn shutdown(&mut self) -> anyhow::Result<()> {
    unsafe {
      // Post WM_QUIT to terminate the message loop
      if PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0))
        .is_err()
      {
        return Err(anyhow::anyhow!("Failed to post quit message"));
      }
    }

    if let Some(handle) = self.thread_handle.take() {
      // Wait for the thread to finish
      if let Err(err) = handle.join() {
        let msg = match err.downcast_ref::<&'static str>() {
          Some(s) => *s,
          None => match err.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Unknown error",
          },
        };

        Err(anyhow::anyhow!("Event loop thread panicked: {}", msg))
      } else {
        Ok(())
      }
    } else {
      Ok(())
    }
  }

  /// Returns true if the event loop is still running
  pub fn is_running(&self) -> bool {
    if let Some(ref handle) = self.thread_handle {
      !handle.is_finished()
    } else {
      false
    }
  }

  /// Returns the thread ID of the message loop thread
  pub fn thread_id(&self) -> u32 {
    self.thread_id
  }

  /// Returns the window handle of the message loop
  pub fn message_window_handle(&self) -> crate::this::WindowHandle {
    self.message_window_handle
  }

  /// Window procedure for handling messages
  unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
  ) -> LRESULT {
    match msg {
      WM_DISPATCH_CALLBACK => {
        // Extract the callback pointer from lParam
        let callback_ptr = lparam.0 as *mut EventCallback;
        if !callback_ptr.is_null() {
          // Convert back to Box and execute
          let callback = Box::from_raw(callback_ptr);
          callback();
        }
        LRESULT(0)
      }
      WM_POWERBROADCAST => {
        #[allow(clippy::cast_possible_truncation)]
        match wparam.0 as u32 {
          // System is resuming from sleep/hibernation.
          PBT_APMRESUMEAUTOMATIC | PBT_APMRESUMESUSPEND => {
            IS_SYSTEM_SUSPENDED.store(false, Ordering::Relaxed);
          }
          // System is entering sleep/hibernation.
          PBT_APMSUSPEND => {
            IS_SYSTEM_SUSPENDED.store(true, Ordering::Relaxed);
          }
          _ => {}
        }

        LRESULT(0)
      }
      WM_DISPLAYCHANGE | WM_SETTINGCHANGE | WM_DEVICECHANGE => {
        // Ignore display change messages if the system hasn't fully
        // resumed from sleep.
        if !IS_SYSTEM_SUSPENDED.load(Ordering::Relaxed) {
          if let Err(err) = DisplayHook::handle_display_event(msg, wparam)
          {
            tracing::warn!(
              "Failed to handle display change message: {}",
              err
            );
          }
        }

        LRESULT(0)
      }
      WM_INPUT => {
        if let Err(err) = MouseHook::handle_mouse_input(wparam, lparam) {
          tracing::warn!("Failed to handle input message: {}", err);
        }

        LRESULT(0)
      }

      _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
  }

  /// Runs the Win32 message loop
  fn run_message_loop() -> anyhow::Result<()> {
    unsafe {
      let mut msg = MSG::default();

      loop {
        let result = GetMessageW(&mut msg, HWND::default(), 0, 0);

        if result.0 == -1 {
          return Err(anyhow::anyhow!("GetMessage failed"));
        } else if result.0 == WM_QUIT as i32 {
          break;
        }

        TranslateMessage(&msg);
        DispatchMessageW(&msg);
      }
    }

    Ok(())
  }
}

impl Drop for EventLoop {
  fn drop(&mut self) {
    if let Err(err) = self.shutdown() {
      eprintln!("Failed to shut down event loop: {}", err);
    }
  }
}

// Convenience methods for common Win32 operations that need to run on the
// message loop thread
impl EventLoop {
  /// Dispatches a callback and waits for it to complete
  ///
  /// # Warning
  /// This method blocks the calling thread until the callback completes.
  /// Do not call this from the message loop thread itself to avoid
  /// deadlock.
  pub fn dispatch_and_wait<F, R>(&self, callback: F) -> anyhow::Result<R>
  where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
  {
    let (sender, receiver) = std::sync::mpsc::channel();

    self.dispatch(move || {
      let result = callback();
      let _ = sender.send(result);
    })?;

    let res = receiver.recv()?;
    Ok(res)
  }
}
