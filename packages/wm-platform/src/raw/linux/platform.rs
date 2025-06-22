use crate::CommonPlatform;

pub struct Platform;

impl CommonPlatform for Platform {
  fn foreground_window() -> crate::NativeWindow {
    todo!()
  }

  fn desktop_window() -> crate::NativeWindow {
    todo!()
  }

  fn sorted_monitors() -> anyhow::Result<Vec<crate::NativeMonitor>> {
    todo!()
  }

  fn nearest_monitor(
    window: &crate::NativeWindow,
  ) -> crate::NativeMonitor {
    todo!()
  }

  fn manageable_windows() -> anyhow::Result<Vec<crate::NativeWindow>> {
    todo!()
  }

  fn start_event_listener(
    config: &wm_common::ParsedConfig,
  ) -> anyhow::Result<crate::EventListener> {
    todo!()
  }

  fn new_single_instance() -> anyhow::Result<crate::SingleInstance> {
    todo!()
  }

  fn root_ancestor(
    window: &crate::NativeWindow,
  ) -> anyhow::Result<crate::NativeWindow> {
    todo!()
  }

  fn set_cursor_pos(x: i32, y: i32) -> anyhow::Result<()> {
    todo!()
  }

  fn window_from_point(
    point: &wm_common::Point,
  ) -> anyhow::Result<crate::NativeWindow> {
    todo!()
  }

  fn mouse_position() -> anyhow::Result<wm_common::Point> {
    todo!()
  }

  fn create_message_window() -> anyhow::Result<isize> {
    todo!()
  }

  fn run_message_loop() {
    todo!()
  }

  fn run_message_cycle() -> anyhow::Result<()> {
    todo!()
  }

  fn kill_message_loop<T>(
    handle: &std::thread::JoinHandle<T>,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn window_animations_enabled() -> anyhow::Result<bool> {
    todo!()
  }

  fn set_window_animations_enabled(enabled: bool) -> anyhow::Result<()> {
    todo!()
  }

  fn open_file_explorer(path: &std::path::PathBuf) -> anyhow::Result<()> {
    todo!()
  }

  fn parse_command(command: &str) -> anyhow::Result<(String, String)> {
    todo!()
  }

  fn run_command(
    program: &str,
    args: &str,
    hide_window: bool,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn show_error_dialog(title: &str, message: &str) -> anyhow::Result<()> {
    todo!()
  }
}
