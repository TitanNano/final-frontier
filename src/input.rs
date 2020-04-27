use crate::c_lib;

pub struct MouseInput {
    pub motion_x: isize,
    pub motion_y: isize,
    pub abs_x: usize,
    pub abs_y: usize,
}

pub fn set_mouse(state: MouseInput) {
    c_lib::update_mouse_input(state.motion_x, state.motion_y, state.abs_x, state.abs_y);
}
