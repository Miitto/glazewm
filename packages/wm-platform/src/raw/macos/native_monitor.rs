use crate::CommonNativeMonitor;

#[derive(Debug, Clone, PartialEq)]
pub struct NativeMonitor;

impl CommonNativeMonitor for NativeMonitor {
  fn new(handle: crate::MonitorHandle) -> Self {
    todo!()
  }

  fn handle(&self) -> crate::MonitorHandle {
    todo!()
  }

  fn device_name(&self) -> anyhow::Result<&String> {
    todo!()
  }

  fn device_path(&self) -> anyhow::Result<Option<&String>> {
    todo!()
  }

  fn hardware_id(&self) -> anyhow::Result<Option<&String>> {
    todo!()
  }

  fn rect(&self) -> anyhow::Result<&wm_common::Rect> {
    todo!()
  }

  fn working_rect(&self) -> anyhow::Result<&wm_common::Rect> {
    todo!()
  }

  fn dpi(&self) -> anyhow::Result<u32> {
    todo!()
  }

  fn scale_factor(&self) -> anyhow::Result<f32> {
    todo!()
  }
}
