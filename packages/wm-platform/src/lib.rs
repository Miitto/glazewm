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

pub struct Data<H>
where
  H: EventHandler + 'static,
{
  pub user: H::Data,
  pub platform: PlatformData<H>,
  pub handler: H,
}

impl<H> Data<H>
where
  H: EventHandler,
{
  pub fn new(
    user: H::Data,
    platform: PlatformData<H>,
    handler: H,
  ) -> Self {
    Self {
      user,
      platform,
      handler,
    }
  }
}
