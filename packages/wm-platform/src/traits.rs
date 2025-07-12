pub trait EventLoopData {
  fn platform_data(&self) -> &crate::PlatformData;
  fn platform_data_mut(&mut self) -> &mut crate::PlatformData;

  fn config(&self) -> &wm_common::ParsedConfig;
}

#[allow(unused_variables)]
pub trait WindowEventHandler<D: EventLoopData> {
  fn on_window_create(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_destroy(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_hidden(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_shown(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_moved(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_resized(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_minimized(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_minimized_end(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_maximized(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_maximized_end(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_fullscreened(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_fullscreened_end(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_focused(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_title_changed(
    &mut self,
    data: &mut D,
    window: &crate::NativeWindow,
  ) {
  }
}
