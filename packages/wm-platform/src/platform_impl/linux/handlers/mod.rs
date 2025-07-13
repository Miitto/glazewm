mod compositor;
mod xdg_shell;

//
// Wl Seat
use smithay::{
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
use crate::{Data, EventHandler};

impl<D, H> SeatHandler for Data<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  type KeyboardFocus = WlSurface;
  type PointerFocus = WlSurface;
  type TouchFocus = WlSurface;

  fn seat_state(&mut self) -> &mut SeatState<Data<D, H>> {
    &mut self.platform.state.state.seat
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
    let dh = &self.platform.display_handle;
    let client = focused.and_then(|s| dh.get_client(s.id()).ok());
    set_data_device_focus(dh, seat, client);
  }
}

/// Macro expansion for the `delegate_seat!` macro, since Data uses
/// generics
#[allow(clippy::semicolon_if_nothing_returned)]
mod seat_delegate {
  use smithay::reexports::wayland_server;

  use crate::{Data, EventHandler};
  impl<D, H>
    wayland_server::GlobalDispatch<
      smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
      smithay::wayland::seat::SeatGlobalData<Data<D, H>>,
    > for Data<D, H>
  where
    D: 'static,
    H: EventHandler<D> + 'static,
  {
    fn bind(
      state: &mut Self,
      dhandle: &wayland_server::DisplayHandle,
      client: &wayland_server::Client,
      resource: wayland_server::New<
        smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
      >,
      global_data: &smithay::wayland::seat::SeatGlobalData<Data<D, H>>,
      data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
      <smithay::input::SeatState<Data<D, H> >as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,smithay::wayland::seat::SeatGlobalData<Data<D, H> > ,Self>>::bind(state,dhandle,client,resource,global_data,data_init)
    }
    fn can_view(
      client: wayland_server::Client,
      global_data: &smithay::wayland::seat::SeatGlobalData<Data<D, H>>,
    ) -> bool {
      <smithay::input::SeatState<Data<D, H> >as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,smithay::wayland::seat::SeatGlobalData<Data<D, H> > ,Self>>::can_view(client,global_data)
    }
  }
  impl<D, H>
    wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
      smithay::wayland::seat::SeatUserData<Data<D, H>>,
    > for Data<D, H>
  where
    D: 'static,
    H: EventHandler<D> + 'static,
  {
    fn request(
      state: &mut Self,
      client: &wayland_server::Client,
      resource: &smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
      request: <smithay::reexports::wayland_server::protocol::wl_seat::WlSeat as wayland_server::Resource>::Request,
      data: &smithay::wayland::seat::SeatUserData<Data<D, H>>,
      dhandle: &wayland_server::DisplayHandle,
      data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
      <smithay::input::SeatState<Data<D, H>> as wayland_server::Dispatch<
        smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        smithay::wayland::seat::SeatUserData<Data<D, H>>,
        Self,
      >>::request(
        state, client, resource, request, data, dhandle, data_init,
      )
    }
    fn destroyed(
      state: &mut Self,
      client: wayland_server::backend::ClientId,
      resource: &smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
      data: &smithay::wayland::seat::SeatUserData<Data<D, H>>,
    ) {
      <smithay::input::SeatState<Data<D, H>> as wayland_server::Dispatch<
        smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        smithay::wayland::seat::SeatUserData<Data<D, H>>,
        Self,
      >>::destroyed(state, client, resource, data)
    }
  }
  impl<D, H>
    wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_pointer::WlPointer,
      smithay::wayland::seat::PointerUserData<Data<D, H>>,
    > for Data<D, H>
  where
    D: 'static,
    H: EventHandler<D> + 'static,
  {
    fn request(
      state: &mut Self,
      client: &wayland_server::Client,
      resource: &smithay::reexports::wayland_server::protocol::wl_pointer::WlPointer,
      request: <smithay::reexports::wayland_server::protocol::wl_pointer::WlPointer as wayland_server::Resource>::Request,
      data: &smithay::wayland::seat::PointerUserData<Data<D, H>>,
      dhandle: &wayland_server::DisplayHandle,
      data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
      <smithay::input::SeatState<Data<D, H> >as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_pointer::WlPointer,smithay::wayland::seat::PointerUserData<Data<D, H> > ,Self>>::request(state,client,resource,request,data,dhandle,data_init)
    }
    fn destroyed(
      state: &mut Self,
      client: wayland_server::backend::ClientId,
      resource: &smithay::reexports::wayland_server::protocol::wl_pointer::WlPointer,
      data: &smithay::wayland::seat::PointerUserData<Data<D, H>>,
    ) {
      <smithay::input::SeatState<Data<D, H> >as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_pointer::WlPointer,smithay::wayland::seat::PointerUserData<Data<D, H> > ,Self>>::destroyed(state,client,resource,data)
    }
  }
  impl<D, H> wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_keyboard::WlKeyboard,smithay::wayland::seat::KeyboardUserData<Data<D, H> > >for Data<D, H> where
D: 'static,
    H: EventHandler<D> + 'static,
{
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_server::protocol::wl_keyboard::WlKeyboard,request: <smithay::reexports::wayland_server::protocol::wl_keyboard::WlKeyboard as wayland_server::Resource>::Request,data: &smithay::wayland::seat::KeyboardUserData<Data<D, H> > ,dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::input::SeatState<Data<D, H> >as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_keyboard::WlKeyboard,smithay::wayland::seat::KeyboardUserData<Data<D, H> > ,Self>>::request(state,client,resource,request,data,dhandle,data_init)
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_server::protocol::wl_keyboard::WlKeyboard,data: &smithay::wayland::seat::KeyboardUserData<Data<D, H> >){
        <smithay::input::SeatState<Data<D, H> >as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_keyboard::WlKeyboard,smithay::wayland::seat::KeyboardUserData<Data<D, H> > ,Self>>::destroyed(state,client,resource,data)
    }

    }
  impl<D, H>
    wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_touch::WlTouch,
      smithay::wayland::seat::TouchUserData<Data<D, H>>,
    > for Data<D, H>
  where
    D: 'static,
    H: EventHandler<D> + 'static,
  {
    fn request(
      state: &mut Self,
      client: &wayland_server::Client,
      resource: &smithay::reexports::wayland_server::protocol::wl_touch::WlTouch,
      request: <smithay::reexports::wayland_server::protocol::wl_touch::WlTouch as wayland_server::Resource>::Request,
      data: &smithay::wayland::seat::TouchUserData<Data<D, H>>,
      dhandle: &wayland_server::DisplayHandle,
      data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
      <smithay::input::SeatState<Data<D, H>> as wayland_server::Dispatch<
        smithay::reexports::wayland_server::protocol::wl_touch::WlTouch,
        smithay::wayland::seat::TouchUserData<Data<D, H>>,
        Self,
      >>::request(
        state, client, resource, request, data, dhandle, data_init,
      )
    }
    fn destroyed(
      state: &mut Self,
      client: wayland_server::backend::ClientId,
      resource: &smithay::reexports::wayland_server::protocol::wl_touch::WlTouch,
      data: &smithay::wayland::seat::TouchUserData<Data<D, H>>,
    ) {
      <smithay::input::SeatState<Data<D, H>> as wayland_server::Dispatch<
        smithay::reexports::wayland_server::protocol::wl_touch::WlTouch,
        smithay::wayland::seat::TouchUserData<Data<D, H>>,
        Self,
      >>::destroyed(state, client, resource, data)
    }
  }
}

