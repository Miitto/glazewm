use crate::state::State;

mod display;
mod key;
mod mouse;
mod window;

#[derive(Default)]
pub struct Handler {
  win: window::WindowEventHandler,
  mouse: mouse::MouseEventHandler,
  display: display::DisplayEventHandler,
  key: key::KeyEventHandler,
}

impl wm_platform::EventHandler for Handler {
  type Data = State;

  type WindowEventHandler = window::WindowEventHandler;

  type MouseEventHandler = mouse::MouseEventHandler;

  type DisplayEventHandler = display::DisplayEventHandler;

  type KeyEventHandler = key::KeyEventHandler;

  fn window_event_handler(&mut self) -> &mut Self::WindowEventHandler {
    &mut self.win
  }

  fn mouse_event_handler(&mut self) -> &mut Self::MouseEventHandler {
    &mut self.mouse
  }

  fn display_event_handler(&mut self) -> &mut Self::DisplayEventHandler {
    &mut self.display
  }

  fn key_event_handler(&mut self) -> &mut Self::KeyEventHandler {
    &mut self.key
  }
}
