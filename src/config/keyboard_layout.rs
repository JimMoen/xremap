use evdev::KeyCode as Key;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  /// Map the evdev keys to labels on the keyboard.
  ///  - It's the same mapping the OS does to change keyboard layouts. Xremap also
  ///      needs to do it, so config files can use the keyboard layout.
  ///  - It's possible to use pseudo keys (only xremap knows of), because they're reversed before emitting.
  ///  - The evdev keys are defined here: https://github.com/emberian/evdev/blob/master/src/scancodes.rs
  ///     - They can also be obtained by running `RUST_LOG=debug xremap config.yml`
  ///     - They are close to the QWERTY keyboard layout: https://en.wikipedia.org/wiki/QWERTY
  ///  - Keys, that map to the same key on your keyboard, can be omitted.
  pub static ref keyboard_layout_definition: HashMap<u16, &'static str> = HashMap::from([
      //for AZERTY - only partial layout
      // (Key::KEY_Q.code(), "A"),
      // (Key::KEY_A.code(), "Q"),
      // (Key::KEY_W.code(), "Z"),
      // (Key::KEY_Z.code(), "W"),

      //for Dvorak layout

      // Line One (nums)
      (Key::KEY_GRAVE.code(), "`"),
      (Key::KEY_1.code(), "1"),
      (Key::KEY_2.code(), "2"),
      (Key::KEY_3.code(), "3"),
      (Key::KEY_4.code(), "4"),
      (Key::KEY_5.code(), "5"),
      (Key::KEY_6.code(), "6"),
      (Key::KEY_7.code(), "7"),
      (Key::KEY_8.code(), "8"),
      (Key::KEY_9.code(), "9"),
      (Key::KEY_0.code(), "0"),
      (Key::KEY_MINUS.code(), "["),
      (Key::KEY_EQUAL.code(), "]"),

      // Line two
      (Key::KEY_Q.code(), "'"),
      (Key::KEY_W.code(), ","),
      (Key::KEY_E.code(), "."),
      (Key::KEY_R.code(), "p"),
      (Key::KEY_T.code(), "y"),
      (Key::KEY_Y.code(), "f"),
      (Key::KEY_U.code(), "g"),
      (Key::KEY_I.code(), "c"),
      (Key::KEY_O.code(), "r"),
      (Key::KEY_P.code(), "l"),
      (Key::KEY_LEFTBRACE.code(), "/"),
      (Key::KEY_RIGHTBRACE.code(), "="),
      (Key::KEY_BACKSLASH.code(), "\\"),

      // Line three
      (Key::KEY_A.code(), "a"),
      (Key::KEY_S.code(), "o"),
      (Key::KEY_D.code(), "e"),
      (Key::KEY_F.code(), "u"),
      (Key::KEY_G.code(), "i"),
      (Key::KEY_H.code(), "d"),
      (Key::KEY_J.code(), "h"),
      (Key::KEY_K.code(), "t"),
      (Key::KEY_L.code(), "n"),
      (Key::KEY_SEMICOLON.code(), "s"),
      (Key::KEY_APOSTROPHE.code(), "-"),

      // Line four
      (Key::KEY_Z.code(), ";"),
      (Key::KEY_X.code(), "q"),
      (Key::KEY_C.code(), "j"),
      (Key::KEY_V.code(), "k"),
      (Key::KEY_B.code(), "x"),
      (Key::KEY_N.code(), "b"),
      (Key::KEY_M.code(), "m"),
      (Key::KEY_COMMA.code(), "w"),
      (Key::KEY_DOT.code(), "v"),
      (Key::KEY_SLASH.code(), "z"),

      // keep space
      (Key::KEY_SPACE.code(), " "),

  ]);

  // Map from evdev key to pseudo_key_code
  pub static ref keyboard_layout: HashMap<u16, u16> = keyboard_layout_definition.clone().into_iter().map(|(a, _)| (a, 40000 + a)).collect();

  // Map from pseudo_key_code to evdev key
  pub static ref reverse_keyboard_layout: HashMap<u16, u16> = keyboard_layout.iter().map(|(&a, &b)| (b, a)).collect();

  // Map from user-defined key to pseudo_key_code
  pub static ref keyboard_layout_keys: HashMap<String, u16> = keyboard_layout_definition.clone().into_iter().map(|(a, b)| (b.to_ascii_uppercase(), 40000 + a)).collect();

}

///
/// - The key_name is what ever the user wants. It doesn't have to be related to the evdev key names.
///
pub fn apply_keyboard_layout(key_name: &str) -> Option<Key> {
    let key_name = key_name.to_uppercase();

    keyboard_layout_keys
        .get(&format!("{}", key_name))
        .or_else(|| keyboard_layout_keys.get(&format!("KEY_{}", key_name)))
        .map(|&code| Key(code))
}
