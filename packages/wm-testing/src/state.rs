pub struct State {
  pub platform: wm_platform::PlatformData,
  pub config: crate::user_config::UserConfig,
}

impl State {
  pub fn new(
    platform: wm_platform::PlatformData,
    config: crate::user_config::UserConfig,
  ) -> Self {
    Self { platform, config }
  }
}

impl wm_platform::EventLoopData for State {
  fn platform_data(&self) -> &wm_platform::PlatformData {
    &self.platform
  }

  fn platform_data_mut(&mut self) -> &mut wm_platform::PlatformData {
    &mut self.platform
  }

  fn config(&self) -> &wm_common::ParsedConfig {
    &self.config.value
  }
}
