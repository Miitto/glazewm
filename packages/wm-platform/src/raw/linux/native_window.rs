#[derive(Debug, Clone, PartialEq)]
pub struct NativeWindow;

impl crate::interfaces::traits::CommonNativeWindow for NativeWindow {
  fn new(handle: crate::WindowHandle) -> Self {
    todo!()
  }

  fn handle(&self) -> crate::WindowHandle {
    todo!()
  }

  fn title(&self) -> anyhow::Result<String> {
    todo!()
  }

  fn refresh_title(&self) -> anyhow::Result<String> {
    todo!()
  }

  fn updated_title(&self) -> anyhow::Result<String> {
    todo!()
  }

  fn process_name(&self) -> anyhow::Result<String> {
    todo!()
  }

  fn updated_process_name(&self) -> anyhow::Result<String> {
    todo!()
  }

  fn class_name(&self) -> anyhow::Result<String> {
    todo!()
  }

  fn updated_class_name(&self) -> anyhow::Result<String> {
    todo!()
  }

  fn is_visible(&self) -> anyhow::Result<bool> {
    todo!()
  }

  fn is_manageable(&self) -> anyhow::Result<bool> {
    todo!()
  }

  fn is_minimized(&self) -> anyhow::Result<bool> {
    todo!()
  }

  fn refresh_is_minimized(&self) -> anyhow::Result<bool> {
    todo!()
  }

  fn is_maximized(&self) -> anyhow::Result<bool> {
    todo!()
  }

  fn refresh_is_maximized(&self) -> anyhow::Result<bool> {
    todo!()
  }

  fn is_resizable(&self) -> bool {
    todo!()
  }

  fn is_popup(&self) -> bool {
    todo!()
  }

  fn is_fullscreen(
    &self,
    monitor_rect: &wm_common::Rect,
  ) -> anyhow::Result<bool> {
    todo!()
  }

  fn set_foreground(&self) -> anyhow::Result<()> {
    todo!()
  }

  fn set_border_color(
    &self,
    color: Option<&wm_common::Color>,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn set_corner_style(
    &self,
    corner_style: &wm_common::CornerStyle,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn set_title_bar_visibility(&self, visible: bool) -> anyhow::Result<()> {
    todo!()
  }

  fn adjust_transparency(
    &self,
    transparency: &wm_common::Delta<wm_common::OpacityValue>,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn set_transparency(
    &self,
    transparency: &wm_common::OpacityValue,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn frame_position(&self) -> anyhow::Result<wm_common::Rect> {
    todo!()
  }

  fn refresh_frame_position(&self) -> anyhow::Result<wm_common::Rect> {
    todo!()
  }

  fn border_position(&self) -> anyhow::Result<wm_common::Rect> {
    todo!()
  }

  fn refresh_border_position(&self) -> anyhow::Result<wm_common::Rect> {
    todo!()
  }

  fn shadow_border_delta(&self) -> anyhow::Result<wm_common::RectDelta> {
    todo!()
  }

  fn restore_to_position(
    &self,
    rect: &wm_common::Rect,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn maximize(&self) -> anyhow::Result<()> {
    todo!()
  }

  fn minimize(&self) -> anyhow::Result<()> {
    todo!()
  }

  fn close(&self) -> anyhow::Result<()> {
    todo!()
  }

  fn set_visible(
    &self,
    visible: bool,
    hide_method: &wm_common::HideMethod,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn show(&self) -> anyhow::Result<()> {
    todo!()
  }

  fn hide(&self) -> anyhow::Result<()> {
    todo!()
  }

  fn set_cloaked(&self, cloaked: bool) -> anyhow::Result<()> {
    todo!()
  }

  fn set_taskbar_visibility(&self, visible: bool) -> anyhow::Result<()> {
    todo!()
  }

  fn set_position(
    &self,
    state: &wm_common::WindowState,
    rect: &wm_common::Rect,
    z_order: &crate::ZOrder,
    is_visible: bool,
    hide_method: &wm_common::HideMethod,
    has_pending_dpi_adjustment: bool,
  ) -> anyhow::Result<()> {
    todo!()
  }

  fn mark_fullscreen(&self, fullscreen: bool) -> anyhow::Result<()> {
    todo!()
  }

  fn set_z_order(&self, z_order: &crate::ZOrder) -> anyhow::Result<()> {
    todo!()
  }

  fn cleanup(&self) {
    todo!()
  }
}
