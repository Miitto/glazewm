pub struct State {
  pub config: crate::user_config::UserConfig,
}

impl State {
  pub fn new(config: crate::user_config::UserConfig) -> Self {
    Self { config }
  }
}
