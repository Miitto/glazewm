use smithay::{
  input::pointer::{
    AxisFrame, ButtonEvent, GestureHoldBeginEvent, GestureHoldEndEvent,
    GesturePinchBeginEvent, GesturePinchEndEvent, GesturePinchUpdateEvent,
    GestureSwipeBeginEvent, GestureSwipeEndEvent, GestureSwipeUpdateEvent,
    GrabStartData as PointerGrabStartData, MotionEvent, PointerGrab,
    PointerInnerHandle, RelativeMotionEvent,
  },
  reexports::wayland_server::protocol::wl_surface::WlSurface,
  utils::{Logical, Point},
};

use crate::{Data, EventHandler, NativeWindow};

pub struct MoveSurfaceGrab<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  pub start_data: PointerGrabStartData<Data<D, H>>,
  pub window: NativeWindow,
  pub initial_window_location: Point<i32, Logical>,
}

impl<D, H> PointerGrab<Data<D, H>> for MoveSurfaceGrab<D, H>
where
  H: EventHandler<D>,
{
  fn motion(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    _focus: Option<(WlSurface, Point<f64, Logical>)>,
    event: &MotionEvent,
  ) {
    // While the grab is active, no client has pointer focus
    handle.motion(data, None, event);

    let state = &mut data.platform.state;

    let delta = event.location - self.start_data.location;
    let new_location = self.initial_window_location.to_f64() + delta;
    state.space.map_element(
      self.window.clone(),
      new_location.to_i32_round(),
      true,
    );
  }

  fn relative_motion(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    focus: Option<(WlSurface, Point<f64, Logical>)>,
    event: &RelativeMotionEvent,
  ) {
    handle.relative_motion(data, focus, event);
  }

  fn button(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &ButtonEvent,
  ) {
    // The button is a button code as defined in the
    // Linux kernel's linux/input-event-codes.h header file, e.g. BTN_LEFT.
    const BTN_LEFT: u32 = 0x110;

    handle.button(data, event);

    if !handle.current_pressed().contains(&BTN_LEFT) {
      // No more buttons are pressed, release the grab.
      handle.unset_grab(self, data, event.serial, event.time, true);
    }
  }

  fn axis(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    details: AxisFrame,
  ) {
    handle.axis(data, details);
  }

  fn frame(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
  ) {
    handle.frame(data);
  }

  fn gesture_swipe_begin(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GestureSwipeBeginEvent,
  ) {
    handle.gesture_swipe_begin(data, event);
  }

  fn gesture_swipe_update(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GestureSwipeUpdateEvent,
  ) {
    handle.gesture_swipe_update(data, event);
  }

  fn gesture_swipe_end(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GestureSwipeEndEvent,
  ) {
    handle.gesture_swipe_end(data, event);
  }

  fn gesture_pinch_begin(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GesturePinchBeginEvent,
  ) {
    handle.gesture_pinch_begin(data, event);
  }

  fn gesture_pinch_update(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GesturePinchUpdateEvent,
  ) {
    handle.gesture_pinch_update(data, event);
  }

  fn gesture_pinch_end(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GesturePinchEndEvent,
  ) {
    handle.gesture_pinch_end(data, event);
  }

  fn gesture_hold_begin(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GestureHoldBeginEvent,
  ) {
    handle.gesture_hold_begin(data, event);
  }

  fn gesture_hold_end(
    &mut self,
    data: &mut Data<D, H>,
    handle: &mut PointerInnerHandle<'_, Data<D, H>>,
    event: &GestureHoldEndEvent,
  ) {
    handle.gesture_hold_end(data, event);
  }

  fn start_data(&self) -> &PointerGrabStartData<Data<D, H>> {
    &self.start_data
  }

  fn unset(&mut self, _data: &mut Data<D, H>) {}
}
