use sdl2::keyboard;
use sdl2::keyboard::{ Keycode, Scancode };

use crate::SdlContext;
use crate::c_lib::{ c_Input_PressSTKey };

pub fn init(_context: &SdlContext) {}

struct ShortCutKey {
    shift_pressed: bool,
    key: Keycode,
    ctrl_pressed: bool,
}

impl ShortCutKey {
    fn new() -> Self {
        Self {
            shift_pressed: false,
            key: Keycode::Escape,
            ctrl_pressed: false,
        }
    }
}

/*-----------------------------------------------------------------------*/
/*
User press key down
*/
pub fn key_down(keycode: Keycode, scancode: Scancode, keymod: sdl2::keyboard::Mod) {
    // BOOL bPreviousKeyState;
    // char STScanCode;
    // int symkey = sdlkey->sym; // keycode

    let mut shortcut_key = ShortCutKey::new();

    println!("keydown: sym={} scan={} mod=${:x}", keycode, scancode, keymod);

    /* Handle special keys */
    match keycode {
        Keycode::Mode | Keycode::LGui | Keycode::NumLockClear => {
            /* Ignore modifier keys that aren't passed to the ST */
            return;
        }

        Keycode::F11 | Keycode::F12 => {
            shortcut_key.key = keycode;
            return;
        },

        _ => {}
    };

    /* If pressed short-cut key, retain keypress until safe to execute (start of VBL) */
    if keymod.contains(keyboard::Mod::MODEMOD) || keymod.contains(keyboard::Mod::RGUIMOD) || keymod.intersects(keyboard::Mod::LCTRLMOD|keyboard::Mod::RCTRLMOD) {
        // ShortCutKey.Key = symkey;

        if keymod.intersects(keyboard::Mod::LCTRLMOD|keyboard::Mod::RCTRLMOD) {
            shortcut_key.ctrl_pressed = true;
        }

        if keymod.intersects(keyboard::Mod::LSHIFTMOD|keyboard::Mod::RSHIFTMOD) {
            shortcut_key.shift_pressed = true;
        }

        return;
    }

    let st_scancode = remap_key_to_st_scancode(keycode, keymod);

    if st_scancode.is_none() {
        return;
    }

    c_Input_PressSTKey(st_scancode.unwrap(), true);
}


/*-----------------------------------------------------------------------*/
/*
User released key
*/
pub fn key_up(keycode: Keycode, scancode: Scancode, keymod: keyboard::Mod) {
    println!("keyup: sym={} scan={} mod=${:x}", keycode, scancode, keymod);

    /* Handle special keys */
    match keycode {
        Keycode::Mode | Keycode::LGui | Keycode::NumLockClear => {
            /* Ignore modifier keys that aren't passed to the ST */
        },

        Keycode::F11 | Keycode::F12 => {
        },

        Keycode::CapsLock => {
            /* Simulate another capslock key press */
            c_Input_PressSTKey(0x3A, true);
        },

        _ => {
            let st_scancode = remap_key_to_st_scancode(keycode, keymod);

            if st_scancode.is_none() {
                return;
            }

            c_Input_PressSTKey(st_scancode.unwrap(), false);
        }
    };
}

/*-----------------------------------------------------------------------*/
/*
  Remap SDL Key to ST Scan code
*/
fn remap_key_to_st_scancode(keycode: Keycode, keymod: keyboard::Mod) -> Option<usize> {

    /* Check for keypad first so we can handle numlock */
    if keycode as usize >= Keycode::Kp0 as usize && keycode as usize <= Keycode::Kp9 as usize {
        return Some(get_keypad_scancode(keycode, keymod));
    }

    pc_to_st_scancode(keycode)
}

