// TODO: Linux keyboard hook
#[derive(Debug, Default)]
pub struct Hook;

impl Hook {
  pub fn start() -> anyhow::Result<Self> {
    todo!("Implement Linux keyboard hook start logic");
  }

  pub fn stop(self) -> anyhow::Result<()> {
    todo!("Implement Linux keyboard hook stop logic");
  }
}
