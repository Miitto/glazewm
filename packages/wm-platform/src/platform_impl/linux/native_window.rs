#![allow(unused_variables)]
// TODO: Remove this once the code is complete

use smithay::{
  backend::renderer::element::AsRenderElements,
  desktop::{space::SpaceElement, Window},
  reexports::wayland_protocols::xdg::shell::server::xdg_toplevel,
  utils::IsAlive,
  wayland::seat::WaylandFocus,
};
use wm_common::{
  Color, CornerStyle, HideMethod, OpacityValue, Rect, WindowState,
};

use crate::ZOrder;

#[derive(Debug, Clone, PartialEq)]
pub struct NativeWindow {
  // Smithay doesn't expose a window ID (that I can find), so create our
  // own
  id: uuid::Uuid,
  inner: Window,
  needs_configure: bool,
}

impl NativeWindow {
  #[must_use]
  pub fn new(inner: Window) -> Self {
    let id = uuid::Uuid::new_v4();
    Self {
      id,
      inner,
      needs_configure: false,
    }
  }

  #[must_use]
  pub fn needs_configure(&self) -> bool {
    self.needs_configure
  }

  #[must_use]
  pub fn handle(&self) -> crate::WindowHandle {
    self.id
  }

  pub fn frame_position(&self) -> anyhow::Result<Rect> {
    // Assuming the frame position is the same as the window position
    let geom = self.inner.geometry();
    let rect =
      Rect::from_xy(geom.loc.x, geom.loc.y, geom.size.w, geom.size.h);
    Ok(rect)
  }

  pub fn is_minimized(&self) -> anyhow::Result<bool> {
    todo!()
  }

  pub fn minimize(&self) -> anyhow::Result<()> {
    todo!()
  }

  pub fn is_maximized(&self) -> anyhow::Result<bool> {
    todo!()
  }

  pub fn mark_fullscreen(&self, b: bool) -> anyhow::Result<()> {
    let toplevel = self.toplevel().ok_or_else(|| {
      anyhow::anyhow!(
        "Window is not a toplevel window, cannot mark fullscreen"
      )
    })?;
    if toplevel
      .current_state()
      .states
      .contains(xdg_toplevel::State::Fullscreen)
      == b
    {
      return Ok(());
    }
    toplevel.with_pending_state(|state| {
      if b {
        state.states.set(xdg_toplevel::State::Fullscreen);
      } else {
        state.states.unset(xdg_toplevel::State::Fullscreen);
      }
    });

    Ok(())
  }

  pub fn is_fullscreen(&self, rect: &Rect) -> anyhow::Result<bool> {
    todo!()
  }

  #[must_use]
  pub fn is_resizable(&self) -> bool {
    todo!()
  }

  pub fn set_taskbar_visibility(&self, b: bool) -> anyhow::Result<()> {
    todo!()
  }

  pub fn set_border_color(
    &self,
    color: Option<&Color>,
  ) -> anyhow::Result<()> {
    todo!()
  }

  pub fn set_title_bar_visibility(&self, b: bool) -> anyhow::Result<()> {
    todo!()
  }

  pub fn set_corner_style(&self, b: &CornerStyle) -> anyhow::Result<()> {
    todo!()
  }

  pub fn set_transparency(&self, v: &OpacityValue) -> anyhow::Result<()> {
    todo!()
  }

  pub fn set_foreground(&self) -> anyhow::Result<()> {
    self.set_activated(true);
    Ok(())
  }

  pub fn show(&self) -> anyhow::Result<()> {
    todo!()
  }

  pub fn set_position(
    &self,
    state: &WindowState,
    rect: &Rect,
    z_order: &ZOrder,
    is_visible: bool,
    hide_method: &HideMethod,
    has_pending_dpi_adjustment: bool,
  ) -> anyhow::Result<()> {
    todo!()
  }

  pub fn set_z_order(&self, _z_order: &ZOrder) -> anyhow::Result<()> {
    todo!()
  }
}

impl std::ops::Deref for NativeWindow {
  type Target = Window;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl std::ops::DerefMut for NativeWindow {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl SpaceElement for NativeWindow {
  fn bbox(
    &self,
  ) -> smithay::utils::Rectangle<i32, smithay::utils::Logical> {
    self.inner.bbox()
  }

  fn is_in_input_region(
    &self,
    point: &smithay::utils::Point<f64, smithay::utils::Logical>,
  ) -> bool {
    self.inner.is_in_input_region(point)
  }

  fn set_activate(&self, activated: bool) {
    self.inner.set_activate(activated);
  }

  fn output_enter(
    &self,
    output: &smithay::output::Output,
    overlap: smithay::utils::Rectangle<i32, smithay::utils::Logical>,
  ) {
    self.inner.output_enter(output, overlap);
  }

  fn output_leave(&self, output: &smithay::output::Output) {
    self.inner.output_leave(output);
  }
}

impl IsAlive for NativeWindow {
  fn alive(&self) -> bool {
    self.inner.alive()
  }
}

impl<R> AsRenderElements<R> for NativeWindow
where
  R: smithay::backend::renderer::ImportAll,
  <R as smithay::backend::renderer::RendererSuper>::TextureId:
    std::clone::Clone + 'static,
{
  type RenderElement = <smithay::desktop::Window as smithay::backend::renderer::element::AsRenderElements<R>>::RenderElement;

  fn render_elements<C: From<Self::RenderElement>>(
    &self,
    renderer: &mut R,
    location: smithay::utils::Point<i32, smithay::utils::Physical>,
    scale: smithay::utils::Scale<f64>,
    alpha: f32,
  ) -> Vec<C> {
    self.inner.render_elements(renderer, location, scale, alpha)
  }
}

impl WaylandFocus for NativeWindow {
  fn wl_surface(
    &self,
  ) -> Option<
    std::borrow::Cow<
      '_,
      smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    >,
  > {
    self.inner.wl_surface()
  }
}
