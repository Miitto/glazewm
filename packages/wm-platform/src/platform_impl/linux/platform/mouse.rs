//! Impls for [`PlatformData`] involving the mouse

use smithay::utils::SERIAL_COUNTER;

use crate::{Data, EventHandler};

impl<D, H> Data<D, H>
where
  H: EventHandler<D>,
{
  pub fn mouse_position(&self) -> anyhow::Result<wm_common::Point> {
    let state = &self.platform.state;
    if let Some(pointer) = state.seat.get_pointer() {
      let pos = pointer.current_location();
      Ok(pos.into())
    } else {
      Err(anyhow::anyhow!("No pointer found"))
    }
  }

  pub fn set_cursor_pos(&mut self, x: i32, y: i32) {
    if let Some(pointer) = self.platform.state.seat.get_pointer() {
      let point = smithay::utils::Point::new(f64::from(x), f64::from(y));
      let surface = self.platform.state.surface_under(point);
      #[allow(clippy::cast_possible_truncation)]
      let event = smithay::input::pointer::MotionEvent {
        location: point,
        serial: SERIAL_COUNTER.next_serial(),
        time: self.platform.state.clock.now().as_millis(),
      };
      pointer.motion(self, surface, &event);
    }
  }
}
