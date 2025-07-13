use smithay::{
  backend::renderer::utils::on_commit_buffer_handler,
  reexports::wayland_server::{
    self,
    protocol::{wl_buffer, wl_surface::WlSurface},
    Client,
  },
  wayland::{
    buffer::BufferHandler,
    compositor::{
      get_parent, is_sync_subsurface, CompositorClientState,
      CompositorHandler, CompositorState,
    },
    shm::{ShmHandler, ShmState},
  },
};

use super::xdg_shell;
use crate::{grabs::resize_grab, state::ClientState, Data, EventHandler};

impl<D, H> CompositorHandler for Data<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  fn compositor_state(&mut self) -> &mut CompositorState {
    &mut self.platform.state.state.compositor
  }

  fn client_compositor_state<'a>(
    &self,
    client: &'a Client,
  ) -> &'a CompositorClientState {
    &client.get_data::<ClientState>().unwrap().compositor_state
  }

  fn commit(&mut self, surface: &WlSurface) {
    on_commit_buffer_handler::<Self>(surface);
    let state = &mut self.platform.state;
    if !is_sync_subsurface(surface) {
      let mut root = surface.clone();
      while let Some(parent) = get_parent(&root) {
        root = parent;
      }
      if let Some(window) = state
        .space
        .elements()
        .find(|w| w.toplevel().unwrap().wl_surface() == &root)
      {
        window.on_commit();
      }
    }

    xdg_shell::handle_commit(&mut state.popups, &state.space, surface);
    resize_grab::handle_commit(&mut state.space, surface);
  }
}

impl<D, H> BufferHandler for Data<D, H>
where
  H: EventHandler<D>,
{
  fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

impl<D, H> ShmHandler for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn shm_state(&self) -> &ShmState {
    &self.platform.state.state.shm
  }
}

// NOTE: EVERYTHING BELOW THIS LINE CAN BE IGNORED

// Inline of delegate_compositor! macro since Data used generics
impl<D: 'static, H> wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,()>for Data<D, H> where  H: EventHandler<D> + 'static {
    fn bind(state: &mut Self,dhandle: &wayland_server::DisplayHandle,client: &wayland_server::Client,resource:wayland_server::New<smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor>,global_data: &(),data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::compositor::CompositorState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,(),Self>>::bind(state,dhandle,client,resource,global_data,data_init);
    }
    fn can_view(client:wayland_server::Client,global_data: &()) -> bool {
        <smithay::wayland::compositor::CompositorState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,(),Self>>::can_view(client,global_data)
    }

    }
impl<D: 'static, H> wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,()>for Data<D, H> where  H: EventHandler<D> + 'static {
    fn bind(state: &mut Self,dhandle: &wayland_server::DisplayHandle,client: &wayland_server::Client,resource:wayland_server::New<smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor>,global_data: &(),data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::compositor::CompositorState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,(),Self>>::bind(state,dhandle,client,resource,global_data,data_init);
    }
    fn can_view(client:wayland_server::Client,global_data: &()) -> bool {
        <smithay::wayland::compositor::CompositorState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,(),Self>>::can_view(client,global_data)
    }

    }
impl<D: 'static, H> wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,()>for Data<D, H> where  H: EventHandler<D> + 'static {
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,request: <smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor as wayland_server::Resource>::Request,data: &(),dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,(),Self>>::request(state,client,resource,request,data,dhandle,data_init);
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,data: &()){
        <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_compositor::WlCompositor,(),Self>>::destroyed(state,client,resource,data);
    }

    }
