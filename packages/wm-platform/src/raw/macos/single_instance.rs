use crate::CommonSingleInstance;

pub struct SingleInstance;

impl CommonSingleInstance for SingleInstance {
  fn new() -> anyhow::Result<Self> {
    todo!("Implement single instance for Linux");
  }

  fn is_running() -> bool {
    todo!("Implement single instance check for Linux");
  }
}
