use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use wm_macros::KeyConversions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, KeyConversions)]
#[key(win = crate::windows::key::WinKey, macos = !)]
pub enum Key {
  #[key("a", win = A, macos = !)]
  A,
  #[key("abntc1", win = AbntC1, macos = !)]
  AbntC1,
  #[key("abntc2", win = AbntC2, macos = !)]
  AbntC2,
  #[key("accept", win = Accept, macos = !)]
  Accept,
  #[key("add", win = Add, macos = !)]
  Add,
  #[key("apps", win = Apps, macos = !)]
  Apps,
  #[key("attn", win = Attn, macos = !)]
  Attn,
  #[key("b", win = B, macos = !)]
  B,
  #[key("back", win = Back, macos = !)]
  Back,
  #[key("browser back", win = BrowserBack, macos = !)]
  BrowserBack,
  #[key("browser favorites", win = BrowserFavorites, macos = !)]
  BrowserFavourites,
  #[key("browser forward", win = BrowserForward, macos = !)]
  BrowserForward,
  #[key("browser home", win = BrowserHome, macos = !)]
  BrowserHome,
  #[key("browser refresh", win = BrowserRefresh, macos = !)]
  BrowserRefresh,
  #[key("browser search", win = BrowserSearch, macos = !)]
  BrowserSearch,
  #[key("browser stop", win = BrowserStop, macos = !)]
  BrowserStop,
  #[key("c", win = C, macos = !)]
  C,
  #[key("cancel", win = Cancel, macos = !)]
  Cancel,
  #[key("capital", win = Capital, macos = !)]
  Capital,
  #[key("clear", win = Clear, macos = !)]
  Clear,
  #[key("control" | "ctrl" | "ctrl key" | "control key", win = Control, macos = !)]
  Control,
  #[key("convert", win = Convert, macos = !)]
  Convert,
  #[key("crsel", win = Crsel, macos = !)]
  Crsel,
  #[key("d", win = D, macos = !)]
  D,
  #[key("0", win = D0, macos = !)]
  D0,
  #[key("1", win = D1, macos = !)]
  D1,
  #[key("2", win = D2, macos = !)]
  D2,
  #[key("3", win = D3, macos = !)]
  D3,
  #[key("4", win = D4, macos = !)]
  D4,
  #[key("5", win = D5, macos = !)]
  D5,
  #[key("6", win = D6, macos = !)]
  D6,
  #[key("7", win = D7, macos = !)]
  D7,
  #[key("8", win = D8, macos = !)]
  D8,
  #[key("9", win = D9, macos = !)]
  D9,
  #[key("decimal", win = Decimal, macos = !)]
  Decimal,
  #[key("delete" | "del", win = Delete, macos = !)]
  Delete,
  #[key("divide", win = Divide, macos = !)]
  Divide,
  #[key("down", win = Down, macos = !)]
  Down,
  #[key("e", win = E, macos = !)]
  E,
  #[key("end", win = End, macos = !)]
  End,
  #[key("ereof", win = Ereof, macos = !)]
  Ereof,
  #[key("escape" | "esc", win = Escape, macos = !)]
  Escape,
  #[key("execute" | "exec", win = Execute, macos = !)]
  Execute,
  #[key("exsel", win = Exsel, macos = !)]
  Exsel,
  #[key("f", win = F, macos = !)]
  F,
  #[key("f1", win = F1, macos = !)]
  F1,
  #[key("f2", win = F2, macos = !)]
  F2,
  #[key("f3", win = F3, macos = !)]
  F3,
  #[key("f4", win = F4, macos = !)]
  F4,
  #[key("f5", win = F5, macos = !)]
  F5,
  #[key("f6", win = F6, macos = !)]
  F6,
  #[key("f7", win = F7, macos = !)]
  F7,
  #[key("f8", win = F8, macos = !)]
  F8,
  #[key("f9", win = F9, macos = !)]
  F9,
  #[key("f10", win = F10, macos = !)]
  F10,
  #[key("f11", win = F11, macos = !)]
  F11,
  #[key("f12", win = F12, macos = !)]
  F12,
  #[key("f13", win = F13, macos = !)]
  F13,
  #[key("f14", win = F14, macos = !)]
  F14,
  #[key("f15", win = F15, macos = !)]
  F15,
  #[key("f16", win = F16, macos = !)]
  F16,
  #[key("f17", win = F17, macos = !)]
  F17,
  #[key("f18", win = F18, macos = !)]
  F18,
  #[key("f19", win = F19, macos = !)]
  F19,
  #[key("f20", win = F20, macos = !)]
  F20,
  #[key("f21", win = F21, macos = !)]
  F21,
  #[key("f22", win = F22, macos = !)]
  F22,
  #[key("f23", win = F23, macos = !)]
  F23,
  #[key("f24", win = F24, macos = !)]
  F24,
  #[key("final", win = Final, macos = !)]
  Final,
  #[key("g", win = G, macos = !)]
  G,
  #[key("game pad a", win = GamepadA, macos = !)]
  GamepadA,
  #[key("game pad b", win = GamepadB, macos = !)]
  GamepadB,
  #[key("game pad d pad down", win = GamepadDpadDown, macos = !)]
  GamepadDpadDown,
  #[key("game pad d pad left", win = GamepadDpadLeft, macos = !)]
  GamepadDpadLeft,
  #[key("game pad d pad right", win = GamepadDpadRight, macos = !)]
  GamepadDpadRight,
  #[key("game pad d pad up", win = GamepadDpadUp, macos = !)]
  GamepadDpadUp,
  #[key("game pad left shoulder", win = GamepadLeftShoulder, macos = !)]
  GamepadLeftShoulder,
  #[key("game pad left stick" | "gamepad left thumb stick" | "gamepad left stick button" | "gamepad left thumb stick button", win = GamepadLeftThumbstickButton, macos = !)]
  GamepadLeftThumbstickButton,
  #[key("game pad left stick down" | "gamepad left thumb stick down", win = GamepadLeftThumbstickDown, macos = !)]
  GamepadLeftThumbstickDown,
  #[key("game pad left stick left" | "gamepad left thumb stick left", win = GamepadLeftThumbstickLeft, macos = !)]
  GamepadLeftThumbstickLeft,
  #[key("game pad left stick right" | "gamepad left thumb stick right", win = GamepadLeftThumbstickRight, macos = !)]
  GamepadLeftThumbstickRight,
  #[key("game pad left stick up" | "gamepad left thumb stick up", win = GamepadLeftThumbstickUp, macos = !)]
  GamepadLeftThumbstickUp,
  #[key("game pad left trigger", win = GamepadLeftTrigger, macos = !)]
  GamepadLeftTrigger,
  #[key("game pad right shoulder", win = GamepadRightShoulder, macos = !)]
  GamepadRightShoulder,
  #[key("game pad menu", win = GamepadMenu, macos = !)]
  GamepadMenu,
  #[key("game pad right stick" | "gamepad right thumb stick" | "gamepad right stick button" | "gamepad right thumb stick button", win = GamepadRightThumbstickButton, macos = !)]
  GamepadRightThumbstickButton,
  #[key("game pad right stick down" | "gamepad right thumb stick down", win = GamepadRightThumbstickDown, macos = !)]
  GamepadRightThumbstickDown,
  #[key("game pad right stick left" | "gamepad right thumb stick left", win = GamepadRightThumbstickLeft, macos = !)]
  GamepadRightThumbstickLeft,
  #[key("game pad right stick right" | "gamepad right thumb stick right", win = GamepadRightThumbstickRight, macos = !)]
  GamepadRightThumbstickRight,
  #[key("game pad right stick up" | "gamepad right thumb stick up", win = GamepadRightThumbstickUp, macos = !)]
  GamepadRightThumbstickUp,
  #[key("game pad right trigger", win = GamepadRightTrigger, macos = !)]
  GamepadRightTrigger,
  #[key("game pad view", win = GamepadView, macos = !)]
  GamepadView,
  #[key("game pad x", win = GamepadX, macos = !)]
  GamepadX,
  #[key("game pad y", win = GamepadY, macos = !)]
  GamepadY,
  #[key("h", win = H, macos = !)]
  H,
  #[key("help", win = Help, macos = !)]
  Help,
  #[key("home", win = Home, macos = !)]
  Home,
  #[key("i", win = I, macos = !)]
  I,
  #[key("ico 00", win = Ico00, macos = !)]
  Ico00,
  #[key("ico clear", win = IcoClear, macos = !)]
  IcoClear,
  #[key("ico help", win = IcoHelp, macos = !)]
  IcoHelp,
  #[key("ime off", win = ImeOff, macos = !)]
  ImeOff,
  #[key("ime on", win = ImeOn, macos = !)]
  ImeOn,
  #[key("insert", win = Insert, macos = !)]
  Insert,
  #[key("j", win = J, macos = !)]
  J,
  #[key("junja", win = Junja, macos = !)]
  Junja,
  #[key("k", win = K, macos = !)]
  K,
  #[key("kana", win = Kana, macos = !)]
  Kana,
  #[key("kanji", win = Kanji, macos = !)]
  Kanji,
  #[key("l", win = L, macos = !)]
  L,
  #[key("launch app 1", win = LaunchApp1, macos = !)]
  LaunchApp1,
  #[key("launch app 2", win = LaunchApp2, macos = !)]
  LaunchApp2,
  #[key("launch mail", win = LaunchMail, macos = !)]
  LaunchMail,
  #[key("launch media select", win = LaunchMediaSelect, macos = !)]
  LaunchMediaSelect,
  #[key("l button" | "left button", win = LButton, macos = !)]
  LButton,
  #[key("lcontrol" | "lctrl" | "lcontrol key" | "lctrl key" | "left control" | "left ctrl" | "left control key" | "left ctrl key", win = LControl, macos = !)]
  LControl,
  #[key("left" | "left arrow", win = Left, macos = !)]
  Left,
  #[key("lmenu" | "lalt" | "lalt key" | "left alt" | "left alt key", win = LMenu, macos = !)]
  LMenu,
  #[key("lshift" | "lshift key" | "left shift" | "left shift key", win = LShift, macos = !)]
  LShift,
  #[key("lwin" | "lwin key" | "left win" | "left win key", win = LWin, macos = !)]
  LWin,
  #[key("m", win = M, macos = !)]
  M,
  #[key("m button", win = MButton, macos = !)]
  MButton,
  #[key("media next track", win = MediaNextTrack, macos = !)]
  MediaNextTrack,
  #[key("media play pause", win = MediaPlayPause, macos = !)]
  MediaPlayPause,
  #[key("media previous track" | "media prev track", win = MediaPrevTrack, macos = !)]
  MediaPrevTrack,
  #[key("media stop", win = MediaStop, macos = !)]
  MediaStop,
  #[key("menu" | "alt", win = Menu, macos = !)]
  Menu,
  #[key("mode change", win = ModeChange, macos = !)]
  ModeChange,
  #[key("multiply", win = Multiply, macos = !)]
  Multiply,
  #[key("n", win = N, macos = !)]
  N,
  #[key("navigation accept" | "nav accept", win = NavigationAccept, macos = !)]
  NavigationAccept,
  #[key("navigation cancel" | "nav cancel", win = NavigationCancel, macos = !)]
  NavigationCancel,
  #[key("navigation down" | "nav down", win = NavigationDown, macos = !)]
  NavigationDown,
  #[key("navigation left" | "nav left", win = NavigationLeft, macos = !)]
  NavigationLeft,
  #[key("navigation menu" | "nav menu", win = NavigationMenu, macos = !)]
  NavigationMenu,
  #[key("naviagation right" | "nav right", win = NavigationRight, macos = !)]
  NavigationRight,
  #[key("navigation up" | "nav up", win = NavigationUp, macos = !)]
  NavigationUp,
  #[key("navigation view" | "nav view", win = NavigationView, macos = !)]
  NavigationView,
  #[key("next", win = Next, macos = !)]
  Next,
  #[key("no name", win = NoName, macos = !)]
  NoName,
  #[key("non convert", win = NonConvert, macos = !)]
  NonConvert,
  #[key("num lock", win = Numlock, macos = !)]
  Numlock,
  #[key("num pad 0" | "number pad 0", win = Numpad0, macos = !)]
  Numpad0,
  #[key("num pad 1" | "number pad 1", win = Numpad1, macos = !)]
  Numpad1,
  #[key("num pad 2" | "number pad 2", win = Numpad2, macos = !)]
  Numpad2,
  #[key("num pad 3" | "number pad 3", win = Numpad3, macos = !)]
  Numpad3,
  #[key("num pad 4" | "number pad 4", win = Numpad4, macos = !)]
  Numpad4,
  #[key("num pad 5" | "number pad 5", win = Numpad5, macos = !)]
  Numpad5,
  #[key("num pad 6" | "number pad 6", win = Numpad6, macos = !)]
  Numpad6,
  #[key("num pad 7" | "number pad 7", win = Numpad7, macos = !)]
  Numpad7,
  #[key("num pad 8" | "number pad 8", win = Numpad8, macos = !)]
  Numpad8,
  #[key("num pad 9" | "number pad 9", win = Numpad9, macos = !)]
  Numpad9,
  #[key("o", win = O, macos = !)]
  O,
  #[key("oem 1", win = Oem1, macos = !)]
  Oem1,
  #[key("oem 102", win = Oem102, macos = !)]
  Oem102,
  #[key("oem 2", win = Oem2, macos = !)]
  Oem2,
  #[key("oem 3", win = Oem3, macos = !)]
  Oem3,
  #[key("oem 4", win = Oem4, macos = !)]
  Oem4,
  #[key("oem 5", win = Oem5, macos = !)]
  Oem5,
  #[key("oem 6", win = Oem6, macos = !)]
  Oem6,
  #[key("oem 7", win = Oem7, macos = !)]
  Oem7,
  #[key("oem 8", win = Oem8, macos = !)]
  Oem8,
  #[key("oem attn", win = OemAttn, macos = !)]
  OemAttn,
  #[key("oem auto", win = OemAuto, macos = !)]
  OemAuto,
  #[key("oem ax", win = OemAx, macos = !)]
  OemAx,
  #[key("oem back tab", win = OemBacktab, macos = !)]
  OemBacktab,
  #[key("oem clear", win = OemClear, macos = !)]
  OemClear,
  #[key("oem comma", win = OemComma, macos = !)]
  OemComma,
  #[key("oem copy", win = OemCopy, macos = !)]
  OemCopy,
  #[key("oem cusel", win = OemCusel, macos = !)]
  OemCusel,
  #[key("oem enlw", win = OemEnlw, macos = !)]
  OemEnlw,
  #[key("oem finish", win = OemFinish, macos = !)]
  OemFinish,
  #[key("oem fj loya", win = OemFjLoya, macos = !)]
  OemFjLoya,
  #[key("oem fj masshou", win = OemFjMasshou, macos = !)]
  OemFjMasshou,
  #[key("oem fj roya", win = OemFjRoya, macos = !)]
  OemFjRoya,
  #[key("oem fj touroku", win = OemFjTouroku, macos = !)]
  OemFjTouroku,
  #[key("oem jump", win = OemJump, macos = !)]
  OemJump,
  #[key("oem minus", win = OemMinus, macos = !)]
  OemMinus,
  #[key("oem nec equal", win = OemNecEqual, macos = !)]
  OemNecEqual,
  #[key("oem pa 1", win = OemPa1, macos = !)]
  OemPa1,
  #[key("oem pa 2", win = OemPa2, macos = !)]
  OemPa2,
  #[key("oem pa 3", win = OemPa3, macos = !)]
  OemPa3,
  #[key("oem period" | "oem dot", win = OemPeriod, macos = !)]
  OemPeriod,
  #[key("oem plus", win = OemPlus, macos = !)]
  OemPlus,
  #[key("oem reset", win = OemReset, macos = !)]
  OemReset,
  #[key("oem ws ctrl" | "oem ws control", win = OemWsCtrl, macos = !)]
  OemWsCtrl,
  #[key("p", win = P, macos = !)]
  P,
  #[key("pa 1", win = PA1, macos = !)]
  PA1,
  #[key("packet", win = Packet, macos = !)]
  Packet,
  #[key("pause" | "pause break" | "break", win = Pause, macos = !)]
  Pause,
  #[key("play", win = Play, macos = !)]
  Play,
  #[key("print", win = Print, macos = !)]
  Print,
  #[key("prior", win = Prior, macos = !)]
  Prior,
  #[key("process" | "process key", win = Processkey, macos = !)]
  Processkey,
  #[key("q", win = Q, macos = !)]
  Q,
  #[key("r", win = R, macos = !)]
  R,
  #[key("r button", win = Rbutton, macos = !)]
  RButton,
  #[key("rctrl" | "rcontrol" | "rctrl key" | "r control key" | "right ctrl" | "right control" | "right ctrl key" | "right control key", win = RControl, macos = !)]
  RControl,
  #[key("return" | "enter", win = Return, macos = !)]
  Return,
  #[key("right" | "right arrow", win = Right, macos = !)]
  Right,
  #[key("rmenu" | "ralt" | "right alt" | "right menu", win = RMenu, macos = !)]
  RMenu,
  #[key("rshift" | "right shift" | "rshift key" | "right shift key", win = RShift, macos = !)]
  RShift,
  #[key("rwin" | "right win" | "rwin key" | "right win key", win = RWin, macos = !)]
  RWin,
  #[key("s", win = S, macos = !)]
  S,
  #[key("scroll", win = Scroll, macos = !)]
  Scroll,
  #[key("select", win = Select, macos = !)]
  Select,
  #[key("separator", win = Separator, macos = !)]
  Separator,
  #[key("shift" | "shift key", win = Shift, macos = !)]
  Shift,
  #[key("sleep", win = Sleep, macos = !)]
  Sleep,
  #[key("snapshot", win = Snapshot, macos = !)]
  Snapshot,
  #[key("space", win = Space, macos = !)]
  Space,
  #[key("subtract", win = Subtract, macos = !)]
  Subtract,
  #[key("t", win = T, macos = !)]
  T,
  #[key("tab", win = Tab, macos = !)]
  Tab,
  #[key("u", win = U, macos = !)]
  U,
  #[key("up" | "up arrow", win = Up, macos = !)]
  Up,
  #[key("v", win = V, macos = !)]
  V,
  #[key("volume down", win = VolumeDown, macos = !)]
  VolumeDown,
  #[key("volume mute" | "mute", win = VolumeMute, macos = !)]
  VolumeMute,
  #[key("volume up", win = VolumeUp, macos = !)]
  VolumeUp,
  #[key("w", win = W, macos = !)]
  W,
  #[key("win" | "win key", win = Virt(LWin), macos = !)]
  Win,
  #[key("x", win = X, macos = !)]
  X,
  #[key("x button 1", win = XButton1, macos = !)]
  XButton1,
  #[key("x button 2", win = XButton2, macos = !)]
  XButton2,
  #[key("y", win = Y, macos = !)]
  Y,
  #[key("z", win = Z, macos = !)]
  Z,
  #[key("zoom", win = Zoom, macos = !)]
  Zoom,
  #[key("none", win = None, macos = !)]
  None,