//
// Wl Data Device
//

impl<D, H> SelectionHandler for Data<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  type SelectionUserData = ();
}

impl<D, H> DataDeviceHandler for Data<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
  fn data_device_state(&self) -> &DataDeviceState {
    &self.platform.state.state.data_device
  }
}

impl<D, H> ClientDndGrabHandler for Data<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
}
impl<D, H> ServerDndGrabHandler for Data<D, H>
where
  D: 'static,
  H: EventHandler<D> + 'static,
{
}

/// Macro expansion for the `delegate_data_device!` macro, since Data uses
/// generics
#[allow(clippy::semicolon_if_nothing_returned)]
mod data_device_delegate {
  use smithay::reexports::wayland_server;

  use super::Data;
  use crate::EventHandler;
  impl<D: 'static, H> wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,()>for Data<D, H> where  H: EventHandler<D> + 'static {
    fn bind(state: &mut Self,dhandle: &wayland_server::DisplayHandle,client: &wayland_server::Client,resource:wayland_server::New<smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager>,global_data: &(),data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,(),Self>>::bind(state,dhandle,client,resource,global_data,data_init)
    }
    fn can_view(client:wayland_server::Client,global_data: &()) -> bool {
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,(),Self>>::can_view(client,global_data)
    }

    }
  impl<D: 'static, H> wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,()>for Data<D, H> where  H: EventHandler<D> + 'static{
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,request: <smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager as wayland_server::Resource>::Request,data: &(),dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,(),Self>>::request(state,client,resource,request,data,dhandle,data_init)
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,data: &()){
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_device_manager::WlDataDeviceManager,(),Self>>::destroyed(state,client,resource,data)
    }

    }
  impl<D: 'static, H> wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_device::WlDataDevice,smithay::wayland::selection::data_device::DataDeviceUserData>for Data<D, H> where  H: EventHandler<D> + 'static{
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_server::protocol::wl_data_device::WlDataDevice,request: <smithay::reexports::wayland_server::protocol::wl_data_device::WlDataDevice as wayland_server::Resource>::Request,data: &smithay::wayland::selection::data_device::DataDeviceUserData,dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_device::WlDataDevice,smithay::wayland::selection::data_device::DataDeviceUserData,Self>>::request(state,client,resource,request,data,dhandle,data_init)
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_server::protocol::wl_data_device::WlDataDevice,data: &smithay::wayland::selection::data_device::DataDeviceUserData){
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_device::WlDataDevice,smithay::wayland::selection::data_device::DataDeviceUserData,Self>>::destroyed(state,client,resource,data)
    }

    }
  impl<D: 'static, H> wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource,smithay::wayland::selection::data_device::DataSourceUserData>for Data<D, H> where  H: EventHandler<D> + 'static{
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource,request: <smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource as wayland_server::Resource>::Request,data: &smithay::wayland::selection::data_device::DataSourceUserData,dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource,smithay::wayland::selection::data_device::DataSourceUserData,Self>>::request(state,client,resource,request,data,dhandle,data_init)
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource,data: &smithay::wayland::selection::data_device::DataSourceUserData){
        <smithay::wayland::selection::data_device::DataDeviceState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource,smithay::wayland::selection::data_device::DataSourceUserData,Self>>::destroyed(state,client,resource,data)
    }

    }
}

