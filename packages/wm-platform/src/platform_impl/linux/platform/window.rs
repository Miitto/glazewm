//! Impls for [`PlatformData`] involving windows

use super::PlatformData;
use crate::EventHandler;

impl<D, H> PlatformData<D, H> where H: EventHandler<D> {}
