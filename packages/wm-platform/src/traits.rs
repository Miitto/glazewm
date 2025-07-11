pub trait EventLoopData {
  fn platform_data(&self) -> &crate::PlatformData;
  fn platform_data_mut(&mut self) -> &mut crate::PlatformData;

  fn config(&self) -> &wm_common::ParsedConfig;
}
