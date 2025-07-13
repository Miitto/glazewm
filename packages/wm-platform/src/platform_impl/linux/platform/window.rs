//! Impls for [`PlatformData`] involving windows

use super::PlatformData;
use crate::EventHandler;

impl<H> PlatformData<H> where H: EventHandler {}