//
// Wl Output & Xdg Output
//

impl<D, H> OutputHandler for Data<D, H> where H: EventHandler<D> {}
mod output_delegate {
  use smithay::reexports::wayland_server;

  use super::Data;
  use crate::EventHandler;
  impl<D: 'static, H: EventHandler<D> + 'static>
    wayland_server::GlobalDispatch<
      smithay::reexports::wayland_server::protocol::wl_output::WlOutput,
      smithay::wayland::output::WlOutputData,
    > for Data<D, H>
  {
    fn bind(
      state: &mut Self,
      dhandle: &wayland_server::DisplayHandle,
      client: &wayland_server::Client,
      resource: wayland_server::New<
        smithay::reexports::wayland_server::protocol::wl_output::WlOutput,
      >,
      global_data: &smithay::wayland::output::WlOutputData,
      data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
      <smithay::wayland::output::OutputManagerState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_output::WlOutput,smithay::wayland::output::WlOutputData,Self>>::bind(state,dhandle,client,resource,global_data,data_init);
    }
    fn can_view(
      client: wayland_server::Client,
      global_data: &smithay::wayland::output::WlOutputData,
    ) -> bool {
      <smithay::wayland::output::OutputManagerState as wayland_server::GlobalDispatch<smithay::reexports::wayland_server::protocol::wl_output::WlOutput,smithay::wayland::output::WlOutputData,Self>>::can_view(client,global_data)
    }
  }
  impl<D: 'static, H: EventHandler<D> + 'static> wayland_server::GlobalDispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,()>for Data<D, H>{
    fn bind(state: &mut Self,dhandle: &wayland_server::DisplayHandle,client: &wayland_server::Client,resource:wayland_server::New<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1>,global_data: &(),data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::output::OutputManagerState as wayland_server::GlobalDispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,(),Self>>::bind(state,dhandle,client,resource,global_data,data_init);
    }
    fn can_view(client:wayland_server::Client,global_data: &()) -> bool {
        <smithay::wayland::output::OutputManagerState as wayland_server::GlobalDispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,(),Self>>::can_view(client,global_data)
    }

    }
  impl<D: 'static, H: EventHandler<D>>
    wayland_server::Dispatch<
      smithay::reexports::wayland_server::protocol::wl_output::WlOutput,
      smithay::wayland::output::OutputUserData,
    > for Data<D, H>
  {
    fn request(
      state: &mut Self,
      client: &wayland_server::Client,
      resource: &smithay::reexports::wayland_server::protocol::wl_output::WlOutput,
      request: <smithay::reexports::wayland_server::protocol::wl_output::WlOutput as wayland_server::Resource>::Request,
      data: &smithay::wayland::output::OutputUserData,
      dhandle: &wayland_server::DisplayHandle,
      data_init: &mut wayland_server::DataInit<'_, Self>,
    ) {
      <smithay::wayland::output::OutputManagerState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_output::WlOutput,smithay::wayland::output::OutputUserData,Self>>::request(state,client,resource,request,data,dhandle,data_init);
    }
    fn destroyed(
      state: &mut Self,
      client: wayland_server::backend::ClientId,
      resource: &smithay::reexports::wayland_server::protocol::wl_output::WlOutput,
      data: &smithay::wayland::output::OutputUserData,
    ) {
      <smithay::wayland::output::OutputManagerState as wayland_server::Dispatch<smithay::reexports::wayland_server::protocol::wl_output::WlOutput,smithay::wayland::output::OutputUserData,Self>>::destroyed(state,client,resource,data);
    }
  }

