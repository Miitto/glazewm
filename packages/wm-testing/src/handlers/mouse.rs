use crate::state::State;

#[derive(Default)]
pub struct MouseEventHandler {}

impl wm_platform::MouseEventHandler<State, crate::handlers::Handler>
  for MouseEventHandler
{
}
