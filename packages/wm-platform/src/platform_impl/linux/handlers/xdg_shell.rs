use std::cell::RefCell;

use smithay::{
  delegate_xdg_shell,
  desktop::{
    find_popup_root_surface, get_popup_toplevel_coords, PopupKind,
    PopupManager, Space, Window,
  },
  input::{
    pointer::{Focus, GrabStartData as PointerGrabStartData},
    Seat,
  },
  output::Output,
  reexports::{
    wayland_protocols::xdg::shell::server::xdg_toplevel,
    wayland_server::{
      protocol::{wl_seat, wl_surface::WlSurface},
      Resource,
    },
  },
  utils::{IsAlive, Rectangle, Serial},
  wayland::{
    compositor::with_states,
    seat::WaylandFocus,
    shell::xdg::{
      PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler,
      XdgShellState, XdgToplevelSurfaceData,
    },
  },
};

use super::fullscreen_output_geometry;
use crate::{
  grabs::{MoveSurfaceGrab, ResizeSurfaceGrab},
  state::Glaze,
  Data, EventHandler, NativeWindow, WindowEventHandler,
};

#[derive(Default)]
pub struct FullscreenSurface(RefCell<Option<NativeWindow>>);

impl FullscreenSurface {
  pub fn set(&self, window: NativeWindow) {
    *self.0.borrow_mut() = Some(window);
  }

  pub fn get(&self) -> Option<NativeWindow> {
    let mut window = self.0.borrow_mut();
    if window.as_ref().is_some_and(|w| !w.alive()) {
      *window = None;
    }
    window.clone()
  }

  pub fn clear(&self) -> Option<NativeWindow> {
    self.0.borrow_mut().take()
  }
}

