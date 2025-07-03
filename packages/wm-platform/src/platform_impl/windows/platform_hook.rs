use super::MouseHook;
use crate::{
  platform_impl::{EventLoop, Installable, WindowEventHook},
  WindowEventType,
};

struct PlatformHookInstaller {}

struct PlatformHook {
  event_loop: Option<EventLoop>,
  window_event_hook: Option<WindowEventHook>,
}
impl PlatformHook {
  pub fn dedicated() -> anyhow::Result<Self> {
    Ok(Self {
      event_loop: None,
      window_event_hook: None,
    })
  }

  pub fn remote() -> anyhow::Result<(Self, PlatformHookInstaller)> {
    todo!("Implement remote platform hook installation");
  }

  pub fn create_mouse_listener(&mut self) -> anyhow::Result<MouseHook> {
    if self.event_loop.is_none() {
      self.event_loop = Some(EventLoop::new()?);
    }
    let event_loop = self.event_loop.as_mut().unwrap();
    let (mouse_hook, installer) =
      MouseHook::new(event_loop.message_window_handle())?;

    event_loop.install(installer)?;

    Ok(mouse_hook)
  }

  pub fn with_window_events<S, I>(
    &mut self,
    events: &'static [WindowEventType],
  ) -> anyhow::Result<WindowEventHook> {
    let (window_event_hook, installer) = WindowEventHook::new(events)?;

    if self.event_loop.is_none() {
      self.event_loop = Some(EventLoop::new()?);
    }
    let event_loop = self.event_loop.as_mut().unwrap();
    event_loop.install(installer)?;
    Ok(window_event_hook)
  }
}
