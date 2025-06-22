pub struct EventSource;

impl crate::interfaces::CommonEventSource for EventSource {
  fn new(
    tx: &tokio::sync::mpsc::UnboundedSender<crate::PlatformEvent>,
    keybinds: &[wm_common::KeybindingConfig],
    focus_follows_cursor: bool,
  ) -> anyhow::Result<Self> {
    todo!();
  }

  fn update(
    &mut self,
    keybindings: &[wm_common::KeybindingConfig],
    enable_mouse_events: bool,
  ) {
    todo!();
  }

  fn destroy(&mut self) -> anyhow::Result<()> {
    // Implement logic to clean up the event source
    todo!();
  }
}
