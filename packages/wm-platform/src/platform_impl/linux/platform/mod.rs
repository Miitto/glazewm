use smithay::reexports::wayland_server::{Display, DisplayHandle};

use super::winit::WinitError;
use crate::{state::Glaze, Data, EventHandler};

mod mouse;
mod window;

#[derive(thiserror::Error, Debug)]
pub enum InitError {
  #[error("failed to create display: {0}")]
  Display(#[from] smithay::reexports::wayland_server::backend::InitError),
  #[error("failed to create display handle: {0}")]
  Handle(#[from] WinitError),
}

pub struct PlatformData<H>
where
  H: EventHandler + 'static,
{
  pub state: Glaze<H>,
  pub display_handle: DisplayHandle,
}

impl<H> PlatformData<H>
where
  H: EventHandler + 'static,
{
  pub fn new(
    event_loop: &mut calloop::EventLoop<Data<H>>,
  ) -> Result<Self, InitError> {
    let display: Display<Data<H>> = Display::new()?;
    let handle = display.handle();
    let state = Glaze::new(event_loop, display);

    let mut data = PlatformData {
      state,
      display_handle: handle,
    };

    super::winit::init_winit(event_loop, &mut data)?;

    Ok(data)
  }

  pub fn refresh(&mut self) -> anyhow::Result<()> {
    self.state.refresh()?;
    Ok(())
  }
}
