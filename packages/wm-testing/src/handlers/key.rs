use crate::state::State;

#[derive(Default)]
pub struct KeyEventHandler {}

impl wm_platform::KeyEventHandler<State, crate::handlers::Handler>
  for KeyEventHandler
{
  fn key_event(
    &mut self,
    data: &mut State,
    platform: &mut wm_platform::PlatformData<
      State,
      crate::handlers::Handler,
    >,
    key: wm_platform::KeyData,
  ) -> wm_platform::KeyResponse {
    tracing::info!("Key event: {:?}", key);
    wm_platform::KeyResponse::DontCare
  }
}