impl<D: 'static, H>
  wayland_server::Dispatch<
    smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    smithay::wayland::compositor::SurfaceUserData,
  > for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn request(
    state: &mut Self,
    client: &wayland_server::Client,
    resource: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    request: <smithay::reexports::wayland_server::protocol::wl_surface::WlSurface as wayland_server::Resource>::Request,
    data: &smithay::wayland::compositor::SurfaceUserData,
    dhandle: &wayland_server::DisplayHandle,
    data_init: &mut wayland_server::DataInit<'_, Self>,
  ) {
    <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,smithay::wayland::compositor::SurfaceUserData,Self>>::request(state,client,resource,request,data,dhandle,data_init);
  }
  fn destroyed(
    state: &mut Self,
    client: wayland_server::backend::ClientId,
    resource: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    data: &smithay::wayland::compositor::SurfaceUserData,
  ) {
    <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,smithay::wayland::compositor::SurfaceUserData,Self>>::destroyed(state,client,resource,data);
  }
}
impl<D: 'static, H>
  wayland_server::Dispatch<
    smithay::reexports::wayland_server::protocol::wl_region::WlRegion,
    smithay::wayland::compositor::RegionUserData,
  > for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn request(
    state: &mut Self,
    client: &wayland_server::Client,
    resource: &smithay::reexports::wayland_server::protocol::wl_region::WlRegion,
    request: <smithay::reexports::wayland_server::protocol::wl_region::WlRegion as wayland_server::Resource>::Request,
    data: &smithay::wayland::compositor::RegionUserData,
    dhandle: &wayland_server::DisplayHandle,
    data_init: &mut wayland_server::DataInit<'_, Self>,
  ) {
    <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_region::WlRegion,smithay::wayland::compositor::RegionUserData,Self>>::request(state,client,resource,request,data,dhandle,data_init);
  }
  fn destroyed(
    state: &mut Self,
    client: wayland_server::backend::ClientId,
    resource: &smithay::reexports::wayland_server::protocol::wl_region::WlRegion,
    data: &smithay::wayland::compositor::RegionUserData,
  ) {
    <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_region::WlRegion,smithay::wayland::compositor::RegionUserData,Self>>::destroyed(state,client,resource,data);
  }
}
impl<D: 'static, H>
  wayland_server::Dispatch<
    smithay::reexports::wayland_server::protocol::wl_callback::WlCallback,
    (),
  > for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn request(
    state: &mut Self,
    client: &wayland_server::Client,
    resource: &smithay::reexports::wayland_server::protocol::wl_callback::WlCallback,
    request: <smithay::reexports::wayland_server::protocol::wl_callback::WlCallback as wayland_server::Resource>::Request,
    data: &(),
    dhandle: &wayland_server::DisplayHandle,
    data_init: &mut wayland_server::DataInit<'_, Self>,
  ) {
    <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_callback::WlCallback,(),Self>>::request(state,client,resource,request,data,dhandle,data_init);
  }
  fn destroyed(
    state: &mut Self,
    client: wayland_server::backend::ClientId,
    resource: &smithay::reexports::wayland_server::protocol::wl_callback::WlCallback,
    data: &(),
  ) {
    <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_callback::WlCallback,(),Self>>::destroyed(state,client,resource,data);
  }
}
impl<D: 'static, H> wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,()>for Data<D, H> where  H: EventHandler<D> + 'static {
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,request: <smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor as wayland_server::Resource>::Request,data: &(),dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,(),Self>>::request(state,client,resource,request,data,dhandle,data_init);
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,data: &()){
        <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_subcompositor::WlSubcompositor,(),Self>>::destroyed(state,client,resource,data);
    }

    }
impl<D: 'static, H> wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_subsurface::WlSubsurface,smithay::wayland::compositor::SubsurfaceUserData>for Data<D, H> where  H: EventHandler<D> + 'static {
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_server::protocol::wl_subsurface::WlSubsurface,request: <smithay::reexports::wayland_server::protocol::wl_subsurface::WlSubsurface as wayland_server::Resource>::Request,data: &smithay::wayland::compositor::SubsurfaceUserData,dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_subsurface::WlSubsurface,smithay::wayland::compositor::SubsurfaceUserData,Self>>::request(state,client,resource,request,data,dhandle,data_init);
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_server::protocol::wl_subsurface::WlSubsurface,data: &smithay::wayland::compositor::SubsurfaceUserData){
        <smithay::wayland::compositor::CompositorState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_subsurface::WlSubsurface,smithay::wayland::compositor::SubsurfaceUserData,Self>>::destroyed(state,client,resource,data);
    }

    }

