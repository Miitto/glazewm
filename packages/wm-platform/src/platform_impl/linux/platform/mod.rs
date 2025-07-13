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

pub struct PlatformData<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  pub state: Glaze<D, H>,
  pub display_handle: DisplayHandle,
}

impl<D, H> PlatformData<D, H>
where
  H: EventHandler<D> + 'static,
{
  pub fn new(
    event_loop: &mut calloop::EventLoop<Data<D, H>>,
  ) -> Result<Self, InitError> {
    let display: Display<Data<D, H>> = Display::new()?;
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