/*-----------------------------------------------------------------------*/
/*
  Map PC scancode to ST scancode.
*/
fn pc_to_st_scancode(keycode: Keycode) -> Option<usize> {
    match keycode {
        /* Numeric Pad */
        /* note that the numbers are handled in Keymap_GetKeyPadScanCode()! */
        Keycode::KpDivide => Some(0x65),  /* Numpad / */
        Keycode::KpMultiply => Some(0x66),  /* NumPad * */
        Keycode::KpMinus => Some(0x4a),  /* NumPad - */
        Keycode::KpPlus => Some(0x4e),  /* NumPad + */
        Keycode::KpPeriod => Some(0x71),  /* NumPad . */
        Keycode::KpEnter => Some(0x72),  /* NumPad Enter */

        /* Special Keys */
        Keycode::Home => Some(0x47),  /* Home */
        Keycode::End => Some(0x60),  /* End => "<>" on German Atari kbd */
        Keycode::Up => Some(0x48),  /* Arrow Up */
        Keycode::Left => Some(0x4B),  /* Arrow Left */
        Keycode::Right => Some(0x4D),  /* Arrow Right */
        Keycode::Down => Some(0x50),  /* Arrow Down */
        Keycode::Insert => Some(0x52),  /* Insert */
        Keycode::Delete => Some(0x53), /* Delete */
        Keycode::Less => Some(0x60),  /* "<" */

        Keycode::Escape => Some(0x01),
        Keycode::RCtrl => Some(0x1D),  /* Control */
        Keycode::RAlt => Some(0x38),  /* Alternate */
        Keycode::Num1 => Some(0x02),
        Keycode::Num2 => Some(0x03),
        Keycode::Num3 => Some(0x04),
        Keycode::Num4 => Some(0x05),
        Keycode::Num5 => Some(0x06),
        Keycode::Num6 => Some(0x07),
        Keycode::Num7 => Some(0x08),
        Keycode::Num8 => Some(0x09),
        Keycode::Num9 => Some(0x0A),
        Keycode::Num0 => Some(0x0B),
        Keycode::Backspace => Some(0x0E),
        Keycode::Tab => Some(0x0F),
        Keycode::Return => Some(0x1C),
        Keycode::Space => Some(0x39),
        Keycode::Q => Some(0x10),
        Keycode::W => Some(0x11),
        Keycode::E => Some(0x12),
        Keycode::R => Some(0x13),
        Keycode::T => Some(0x14),
        Keycode::Y => Some(0x15),
        Keycode::U => Some(0x16),
        Keycode::I => Some(0x17),
        Keycode::O => Some(0x18),
        Keycode::P => Some(0x19),
        Keycode::A => Some(0x1E),
        Keycode::S => Some(0x1F),
        Keycode::D => Some(0x20),
        Keycode::F => Some(0x21),
        Keycode::G => Some(0x22),
        Keycode::H => Some(0x23),
        Keycode::J => Some(0x24),
        Keycode::K => Some(0x25),
        Keycode::L => Some(0x26),
        Keycode::Z => Some(0x2C),
        Keycode::X => Some(0x2D),
        Keycode::C => Some(0x2E),
        Keycode::V => Some(0x2F),
        Keycode::B => Some(0x30),
        Keycode::N => Some(0x31),
        Keycode::M => Some(0x32),
        Keycode::CapsLock => Some(0x3A),
        Keycode::LShift => Some(0x2A),
        Keycode::LCtrl => Some(0x1D),
        Keycode::LAlt => Some(0x38),
        Keycode::F1 => Some(0x3B),
        Keycode::F2 => Some(0x3C),
        Keycode::F3 => Some(0x3D),
        Keycode::F4 => Some(0x3E),
        Keycode::F5 => Some(0x3F),
        Keycode::F6 => Some(0x40),
        Keycode::F7 => Some(0x41),
        Keycode::F8 => Some(0x42),
        Keycode::F9 => Some(0x43),
        Keycode::F10 => Some(0x44),

        _ => {
            println!("Keymap: received dead keycode {:?}", keycode);

            None
        }
    }
}

fn get_keypad_scancode(keycode: Keycode, keymod: keyboard::Mod) -> usize {
    match keymod {
        keyboard::Mod::NUMMOD => {
            match keycode {
                Keycode::Kp0 => 0x70,  /* NumPad 0 */
                Keycode::Kp1 => 0x6d,  /* NumPad 1 */
                Keycode::Kp2 => 0x6e,  /* NumPad 2 */
                Keycode::Kp3 => 0x6f,  /* NumPad 3 */
                Keycode::Kp4 => 0x6a,  /* NumPad 4 */
                Keycode::Kp5 => 0x6b,  /* NumPad 5 */
                Keycode::Kp6 => 0x6c,  /* NumPad 6 */
                Keycode::Kp7 => 0x67,  /* NumPad 7 */
                Keycode::Kp8 => 0x68,  /* NumPad 8 */
                Keycode::Kp9 => 0x69,  /* NumPad 9 */

                _ => unreachable!("invalid keypad keycode {:?}", keycode),
            }
        },

        _ => {
            match keycode {
                Keycode::Kp0 => 0x70,  /* NumPad 0 */
                Keycode::Kp1 => 0x6d,  /* NumPad 1 */
                Keycode::Kp2 => 0x50,  /* Cursor down */
                Keycode::Kp3 => 0x6f,  /* NumPad 3 */
                Keycode::Kp4 => 0x4b,  /* Cursor left */
                Keycode::Kp5 => 0x50,  /* Cursor down (again?) */
                Keycode::Kp6 => 0x4d,  /* Cursor right */
                Keycode::Kp7 => 0x52,  /* Insert - good for Dungeon Master */
                Keycode::Kp8 => 0x48,  /* Cursor up */
                Keycode::Kp9 => 0x47,  /* Home - again for Dungeon Master */

                _ => {
                    println!("unexpected keypad keycode {:?}", keycode);
                    0x0
                },
            }
        }
    }
}
