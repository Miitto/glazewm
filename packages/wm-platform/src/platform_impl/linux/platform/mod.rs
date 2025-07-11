use smithay::reexports::wayland_server::{
  backend::InitError, Display, DisplayHandle,
};

use crate::{state::Glaze, EventLoopData};

mod mouse;
mod window;

pub struct PlatformData {
  pub state: Glaze,
  pub display_handle: DisplayHandle,
}

impl PlatformData {
  pub fn setup_event_loop<D>(
    event_loop: &mut calloop::EventLoop<D>,
  ) -> Result<Self, InitError>
  where
    D: EventLoopData,
  {
    let display: Display<Glaze> = Display::new()?;
    let handle = display.handle();
    let state = Glaze::new(event_loop, display);

    Ok(PlatformData {
      state,
      display_handle: handle,
    })
  }
}
