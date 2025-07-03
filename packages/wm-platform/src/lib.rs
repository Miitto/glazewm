#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![feature(iterator_try_collect)]
#![feature(once_cell_try)]

mod events;
mod native_window;
mod platform_impl;

pub use events::*;
pub use native_window::*;
pub(crate) use platform_impl as this;
pub use this::{NativeMonitor, Platform, WindowHandle};
