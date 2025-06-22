pub struct EventSource;

impl crate::interfaces::CommonEventSource for EventSource {
  fn new(
    tx: &tokio::sync::mpsc::UnboundedSender<crate::PlatformEvent>,
    keybinds: &[wm_common::KeybindingConfig],
    focus_follows_cursor: bool,
  ) -> anyhow::Result<Self> {
    todo!("Implement Linux event source logic here");
  }

  fn update(
    &self,
    keybindings: &[wm_common::KeybindingConfig],
    enable_mouse_events: bool,
  ) {
    // Implement logic to update the event source with new keybindings and
    // paused state
    todo!("Implement Linux event source update logic here");
  }
}
