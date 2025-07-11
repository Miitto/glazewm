//! Impls for [`PlatformData`] involving the mouse

use smithay::utils::SERIAL_COUNTER;

use super::PlatformData;

impl PlatformData {
  pub fn mouse_position(&self) -> anyhow::Result<wm_common::Point> {
    if let Some(pointer) = self.state.seat.get_pointer() {
      let pos = pointer.current_location();
      Ok(pos.into())
    } else {
      Err(anyhow::anyhow!("No pointer found"))
    }
  }

  pub fn set_cursor_pos(&mut self, x: i32, y: i32) {
    if let Some(pointer) = self.state.seat.get_pointer() {
      let point = smithay::utils::Point::new(f64::from(x), f64::from(y));
      let surface = self.state.surface_under(point);
      #[allow(clippy::cast_possible_truncation)]
      let event = smithay::input::pointer::MotionEvent {
        location: point,
        serial: SERIAL_COUNTER.next_serial(),
        time: self.state.clock.now().as_millis(),
      };
      pointer.motion(&mut self.state, surface, &event);
    }
  }
}
