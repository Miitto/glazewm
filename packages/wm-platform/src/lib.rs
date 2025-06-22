#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![feature(iterator_try_collect)]
#![feature(once_cell_try)]

mod event_listener;
mod key;
mod keyboard_hook;

mod interfaces;
mod raw;

pub use event_listener::*;
pub use keyboard_hook::*;

pub use interfaces::*;
