use windows::Win32::{
  Foundation::{LPARAM, LRESULT, WPARAM},
  UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HHOOK,
    KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
  },
};

#[derive(Debug, Default)]
pub struct Hook(HHOOK);

impl Hook {
  pub fn start() -> anyhow::Result<Self> {
    let hook = unsafe {
      SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0)?
    };

    Ok(Self(hook))
  }

  pub fn stop(self) -> anyhow::Result<()> {
    unsafe { UnhookWindowsHookEx(self.0) }?;
    Ok(())
  }
}

extern "system" fn keyboard_hook_proc(
  code: i32,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  #[allow(clippy::cast_possible_truncation)]
  let should_ignore = code != 0
    || !(wparam.0 as u32 == WM_KEYDOWN
      || wparam.0 as u32 == WM_SYSKEYDOWN);

  // If the code is less than zero, the hook procedure must pass the hook
  // notification directly to other applications. We also only care about
  // keydown events.
  if should_ignore {
    return unsafe { CallNextHookEx(None, code, wparam, lparam) };
  }

  // Get struct with keyboard input event.
  let input = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };

  if let Some(hook) = crate::keyboard_hook::KEYBOARD_HOOK.get() {
    #[allow(clippy::cast_possible_truncation)]
    let should_block = hook.handle_key_event(input.vkCode as u16);

    if should_block {
      return LRESULT(1);
    }
  }

  unsafe { CallNextHookEx(None, code, wparam, lparam) }
}