impl<H> XdgShellHandler for Data<H>
where
  H: EventHandler + 'static,
{
  fn xdg_shell_state(&mut self) -> &mut XdgShellState {
    &mut self.platform.state.state.xdg_shell
  }

  fn new_client(
    &mut self,
    _client: smithay::wayland::shell::xdg::ShellClient,
  ) {
  }

  fn client_pong(
    &mut self,
    _client: smithay::wayland::shell::xdg::ShellClient,
  ) {
  }

  // Called whenever a new window is added to the compositor
  fn new_toplevel(&mut self, surface: ToplevelSurface) {
    let window = Window::new_wayland_window(surface);

    let state = &mut self.platform.state;

    let native_window = NativeWindow::new(window);
    let window = state.windows.new_window(native_window).clone();

    state.space.map_element(window.clone(), (0, 0), false);

    self
      .handler
      .window_event_handler()
      .on_window_create(&mut self.user, &window);
  }

  fn new_popup(
    &mut self,
    surface: PopupSurface,
    _positioner: PositionerState,
  ) {
    let state = &mut self.platform.state;
    state.unconstrain_popup(&surface);
    let _ = state.popups.track_popup(PopupKind::Xdg(surface));
  }

  fn move_request(
    &mut self,
    surface: ToplevelSurface,
    seat: wl_seat::WlSeat,
    serial: Serial,
  ) {
    let state = &mut self.platform.state;

    let seat = Seat::from_resource(&seat).unwrap();

    let wl_surface = surface.wl_surface();

    if let Some(start_data) = check_grab(&seat, wl_surface, serial) {
      let pointer = seat.get_pointer().unwrap();

      let window = state
        .space
        .elements()
        .find(|w| w.toplevel().unwrap().wl_surface() == wl_surface)
        .unwrap()
        .clone();
      let initial_window_location =
        state.space.element_location(&window).unwrap();

      let grab = MoveSurfaceGrab {
        start_data,
        window,
        initial_window_location,
      };

      pointer.set_grab(self, grab, serial, Focus::Clear);
    }
  }

  fn resize_request(
    &mut self,
    surface: ToplevelSurface,
    seat: wl_seat::WlSeat,
    serial: Serial,
    edges: xdg_toplevel::ResizeEdge,
  ) {
    let state = &mut self.platform.state;
    let seat = Seat::from_resource(&seat).unwrap();

    let wl_surface = surface.wl_surface();

    if let Some(start_data) = check_grab(&seat, wl_surface, serial) {
      let pointer = seat.get_pointer().unwrap();

      let window = state
        .space
        .elements()
        .find(|w| w.toplevel().unwrap().wl_surface() == wl_surface)
        .unwrap()
        .clone();
      let initial_window_location =
        state.space.element_location(&window).unwrap();
      let initial_window_size = window.geometry().size;

      surface.with_pending_state(|state| {
        state.states.set(xdg_toplevel::State::Resizing);
      });

      surface.send_pending_configure();

      let grab = ResizeSurfaceGrab::start(
        start_data,
        window,
        edges.into(),
        Rectangle::new(initial_window_location, initial_window_size),
      );

      pointer.set_grab(self, grab, serial, Focus::Clear);
    }
  }

  fn grab(
    &mut self,
    _surface: PopupSurface,
    _seat: wl_seat::WlSeat,
    _serial: Serial,
  ) {
    // TODO popup grabs
  }

  fn maximize_request(&mut self, surface: ToplevelSurface) {
    if surface
      .current_state()
      .capabilities
      .contains(xdg_toplevel::WmCapabilities::Maximize)
    {
      let state = &mut self.platform.state;
      if let Some(window) = state.windows.find_from_surface(&surface) {
        let outputs = state.space.outputs_for_element(window);
        let output = outputs
          .first()
          .or_else(|| state.space.outputs().next())
          .expect("WM has no outputs");
        let geometry = state.space.output_geometry(output).unwrap();

        surface.with_pending_state(|state| {
          state.states.set(xdg_toplevel::State::Maximized);
          state.size = Some(geometry.size);
        });
      }
    }

    if let Some(window) =
      self.platform.state.windows.find_from_surface(&surface)
    {
      self
        .handler
        .window_event_handler()
        .on_window_maximized(&mut self.user, window);
    } else {
      tracing::warn!("Maximize request for a non-existing window");
    }

    if surface.is_initial_configure_sent() {
      surface.send_configure();
    } else {
      // Will be sent on initial configure
    }
  }

  fn unmaximize_request(&mut self, surface: ToplevelSurface) {
    surface.with_pending_state(|state| {
      state.states.unset(xdg_toplevel::State::Maximized);
      state.size = None;
    });

    if let Some(window) =
      self.platform.state.windows.find_from_surface(&surface)
    {
      self
        .handler
        .window_event_handler()
        .on_window_maximized_end(&mut self.user, window);
    } else {
      tracing::warn!("Unmaximize request for a non-existing window");
    }

    surface.send_pending_configure();
  }

  fn fullscreen_request(
    &mut self,
    surface: ToplevelSurface,
    mut wl_output: Option<
      smithay::reexports::wayland_server::protocol::wl_output::WlOutput,
    >,
  ) {
    if surface
      .current_state()
      .capabilities
      .contains(xdg_toplevel::WmCapabilities::Fullscreen)
    {
      let state = &mut self.platform.state;

      // NOTE: This is only one part of the solution. We can set the
      // location and configure size here, but the surface should be
      // rendered fullscreen independently from its buffer size
      let wl_surface = surface.wl_surface();

      let output_geometry = fullscreen_output_geometry(
        wl_surface,
        wl_output.as_ref(),
        &mut state.space,
      );

      if let Some(geometry) = output_geometry {
        let output = wl_output
          .as_ref()
          .and_then(Output::from_resource)
          .unwrap_or_else(|| {
            state.space.outputs().next().unwrap().clone()
          });

        // False positive?
        #[allow(clippy::manual_let_else)]
        let client = if let Ok(client) =
          state.display_handle.get_client(wl_surface.id())
        {
          client
        } else {
          return;
        };

        for output in output.client_outputs(&client) {
          wl_output = Some(output);
        }
        let window = state
          .space
          .elements()
          .find(|window| {
            window.wl_surface().is_some_and(|s| &*s == wl_surface)
          })
          .unwrap();

        surface.with_pending_state(|state| {
          state.states.set(xdg_toplevel::State::Fullscreen);
          state.size = Some(geometry.size);
          state.fullscreen_output = wl_output;
        });
        output
          .user_data()
          .insert_if_missing(FullscreenSurface::default);
        output
          .user_data()
          .get::<FullscreenSurface>()
          .unwrap()
          .set(window.clone());
        tracing::trace!("Fullscreening: {:?}", window);
      }
    }

    // The protocol demands us to always reply with a configure,
    // regardless of we fulfilled the request or not
    if surface.is_initial_configure_sent() {
      surface.send_configure();
    } else {
      // Will be sent during initial configure
    }
  }

  fn unfullscreen_request(&mut self, surface: ToplevelSurface) {
    if !surface
      .current_state()
      .states
      .contains(xdg_toplevel::State::Fullscreen)
    {
      return;
    }

    let ret = surface.with_pending_state(|state| {
      state.states.unset(xdg_toplevel::State::Fullscreen);
      state.size = None;
      state.fullscreen_output.take()
    });
    if let Some(output) = ret {
      let output = Output::from_resource(&output).unwrap();
      if let Some(fullscreen) =
        output.user_data().get::<FullscreenSurface>()
      {
        tracing::trace!("Unfullscreening: {:?}", fullscreen.get());
        fullscreen.clear();

        // From anvil, may be to do with udev?
        // self.backend_data.reset_buffers(&output);
      }
    }

    surface.send_pending_configure();
  }

  fn minimize_request(&mut self, surface: ToplevelSurface) {
    let state = &mut self.platform.state;

    if let Some(window) =
      state.windows.find_from_surface(&surface).cloned()
    {
      state.space.unmap_elem(&window);
      state.windows.window_minimize(&window);
    } else {
      tracing::warn!("Minimize request for a non-existing window");
    }
  }

  fn show_window_menu(
    &mut self,
    _surface: ToplevelSurface,
    _seat: wl_seat::WlSeat,
    _serial: Serial,
    _location: smithay::utils::Point<i32, smithay::utils::Logical>,
  ) {
  }

  fn ack_configure(
    &mut self,
    _surface: smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    _configure: smithay::wayland::shell::xdg::Configure,
  ) {
  }

  fn reposition_request(
    &mut self,
    surface: PopupSurface,
    positioner: PositionerState,
    token: u32,
  ) {
    surface.with_pending_state(|state| {
      let geometry = positioner.get_geometry();
      state.geometry = geometry;
      state.positioner = positioner;
    });
    self.platform.state.unconstrain_popup(&surface);
    surface.send_repositioned(token);
  }

  fn client_destroyed(
    &mut self,
    _client: smithay::wayland::shell::xdg::ShellClient,
  ) {
  }

  /// Called whenever a window is closed
  fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
    let window = self.platform.state.windows.window_close(&surface);

    self
      .handler
      .window_event_handler()
      .on_window_destroy(&mut self.user, &window);
  }

  fn popup_destroyed(&mut self, _surface: PopupSurface) {}

  fn app_id_changed(&mut self, _surface: ToplevelSurface) {}

  fn title_changed(&mut self, _surface: ToplevelSurface) {}

  fn parent_changed(&mut self, _surface: ToplevelSurface) {}
}

