use crate::CommonSingleInstance;

pub struct SingleInstance;

impl CommonSingleInstance for SingleInstance {
  fn new() -> anyhow::Result<Self> {
    todo!();
  }

  fn is_running() -> bool {
    todo!();
  }
}