  #[key(..)]
  Custom(u16),
}

impl Key {
  /// Check if the key is analogous to another key.
  /// Returns `true` if the keys are the same or if `other` is a
  /// specialisation of self.
  ///
  /// # Example
  /// ```
  /// assert!(Key::A.is_analogous(Key::A));
  /// assert!(Key::Shift.is_analogous(Key::LShift));
  /// assert!(Key::LControl.is_analogous(Key::RControl) == false);
  /// ```
  pub fn is_analogous(self, other: Key) -> bool {
    #[allow(clippy::match_same_arms)]
    match (self, other) {
      (Key::Shift, Key::LShift | Key::RShift) => true,
      (Key::Control, Key::LControl | Key::RControl) => true,
      (Key::Menu, Key::LMenu | Key::RMenu) => true,
      (Key::Win, Key::LWin | Key::RWin) => true,
      _ => self == other,
    }
  }

  /// Returns whether the key is a generic key, such as `Shift`, `Control`,
  /// `Alt`, or `Win` instead of the more specific versions like
  /// `LShift`, `RShift`, etc.
  pub fn is_generic(self) -> bool {
    matches!(self, Key::Shift | Key::Control | Key::Menu | Key::Win)
  }

  /// Returns a generic version of the key.
  /// Special keys like `LShift`, `RShift`, `LControl`, `RControl` etc.
  /// will return the gereric Shift, Control, Alt, or Win key.
  /// Other keys will return themselves unchanged.
  pub fn get_generic(self) -> Self {
    match self {
      Key::LShift | Key::RShift => Key::Shift,
      Key::LControl | Key::RControl => Key::Control,
      Key::LMenu | Key::RMenu => Key::Menu,
      Key::LWin | Key::RWin => Key::Win,
      _ => self,
    }
  }

  /// Get the specific key(s) for a Key.
  /// Generic keys like `Shift`, `Control`, `Alt`, and `Win` will return
  /// both the left and right versions of the key.
  /// Non-generic keys will return a vector containing just the key itself.
  pub fn get_specifics(self) -> Vec<Key> {
    match self {
      Key::Shift => vec![Key::LShift, Key::RShift],
      Key::Control => vec![Key::LControl, Key::RControl],
      Key::Menu => vec![Key::LMenu, Key::RMenu],
      Key::Win => vec![Key::LWin, Key::RWin],
      _ => vec![self],
    }
  }

  /// Gets whether this key is currently down.
  pub fn is_down(self) -> bool {
    self.get_specifics().iter().any(|key| key.is_down_raw())
  }

  /// Gets whether this key is currently down using the raw key
  /// code.
  pub fn is_down_raw(self) -> bool {
    let vk_code = self.into_vk();
    unsafe { (GetKeyState(vk_code.into()) & 0x80) == 0x80 }
  }
}