delegate_xdg_shell!(@<H: EventHandler + 'static> Data<H>);

fn check_grab<H>(
  seat: &Seat<Data<H>>,
  surface: &WlSurface,
  serial: Serial,
) -> Option<PointerGrabStartData<Data<H>>>
where
  H: EventHandler + 'static,
{
  let pointer = seat.get_pointer()?;

  // Check that this surface has a click grab.
  if !pointer.has_grab(serial) {
    return None;
  }

  let start_data = pointer.grab_start_data()?;

  let (focus, _) = start_data.focus.as_ref()?;
  // If the focus was for a different surface, ignore the request.
  if !focus.id().same_client_as(&surface.id()) {
    return None;
  }

  Some(start_data)
}

/// Should be called on `WlSurface::commit`
pub fn handle_commit(
  popups: &mut PopupManager,
  space: &Space<NativeWindow>,
  surface: &WlSurface,
) {
  // Handle toplevel commits.
  if let Some(window) = space
    .elements()
    .find(|w| w.toplevel().unwrap().wl_surface() == surface)
    .cloned()
  {
    let initial_configure_sent = with_states(surface, |states| {
      states
        .data_map
        .get::<XdgToplevelSurfaceData>()
        .unwrap()
        .lock()
        .unwrap()
        .initial_configure_sent
    });

    if !initial_configure_sent {
      window.toplevel().unwrap().send_configure();
    }
  }

  // Handle popup commits.
  popups.commit(surface);
  if let Some(popup) = popups.find_popup(surface) {
    match popup {
      PopupKind::Xdg(ref xdg) => {
        if !xdg.is_initial_configure_sent() {
          // NOTE: This should never fail as the initial configure is
          // always allowed.
          xdg.send_configure().expect("initial configure failed");
        }
      }
      PopupKind::InputMethod(ref _input_method) => {}
    }
  }
}

impl<H> Glaze<H>
where
  H: EventHandler,
{
  fn unconstrain_popup(&self, popup: &PopupSurface) {
    let Ok(root) = find_popup_root_surface(&PopupKind::Xdg(popup.clone()))
    else {
      return;
    };
    let Some(window) = self
      .space
      .elements()
      .find(|w| w.toplevel().unwrap().wl_surface() == &root)
    else {
      return;
    };

    let output = self.space.outputs().next().unwrap();
    let output_geo = self.space.output_geometry(output).unwrap();
    let window_geo = self.space.element_geometry(window).unwrap();

    // The target geometry for the positioner should be relative to its
    // parent's geometry, so we will compute that here.
    let mut target = output_geo;
    target.loc -=
      get_popup_toplevel_coords(&PopupKind::Xdg(popup.clone()));
    target.loc -= window_geo.loc;

    popup.with_pending_state(|state| {
      state.geometry = state.positioner.get_unconstrained_geometry(target);
    });
  }
}
