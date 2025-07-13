use smithay::{
  backend::input::{
    AbsolutePositionEvent, Axis, AxisSource, ButtonState, Event,
    InputBackend, InputEvent, KeyboardKeyEvent, PointerAxisEvent,
    PointerButtonEvent,
  },
  input::{
    keyboard::FilterResult,
    pointer::{AxisFrame, ButtonEvent, MotionEvent},
  },
  reexports::wayland_server::protocol::wl_surface::WlSurface,
  utils::SERIAL_COUNTER,
};

use super::key::LinuxKey;
use crate::{Data, EventHandler, Key, KeyData, KeyEventHandler};

#[derive(Default)]
pub struct InputData {
  pub pressed_keys: Vec<Key>,
}

impl<D, H> Data<D, H>
where
  H: EventHandler<D>,
{
  fn process_keyboard_event<I: InputBackend>(
    &mut self,
    event: &I::KeyboardKeyEvent,
  ) {
    let serial = SERIAL_COUNTER.next_serial();
    let time = Event::time_msec(event);

    let state = &mut self.platform.state;

    state.seat.get_keyboard().unwrap().input::<(), _>(
      self,
      event.key_code(),
      event.state(),
      serial,
      time,
      |data, _modifiers, key| {
        let key = LinuxKey::from(key.raw_code().raw());

        let key = Key::from_vk(key);

        let key_data = KeyData {
          key,
          pressed_keys: data.platform.state.input.pressed_keys.clone(),
        };

        data.handler.key_event_handler().key_event(
          &mut data.user,
          &mut data.platform,
          key_data,
        );

        FilterResult::Forward
      }, /* TODO: Can intercept
          * keystrokes for the WM here,
          * return
          * [`FilterResult::Intercept`] */
    );
  }

  pub fn process_pointer_motion_absolute<I: InputBackend>(
    &mut self,
    event: &I::PointerMotionAbsoluteEvent,
  ) {
    let state = &mut self.platform.state;
    let output = state.space.outputs().next().unwrap();

    let output_geo = state.space.output_geometry(output).unwrap();

    let pos = event.position_transformed(output_geo.size)
      + output_geo.loc.to_f64();

    let serial = SERIAL_COUNTER.next_serial();

    let pointer = state.seat.get_pointer().unwrap();

    let under = state.surface_under(pos);

    pointer.motion(
      self,
      under,
      &MotionEvent {
        location: pos,
        serial,
        time: event.time_msec(),
      },
    );
    pointer.frame(self);
  }

  pub fn process_input_event<I: InputBackend>(
    &mut self,
    event: InputEvent<I>,
  ) {
    match event {
      InputEvent::Keyboard { event, .. } => {
        self.process_keyboard_event::<I>(&event);
      }
      InputEvent::PointerMotionAbsolute { event, .. } => {
        self.process_pointer_motion_absolute::<I>(&event);
      }
      InputEvent::PointerButton { event, .. } => {
        let (pointer, keyboard) = {
          let state = &mut self.platform.state;
          let pointer = state.seat.get_pointer().unwrap();
          let keyboard = state.seat.get_keyboard().unwrap();
          (pointer, keyboard)
        };

        let serial = SERIAL_COUNTER.next_serial();

        let button = event.button_code();

        let button_state = event.state();

        if ButtonState::Pressed == button_state && !pointer.is_grabbed() {
          if let Some((window, _loc)) = self
            .platform
            .state
            .space
            .element_under(pointer.current_location())
            .map(|(w, l)| (w.clone(), l))
          {
            self.platform.state.space.raise_element(&window, true);
            keyboard.set_focus(
              self,
              Some(window.toplevel().unwrap().wl_surface().clone()),
              serial,
            );
            self.platform.state.space.elements().for_each(|window| {
              window.toplevel().unwrap().send_pending_configure();
            });
          } else {
            self.platform.state.space.elements().for_each(|window| {
              window.set_activated(false);
              window.toplevel().unwrap().send_pending_configure();
            });
            keyboard.set_focus(self, Option::<WlSurface>::None, serial);
          }
        };

        pointer.button(
          self,
          &ButtonEvent {
            button,
            state: button_state,
            serial,
            time: event.time_msec(),
          },
        );
        pointer.frame(self);
      }
      #[allow(clippy::cast_possible_truncation)]
      InputEvent::PointerAxis { event, .. } => {
        let state = &mut self.platform.state;
        let source = event.source();

        let horizontal_amount =
          event.amount(Axis::Horizontal).unwrap_or_else(|| {
            event.amount_v120(Axis::Horizontal).unwrap_or(0.0) * 15.0
              / 120.
          });
        let vertical_amount =
          event.amount(Axis::Vertical).unwrap_or_else(|| {
            event.amount_v120(Axis::Vertical).unwrap_or(0.0) * 15.0 / 120.
          });
        let horizontal_amount_discrete =
          event.amount_v120(Axis::Horizontal);
        let vertical_amount_discrete = event.amount_v120(Axis::Vertical);

        let mut frame = AxisFrame::new(event.time_msec()).source(source);
        if horizontal_amount != 0.0 {
          frame = frame.value(Axis::Horizontal, horizontal_amount);
          if let Some(discrete) = horizontal_amount_discrete {
            frame = frame.v120(Axis::Horizontal, discrete as i32);
          }
        }
        if vertical_amount != 0.0 {
          frame = frame.value(Axis::Vertical, vertical_amount);
          if let Some(discrete) = vertical_amount_discrete {
            frame = frame.v120(Axis::Vertical, discrete as i32);
          }
        }

        if source == AxisSource::Finger {
          if event.amount(Axis::Horizontal) == Some(0.0) {
            frame = frame.stop(Axis::Horizontal);
          }
          if event.amount(Axis::Vertical) == Some(0.0) {
            frame = frame.stop(Axis::Vertical);
          }
        }

        let pointer = state.seat.get_pointer().unwrap();
        pointer.axis(self, frame);
        pointer.frame(self);
      }
      _ => {}
    }
  }
}
