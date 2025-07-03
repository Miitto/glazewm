use super::MouseHook;
use crate::{
  platform_impl::{EventLoop, WindowEventHook},
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
    let event_loop = self.event_loop.as_ref().unwrap();
    let (mouse_hook, installer) =
      MouseHook::new(event_loop.message_window_handle())?;

    event_loop.dispatch_and_wait(installer)??;

    Ok(mouse_hook)
  }

  pub fn with_window_events(
    &self,
    events: &[WindowEventType],
  ) -> anyhow::Result<WindowEventHook> {
    Ok(event_listener)
  }
}