  impl<D: 'static, H: EventHandler<D>> wayland_server::Dispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_v1::ZxdgOutputV1,smithay::wayland::output::XdgOutputUserData>for Data<D, H>{
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_v1::ZxdgOutputV1,request: <smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_v1::ZxdgOutputV1 as wayland_server::Resource>::Request,data: &smithay::wayland::output::XdgOutputUserData,dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::output::OutputManagerState as wayland_server::Dispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_v1::ZxdgOutputV1,smithay::wayland::output::XdgOutputUserData,Self>>::request(state,client,resource,request,data,dhandle,data_init);
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_v1::ZxdgOutputV1,data: &smithay::wayland::output::XdgOutputUserData){
        <smithay::wayland::output::OutputManagerState as wayland_server::Dispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_v1::ZxdgOutputV1,smithay::wayland::output::XdgOutputUserData,Self>>::destroyed(state,client,resource,data);
    }

    }

  impl<D: 'static, H: EventHandler<D> + 'static> wayland_server::Dispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,()>for Data<D, H>{
    fn request(state: &mut Self,client: &wayland_server::Client,resource: &smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,request: <smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1 as wayland_server::Resource>::Request,data: &(),dhandle: &wayland_server::DisplayHandle,data_init: &mut wayland_server::DataInit<'_,Self>,){
        <smithay::wayland::output::OutputManagerState as wayland_server::Dispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,(),Self>>::request(state,client,resource,request,data,dhandle,data_init);
    }
    fn destroyed(state: &mut Self,client:wayland_server::backend::ClientId,resource: &smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,data: &()){
        <smithay::wayland::output::OutputManagerState as wayland_server::Dispatch<smithay::reexports::wayland_protocols::xdg::xdg_output::zv1::server::zxdg_output_manager_v1::ZxdgOutputManagerV1,(),Self>>::destroyed(state,client,resource,data);
    }

    }
}
