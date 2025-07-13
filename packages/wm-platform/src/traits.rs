use crate::PlatformData;

pub trait EventHandler<D>: Sized {
  type WindowEventHandler: WindowEventHandler<D, Self>;
  type MouseEventHandler: MouseEventHandler<D, Self>;
  type DisplayEventHandler: DisplayEventHandler<D, Self>;
  type KeyEventHandler: KeyEventHandler<D, Self>;

  fn window_event_handler(&mut self) -> &mut Self::WindowEventHandler;
  fn mouse_event_handler(&mut self) -> &mut Self::MouseEventHandler;
  fn display_event_handler(&mut self) -> &mut Self::DisplayEventHandler;
  fn key_event_handler(&mut self) -> &mut Self::KeyEventHandler;
}

#[allow(unused_variables)]
pub trait WindowEventHandler<D, H>
where
  H: EventHandler<D>,
{
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

pub trait MouseEventHandler<D, H>
where
  H: EventHandler<D>,
{
}

pub trait DisplayEventHandler<D, H>
where
  H: EventHandler<D>,
{
}

pub trait KeyEventHandler<D, H>
where
  H: EventHandler<D>,
{
  #[allow(unused_variables)]
  fn key_event(
    &mut self,
    data: &mut D,
    platform: &mut PlatformData<D, H>,
    key: crate::KeyData,
  ) -> crate::KeyResponse {
    crate::KeyResponse::DontCare
  }
}
