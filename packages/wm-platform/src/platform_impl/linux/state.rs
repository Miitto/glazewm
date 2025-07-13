use std::{ffi::OsString, sync::Arc};

use smithay::{
  desktop::{PopupManager, Space, WindowSurfaceType},
  input::{keyboard::XkbConfig, Seat, SeatState},
  reexports::{
    calloop::{
      generic::Generic, EventLoop, Interest, LoopSignal, Mode, PostAction,
    },
    wayland_server::{
      backend::{ClientData, ClientId, DisconnectReason},
      protocol::wl_surface::WlSurface,
      Display, DisplayHandle,
    },
  },
  utils::{Clock, Logical, Monotonic, Point},
  wayland::{
    compositor::{CompositorClientState, CompositorState},
    output::OutputManagerState,
    selection::data_device::DataDeviceState,
    shell::xdg::XdgShellState,
    shm::ShmState,
    socket::ListeningSocketSource,
  },
};

use super::{windows::Windows, NativeWindow};
use crate::{Data, EventHandler};

pub struct Glaze<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  pub start_time: std::time::Instant,
  pub clock: Clock<Monotonic>,

  pub socket_name: OsString,
  pub display_handle: DisplayHandle,

  pub space: Space<NativeWindow>,
  pub loop_signal: LoopSignal,

  pub state: State<D, H>,

  pub popups: PopupManager,

  pub seat: Seat<Data<D, H>>,

  pub windows: Windows,
  pub input: crate::input::InputData,
}

pub struct State<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  pub compositor: CompositorState,
  pub xdg_shell: XdgShellState,
  pub shm: ShmState,
  pub output_manager: OutputManagerState,
  pub seat: SeatState<Data<D, H>>,
  pub data_device: DataDeviceState,
}

impl<D, H> Glaze<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  pub fn new(
    event_loop: &mut EventLoop<Data<D, H>>,
    display: Display<Data<D, H>>,
  ) -> Self {
    let start_time = std::time::Instant::now();

    let dh = display.handle();

    // Compositor State
    let compositor_state = CompositorState::new::<Data<D, H>>(&dh);
    // State for desktop windows, and their popups
    let xdg_shell_state = XdgShellState::new::<Data<D, H>>(&dh);
    // Shared memory for the compositor and wayland clients
    let shm_state = ShmState::new::<Data<D, H>>(&dh, vec![]);
    // An output is an area of space that the compositor uses, such as a
    // monitor. This uses the xdg-output extension
    let output_manager_state =
      OutputManagerState::new_with_xdg_output::<Data<D, H>>(&dh);
    let seat_state = SeatState::new();
    // Copy-Paste and drag operations
    let data_device_state = DataDeviceState::new::<Data<D, H>>(&dh);

    let mut state = State {
      compositor: compositor_state,
      xdg_shell: xdg_shell_state,
      shm: shm_state,
      output_manager: output_manager_state,
      seat: seat_state,
      data_device: data_device_state,
    };

    let popups = PopupManager::default();

    // A seat is a group of keyboards, pointer and touch devices.
    // A seat typically has a pointer and maintains a keyboard focus and a
    // pointer focus.
    let mut seat: Seat<Data<D, H>> = state.seat.new_wl_seat(&dh, "winit");

    // Notify clients that we have a keyboard, for the sake of the example
    // we assume that keyboard is always present. You may want to track
    // keyboard hot-plug in real compositor.
    seat.add_keyboard(XkbConfig::default(), 200, 25).unwrap();

    // Notify clients that we have a pointer (mouse)
    // Here we assume that there is always pointer plugged in
    seat.add_pointer();

    // A space represents a two-dimensional plane. Windows and Outputs can
    // be mapped onto it.
    //
    // Windows get a position and stacking order through mapping.
    // Outputs become views of a part of the Space and can be rendered via
    // Space::render_output.
    let space = Space::default();

    let socket_name = Self::init_wayland_listener(display, event_loop);

    // Get the loop signal, used to stop the event loop
    let loop_signal = event_loop.get_signal();

    let clock = Clock::new();

    Self {
      start_time,
      clock,
      display_handle: dh,

      space,
      loop_signal,
      socket_name,

      state,
      popups,
      seat,
      windows: Windows::default(),
      input: crate::input::InputData::default(),
    }
  }

  /// Connect wayland to the event loop
  fn init_wayland_listener(
    display: Display<Data<D, H>>,
    event_loop: &mut EventLoop<Data<D, H>>,
  ) -> OsString
  where
    H: EventHandler<D> + 'static,
  {
    // Creates a new listening socket, automatically choosing the next
    // available `wayland` socket name.
    let listening_socket = ListeningSocketSource::new_auto().unwrap();

    // Get the name of the listening socket.
    // Clients will connect to this socket.
    let socket_name = listening_socket.socket_name().to_os_string();

    let loop_handle = event_loop.handle();

    // Add the Unix socket to the event loop so we can process events from
    // clients connected to the wayland server
    loop_handle
      .insert_source(listening_socket, move |client_stream, (), state| {
        // Inside the callback, you should insert the client into the
        // display.
        //
        // You may also associate some data with the client when inserting
        // the client.
        state
          .platform
          .display_handle
          .insert_client(client_stream, Arc::new(ClientState::default()))
          .unwrap();
      })
      .expect("Failed to init the wayland event source.");

    // You also need to add the display itself to the event loop, so that
    // client events will be processed by wayland-server.
    loop_handle
      .insert_source(
        Generic::new(display, Interest::READ, Mode::Level),
        |_, display, mut state| {
          // Safety: we don't drop the display
          // Dispatch wayland events to all clients
          unsafe {
            display.get_mut().dispatch_clients(&mut state).unwrap();
          }
          // Tell the event loop to continue
          Ok(PostAction::Continue)
        },
      )
      .unwrap();

    socket_name
  }

  pub fn surface_under(
    &self,
    pos: Point<f64, Logical>,
  ) -> Option<(WlSurface, Point<f64, Logical>)> {
    self
      .space
      .element_under(pos)
      .and_then(|(window, location)| {
        window
          .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
          .map(|(s, p)| (s, (p + location).to_f64()))
      })
  }

  pub fn refresh(&mut self) -> anyhow::Result<()> {
    self.windows.refresh()?;

    Ok(())
  }
}

#[derive(Default)]
pub struct ClientState {
  pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
  fn initialized(&self, _client_id: ClientId) {}
  fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
  }
}
