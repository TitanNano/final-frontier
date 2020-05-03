use std::process::exit;

use sdl2::keyboard::Keycode;

use crate::SdlContext;
use crate::screen;
use crate::c_lib::{ c_Call_DumpDebug };

pub struct ShortcutKey {
    shift_pressed: bool,
    key: Keycode,
    ctrl_pressed: bool,
}

impl ShortcutKey {
    pub fn set_ctrl_pressed(&mut self, value: bool) {
        self.ctrl_pressed = value;
    }

    pub fn set_shift_pressed(&mut self, value: bool) {
        self.shift_pressed = value;
    }
}

impl From<Keycode> for ShortcutKey {
    fn from(key: Keycode) -> Self {
        Self {
            key,
            shift_pressed: false,
            ctrl_pressed: false,
        }
    }
}

pub fn keychecks(key: ShortcutKey, context: &mut SdlContext) {
    /* Check for supported keys: */
    match key.key {
       Keycode::F11 => screen::toggle_fullscreen(), // Switch between fullscreen/windowed mode
       Keycode::M => mouse_mode(context),                  // Toggle mouse mode
       Keycode::Q => exit(0),                       // Quit program
       Keycode::D => c_Call_DumpDebug(),
       Keycode::E => screen::toggle_renderer(),
       _ => {}
    }
}

fn mouse_mode(context: &mut SdlContext) {
    context.mouse
        .set_relative_mouse_mode(!context.mouse.relative_mouse_mode());
}
