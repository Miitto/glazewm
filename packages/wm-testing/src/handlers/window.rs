use crate::state::State;

#[derive(Default)]
pub struct WindowEventHandler {}

// TODO: Remove this
#[allow(unused_variables)]
impl wm_platform::WindowEventHandler<crate::handlers::Handler>
  for WindowEventHandler
{
  fn on_window_create(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Created");
  }

  fn on_window_destroy(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Destroyed");
  }

  fn on_window_hidden(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Hidden");
  }

  fn on_window_shown(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Shown");
  }

  fn on_window_moved(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Moved");
  }

  fn on_window_resized(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Resized");
  }

  fn on_window_minimized(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Minimized");
  }

  fn on_window_minimized_end(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Minimized End");
  }

  fn on_window_maximized(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Maximized");
  }

  fn on_window_maximized_end(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Maximized End");
  }

  fn on_window_fullscreened(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Fullscreened");
  }

  fn on_window_fullscreened_end(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
  }

  fn on_window_focused(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Focused");
  }

  fn on_window_title_changed(
    &mut self,
    data: &mut State,
    window: &wm_platform::NativeWindow,
  ) {
    tracing::info!("Window Title Changed");
  }
}
