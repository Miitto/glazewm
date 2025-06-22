use anyhow::Result;

use super::NativeWindow;

pub trait IsKeyDownRaw {
  fn is_down_raw(&self) -> bool;
}

pub trait CommonEventSource
where
  Self: Sized,
{
  fn new(
    tx: &tokio::sync::mpsc::UnboundedSender<crate::PlatformEvent>,
    keybinds: &[wm_common::KeybindingConfig],
    focus_follows_cursor: bool,
  ) -> Result<Self>;

  fn update(
    &self,
    keybindings: &[wm_common::KeybindingConfig],
    enable_mouse_events: bool,
  );
}

pub trait CommonSingleInstance
where
  Self: Sized,
{
  fn new() -> Result<Self>;
  fn is_running() -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ZOrder {
  Normal,
  AfterWindow(crate::WindowHandle),
  Top,
  TopMost,
}

pub trait CommonNativeWindow: std::fmt::Debug + Clone + PartialEq {
  fn new(handle: crate::WindowHandle) -> Self;
  fn handle(&self) -> crate::WindowHandle;
  fn title(&self) -> Result<String>;
  fn refresh_title(&self) -> Result<String>;
  fn updated_title(&self) -> Result<String>;
  fn process_name(&self) -> Result<String>;
  fn updated_process_name(&self) -> Result<String>;
  fn class_name(&self) -> Result<String>;
  fn updated_class_name(&self) -> Result<String>;
  fn is_visible(&self) -> Result<bool>;
  fn is_manageable(&self) -> Result<bool>;
  fn is_minimized(&self) -> Result<bool>;
  fn refresh_is_minimized(&self) -> Result<bool>;
  fn is_maximized(&self) -> Result<bool>;
  fn refresh_is_maximized(&self) -> Result<bool>;
  fn is_resizable(&self) -> bool;
  fn is_popup(&self) -> bool;
  fn is_fullscreen(&self, monitor_rect: &wm_common::Rect) -> Result<bool>;
  fn set_foreground(&self) -> Result<()>;
  fn set_border_color(
    &self,
    color: Option<&wm_common::Color>,
  ) -> Result<()>;
  fn set_corner_style(
    &self,
    corner_style: &wm_common::CornerStyle,
  ) -> Result<()>;
  fn set_title_bar_visibility(&self, visible: bool) -> Result<()>;
  fn adjust_transparency(
    &self,
    transparency: &wm_common::Delta<wm_common::OpacityValue>,
  ) -> Result<()>;
  fn set_transparency(
    &self,
    transparency: &wm_common::OpacityValue,
  ) -> Result<()>;
  fn frame_position(&self) -> Result<wm_common::Rect>;
  fn refresh_frame_position(&self) -> Result<wm_common::Rect>;
  fn border_position(&self) -> Result<wm_common::Rect>;
  fn refresh_border_position(&self) -> Result<wm_common::Rect>;
  fn shadow_border_delta(&self) -> Result<wm_common::RectDelta>;
  fn restore_to_position(&self, rect: wm_common::Rect) -> Result<()>;
  fn maximize(&self) -> Result<()>;
  fn minimize(&self) -> Result<()>;
  fn close(&self) -> Result<()>;
  fn set_visible(
    &self,
    visible: bool,
    hide_method: &wm_common::HideMethod,
  ) -> Result<()>;
  fn show(&self) -> Result<()>;
  fn hide(&self) -> Result<()>;
  fn set_cloaked(&self, cloaked: bool) -> Result<()>;
  fn set_taskbar_visibility(&self, visible: bool) -> Result<()>;
  fn set_position(
    &self,
    state: &wm_common::WindowState,
    rect: &wm_common::Rect,
    z_order: &ZOrder,
    is_visible: bool,
    hide_method: &wm_common::HideMethod,
    has_pending_dpi_adjustment: bool,
  ) -> Result<()>;
  fn mark_fullscreen(&self, fullscreen: bool) -> Result<()>;
  fn set_z_order(&self, z_order: &ZOrder) -> Result<()>;
  fn cleanup(&self);
}

pub trait CommonPlatform {
  fn foreground_window() -> crate::NativeWindow;
  fn desktop_window() -> crate::NativeWindow;
  fn sorted_monitors() -> Result<Vec<crate::NativeMonitor>>;
  fn nearest_monitor(window: &crate::NativeWindow)
    -> crate::NativeMonitor;
  fn manageable_windows() -> Result<Vec<crate::NativeWindow>>;
  fn start_event_listener(
    config: &wm_common::ParsedConfig,
  ) -> Result<crate::EventListener>;
  fn new_single_instance() -> Result<crate::SingleInstance>;
  fn root_ancestor(
    window: &crate::NativeWindow,
  ) -> Result<crate::NativeWindow>;
  fn set_cursor_pos(x: i32, y: i32) -> Result<()>;
  fn window_from_point(
    point: &wm_common::Point,
  ) -> Result<crate::NativeWindow>;
  fn mouse_position() -> Result<wm_common::Point>;
  // FIXME: Not sure what to do with this yet.
  fn create_message_window() -> Result<isize>;
  fn run_message_loop();
  fn run_message_cycle() -> Result<()>;
  fn kill_message_loop<T>(
    handle: &std::thread::JoinHandle<T>,
  ) -> Result<()>;
  fn window_animations_enabled() -> Result<bool>;
  fn set_window_animations_enabled(enabled: bool) -> Result<()>;
  fn open_file_explorer(path: &std::path::PathBuf) -> Result<()>;
  fn parse_command(command: &str) -> Result<(String, String)>;
  fn run_command(
    program: &str,
    args: &str,
    hide_window: bool,
  ) -> Result<()>;
  fn show_error_dialog(title: &str, message: &str) -> Result<()>;
}

pub trait CommonNativeMonitor:
  std::fmt::Debug + Clone + PartialEq
{
  fn new(handle: crate::MonitorHandle) -> Self;
  fn handle(&self) -> crate::MonitorHandle;
  fn device_name(&self) -> Result<&String>;
  fn device_path(&self) -> Result<Option<&String>>;
  fn hardware_id(&self) -> Result<Option<&String>>;
  fn rect(&self) -> Result<&wm_common::Rect>;
  fn working_rect(&self) -> Result<&wm_common::Rect>;
  fn dpi(&self) -> Result<u32>;
  fn scale_factor(&self) -> Result<f32>;
}
