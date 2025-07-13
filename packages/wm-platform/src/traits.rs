use crate::PlatformData;

pub trait EventHandler: Sized {
  type Data;

  type WindowEventHandler: WindowEventHandler<Self>;
  type MouseEventHandler: MouseEventHandler<Self>;
  type DisplayEventHandler: DisplayEventHandler<Self>;
  type KeyEventHandler: KeyEventHandler<Self>;

  fn window_event_handler(&mut self) -> &mut Self::WindowEventHandler;
  fn mouse_event_handler(&mut self) -> &mut Self::MouseEventHandler;
  fn display_event_handler(&mut self) -> &mut Self::DisplayEventHandler;
  fn key_event_handler(&mut self) -> &mut Self::KeyEventHandler;
}

#[allow(unused_variables)]
pub trait WindowEventHandler<H>
where
  H: EventHandler,
{
  fn on_window_create(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_destroy(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_hidden(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_shown(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_moved(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_resized(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_minimized(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_minimized_end(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_maximized(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_maximized_end(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_fullscreened(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_fullscreened_end(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_focused(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }

  fn on_window_title_changed(
    &mut self,
    data: &mut H::Data,
    window: &crate::NativeWindow,
  ) {
  }
}

pub trait MouseEventHandler<H>
where
  H: EventHandler,
{
}

pub trait DisplayEventHandler<H>
where
  H: EventHandler,
{
}

pub trait KeyEventHandler<H>
where
  H: EventHandler,
{
  #[allow(unused_variables)]
  fn key_event(
    &mut self,
    data: &mut H::Data,
    platform: &mut PlatformData<H>,
    key: crate::KeyData,
  ) -> crate::KeyResponse {
    crate::KeyResponse::DontCare
  }
}
