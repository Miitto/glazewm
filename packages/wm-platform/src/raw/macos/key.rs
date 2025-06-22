#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, wm_macros::TryToDiscriminant,
)]
#[repr(u16)]
pub enum LinuxKey {
  // FIXME: This is a placeholder for the actual Linux key codes.
  A = 0,
}
