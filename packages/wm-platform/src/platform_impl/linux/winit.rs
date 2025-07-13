use std::time::Duration;

use smithay::{
  backend::{
    renderer::{
      damage::OutputDamageTracker,
      element::surface::WaylandSurfaceRenderElement, gles::GlesRenderer,
    },
    winit::{self, WinitEvent},
  },
  output::{Mode, Output, PhysicalProperties, Subpixel},
  reexports::calloop::EventLoop,
  utils::{Rectangle, Transform},
};
use thiserror::Error;

use crate::{Data, EventHandler, PlatformData};

#[derive(Error, Debug)]
pub enum WinitError {
  #[error("failed to init winit: {0}")]
  Init(#[from] smithay::backend::winit::Error),
  #[error("failed to add the winit event source: {0}")]
  SourceAdd(
    #[from]
    Box<
      smithay::reexports::calloop::InsertError<
        smithay::backend::winit::WinitEventLoop,
      >,
    >,
  ),
}

/// Creates an output window using `winit` to act as a virtual monitor.
/// Used for testing
pub fn init_winit<D, H>(
  event_loop: &mut EventLoop<Data<D, H>>,
  data: &mut PlatformData<D, H>,
) -> Result<(), WinitError>
where
  H: EventHandler<D>,
{
  let display_handle = &mut data.display_handle;
  let state = &mut data.state;

  let (mut backend, winit) = winit::init()?;

  let mode = Mode {
    size: backend.window_size(),
    refresh: 60_000,
  };

  let output = Output::new(
    "winit".to_string(),
    PhysicalProperties {
      size: (0, 0).into(),
      subpixel: Subpixel::Unknown,
      make: "Smithay".into(),
      model: "Winit".into(),
    },
  );
  tracing::info!("Creating output: {:?}", output.name());
  let _global = output.create_global::<Data<D, H>>(display_handle);
  tracing::info!("Output global created");
  output.change_current_state(
    Some(mode),
    Some(Transform::Flipped180),
    None,
    Some((0, 0).into()),
  );
  output.set_preferred(mode);

  state.space.map_output(&output, (0, 0));

  let mut damage_tracker = OutputDamageTracker::from_output(&output);

  std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);

  event_loop
    .handle()
    .insert_source(winit, move |event, (), data| {
      let display = &mut data.platform.display_handle;

      match event {
        WinitEvent::Resized { size, .. } => {
          output.change_current_state(
            Some(Mode {
              size,
              refresh: 60_000,
            }),
            None,
            None,
            None,
          );
        }
        WinitEvent::Input(event) => data.process_input_event(event),
        WinitEvent::Redraw => {
          let size = backend.window_size();
          let damage = Rectangle::from_size(size);

          {
            let (renderer, mut framebuffer) = backend.bind().unwrap();
            smithay::desktop::space::render_output::<
              _,
              WaylandSurfaceRenderElement<GlesRenderer>,
              _,
              _,
            >(
              &output,
              renderer,
              &mut framebuffer,
              1.0,
              0,
              [&data.platform.state.space],
              &[],
              &mut damage_tracker,
              [0.1, 0.1, 0.1, 1.0],
            )
            .unwrap();
          }
          backend.submit(Some(&[damage])).unwrap();

          data.platform.state.space.elements().for_each(|window| {
            window.send_frame(
              &output,
              data.platform.state.start_time.elapsed(),
              Some(Duration::ZERO),
              |_, _| Some(output.clone()),
            );
          });

          data.platform.state.space.refresh();
          data.platform.state.popups.cleanup();
          let _ = display.flush_clients();

          // Ask for redraw to schedule new frame.
          backend.window().request_redraw();
        }
        WinitEvent::CloseRequested => {
          data.platform.state.loop_signal.stop();
        }
        WinitEvent::Focus(_f) => {}
      }
    })
    .map_err(Box::new)?;

  Ok(())
}
