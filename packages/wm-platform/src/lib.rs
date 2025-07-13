#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![feature(iterator_try_collect)]
#![feature(once_cell_try)]

mod events;
mod key;
mod platform_impl;
mod traits;

pub use calloop;
pub use events::*;
pub use key::*;
pub use platform_impl::*;
pub use traits::*;

pub enum ZOrder {
  Normal,
  AfterWindow(crate::WindowHandle),
  Top,
  TopMost,
}

pub struct Data<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  pub user: D,
  pub platform: PlatformData<D, H>,
  pub handler: H,
}

impl<D, H> Data<D, H>
where
  H: EventHandler<D>,
{
  pub fn new(user: D, platform: PlatformData<D, H>, handler: H) -> Self {
    Self {
      user,
      platform,
      handler,
    }
  }
}
