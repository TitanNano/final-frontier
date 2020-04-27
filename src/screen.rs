use std::cell::RefCell;

use sdl2::event::EventType;
use sdl2::video::GLContext;
use sdl2::video::Window;

use crate::{ SdlContext, GameConfig };
use crate::c_lib::{ glu_init, init_viewport_gl, unsafe_nu_draw_screen };

thread_local! {
    static SCREEN_CONTEXT: RefCell<Option<ScreenContext>> = RefCell::default();
}

struct ScreenContext {
    gl_context: GLContext,
    window: Window,
}

pub fn init_viewport(context: &SdlContext, config: &GameConfig) {
    let sdl_gl_attr = context.video().gl_attr();

    sdl_gl_attr.set_double_buffer(true);

    let mut sdl_window_builder = context.video().window("Frontier", config.screen_w, config.screen_h);

    sdl_window_builder.position_centered()
        .opengl();

    if config.use_fullscreen {
        sdl_window_builder.fullscreen();
    }

    let window = sdl_window_builder.build().expect("unable to create SDL window");
    let gl_context = window.gl_create_context().expect("Screen");

    init_viewport_gl();

    let context = ScreenContext { gl_context, window };

    SCREEN_CONTEXT.with(|ref_cell| {
        *ref_cell.borrow_mut() = Some(context);
    });
}

pub fn init(context: &mut SdlContext, config: &GameConfig) {
	init_viewport(context, config);

    glu_init();

	/* Configure some SDL stuff: */
    context.event_pump_mut().enable_event(EventType::MouseMotion);
    context.event_pump_mut().enable_event(EventType::MouseButtonDown);
    context.event_pump_mut().enable_event(EventType::MouseButtonUp);
    context.mouse().show_cursor(true);
}

pub fn nu_draw_screen() {
    with_static_ref_option! {
        let context = { SCREEN_CONTEXT } or { println!("no screen context available yet!"); };

        unsafe_nu_draw_screen(&context.window)
    }
}

pub fn build_rgb_palette(rgb_palette: &mut [u32], st_palette: &[u16], st_palette_len: usize) {
	for  i in 0..st_palette_len {
		let st_col = st_palette[i];

		let b = (st_col as u32 & 0xf) << 4;
		let g = st_col as u32 & 0xf0;
		let r = (st_col as u32 & 0xf00) >> 4;

		rgb_palette[i] = 0xff00_0000 | (b<<16) | (g<<8) | (r);
	}
}
