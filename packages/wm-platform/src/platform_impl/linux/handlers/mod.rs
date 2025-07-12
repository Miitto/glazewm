mod compositor;
mod xdg_shell;

//
// Wl Seat
use smithay::{
  delegate_data_device, delegate_output, delegate_seat,
  desktop::Space,
  input::{Seat, SeatHandler, SeatState},
  output::Output,
  reexports::wayland_server::{
    protocol::{wl_output, wl_surface::WlSurface},
    Resource,
  },
  utils::{Logical, Rectangle},
  wayland::{
    output::OutputHandler,
    seat::WaylandFocus,
    selection::{
      data_device::{
        set_data_device_focus, ClientDndGrabHandler, DataDeviceHandler,
        DataDeviceState, ServerDndGrabHandler,
      },
      SelectionHandler,
    },
  },
};

fn fullscreen_output_geometry(
  wl_surface: &WlSurface,
  wl_output: Option<&wl_output::WlOutput>,
  space: &mut Space<NativeWindow>,
) -> Option<Rectangle<i32, Logical>> {
  // First test if a specific output has been requested
  // if the requested output is not found ignore the request
  wl_output
    .and_then(Output::from_resource)
    .or_else(|| {
      let w = space.elements().find(|window| {
        window.wl_surface().is_some_and(|s| &*s == wl_surface)
      });
      w.and_then(|w| space.outputs_for_element(w).first().cloned())
    })
    .as_ref()
    .and_then(|o| space.output_geometry(o))
}

use super::NativeWindow;
use crate::state::Glaze;

impl SeatHandler for Glaze {
  type KeyboardFocus = WlSurface;
  type PointerFocus = WlSurface;
  type TouchFocus = WlSurface;

  fn seat_state(&mut self) -> &mut SeatState<Glaze> {
    &mut self.state.seat
  }

  fn cursor_image(
    &mut self,
    _seat: &Seat<Self>,
    _image: smithay::input::pointer::CursorImageStatus,
  ) {
  }

  fn focus_changed(
    &mut self,
    seat: &Seat<Self>,
    focused: Option<&WlSurface>,
  ) {
    let dh = &self.display_handle;
    let client = focused.and_then(|s| dh.get_client(s.id()).ok());
    set_data_device_focus(dh, seat, client);
  }
}

delegate_seat!(Glaze);

//
// Wl Data Device
//

impl SelectionHandler for Glaze {
  type SelectionUserData = ();
}

impl DataDeviceHandler for Glaze {
  fn data_device_state(&self) -> &DataDeviceState {
    &self.state.data_device
  }
}

impl ClientDndGrabHandler for Glaze {}
impl ServerDndGrabHandler for Glaze {}

delegate_data_device!(Glaze);

//
// Wl Output & Xdg Output
//

impl OutputHandler for Glaze {}
delegate_output!(Glaze);