// Macro expansion of delegate_shm! macro since Data used generics
impl<D: 'static, H>
  wayland_server::GlobalDispatch<
    smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
    (),
  > for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn bind(
    state: &mut Self,
    dhandle: &wayland_server::DisplayHandle,
    client: &wayland_server::Client,
    resource: wayland_server::New<
      smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
    >,
    global_data: &(),
    data_init: &mut wayland_server::DataInit<'_, Self>,
  ) {
    <smithay::wayland::shm::ShmState as wayland_server::GlobalDispatch<
      smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
      (),
      Self,
    >>::bind(state, dhandle, client, resource, global_data, data_init);
  }
  fn can_view(client: wayland_server::Client, global_data: &()) -> bool {
    <smithay::wayland::shm::ShmState as wayland_server::GlobalDispatch<
      smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
      (),
      Self,
    >>::can_view(client, global_data)
  }
}
impl<D: 'static, H>
  wayland_server::Dispatch<
    smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
    (),
  > for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn request(
    state: &mut Self,
    client: &wayland_server::Client,
    resource: &smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
    request: <smithay::reexports::wayland_server::protocol::wl_shm::WlShm as wayland_server::Resource>::Request,
    data: &(),
    dhandle: &wayland_server::DisplayHandle,
    data_init: &mut wayland_server::DataInit<'_, Self>,
  ) {
    <smithay::wayland::shm::ShmState as wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
      (),
      Self,
    >>::request(
      state, client, resource, request, data, dhandle, data_init,
    );
  }
  fn destroyed(
    state: &mut Self,
    client: wayland_server::backend::ClientId,
    resource: &smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
    data: &(),
  ) {
    <smithay::wayland::shm::ShmState as wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_shm::WlShm,
      (),
      Self,
    >>::destroyed(state, client, resource, data);
  }
}
impl<D: 'static, H>
  wayland_server::Dispatch<
    smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool,
    smithay::wayland::shm::ShmPoolUserData,
  > for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn request(
    state: &mut Self,
    client: &wayland_server::Client,
    resource: &smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool,
    request: <smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool as wayland_server::Resource>::Request,
    data: &smithay::wayland::shm::ShmPoolUserData,
    dhandle: &wayland_server::DisplayHandle,
    data_init: &mut wayland_server::DataInit<'_, Self>,
  ) {
    <smithay::wayland::shm::ShmState as wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool,
      smithay::wayland::shm::ShmPoolUserData,
      Self,
    >>::request(
      state, client, resource, request, data, dhandle, data_init,
    );
  }
  fn destroyed(
    state: &mut Self,
    client: wayland_server::backend::ClientId,
    resource: &smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool,
    data: &smithay::wayland::shm::ShmPoolUserData,
  ) {
    <smithay::wayland::shm::ShmState as wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool,
      smithay::wayland::shm::ShmPoolUserData,
      Self,
    >>::destroyed(state, client, resource, data);
  }
}
impl<D: 'static, H>
  wayland_server::Dispatch<
    smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    smithay::wayland::shm::ShmBufferUserData,
  > for Data<D, H>
where
  H: EventHandler<D> + 'static,
{
  fn request(
    state: &mut Self,
    client: &wayland_server::Client,
    resource: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    request: <smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer as wayland_server::Resource>::Request,
    data: &smithay::wayland::shm::ShmBufferUserData,
    dhandle: &wayland_server::DisplayHandle,
    data_init: &mut wayland_server::DataInit<'_, Self>,
  ) {
    <smithay::wayland::shm::ShmState as wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
      smithay::wayland::shm::ShmBufferUserData,
      Self,
    >>::request(
      state, client, resource, request, data, dhandle, data_init,
    );
  }
  fn destroyed(
    state: &mut Self,
    client: wayland_server::backend::ClientId,
    resource: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    data: &smithay::wayland::shm::ShmBufferUserData,
  ) {
    <smithay::wayland::shm::ShmState as wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
      smithay::wayland::shm::ShmBufferUserData,
      Self,
    >>::destroyed(state, client, resource, data);
  }
}
