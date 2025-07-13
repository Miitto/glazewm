use crate::state::State;

#[derive(Default)]
pub struct DisplayEventHandler {}

impl wm_platform::DisplayEventHandler<State, crate::handlers::Handler>
  for DisplayEventHandler
{
}
