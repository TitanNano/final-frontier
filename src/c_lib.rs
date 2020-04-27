#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;
use std::convert::TryInto;
use ::core::mem;

use sdl2::video::Window;

use crate::audio;
use crate::screen;

include!("bindings.rs");

type FnPointer = unsafe extern "C" fn() -> c_void;

pub fn c_Start680x0() {
    unsafe {
        Start680x0();
    }
}

pub fn c_Init680x0() {
    unsafe {
        Init680x0();
    }
}

pub fn glu_init() {
    unsafe {
        qobj = gluNewQuadric();

        tobj = gluNewTess();

        let vertexCallback_pointer: unsafe extern "C" fn(*mut c_void, *mut c_void) = vertexCallback;
        let vertesCallback_casted_pointer = mem::transmute::<_, FnPointer>(vertexCallback_pointer);

        let beginCallback_pointer: unsafe extern "C" fn(u32) = beginCallback;
        let beginCallback_casted_pointer = mem::transmute::<_, FnPointer>(beginCallback_pointer);

        let endCallback_pointer: unsafe extern "C" fn() = endCallback;
        let endCallback_casted_pointer = mem::transmute::<_, FnPointer>(endCallback_pointer);

        let errorCallback_pointer: unsafe extern "C" fn(u32) = errorCallback;
        let errorCallback_casted_pointer = mem::transmute::<_, FnPointer>(errorCallback_pointer);

        let combineCallback_pointer: unsafe extern "C" fn(*mut f64, *mut *mut f64, *mut f32, *mut *mut f64) = combineCallback;
        let combineCallback_casted_pointer = mem::transmute::<_, FnPointer>(combineCallback_pointer);

        gluTessCallback(tobj, GLU_TESS_VERTEX_DATA, Some(vertesCallback_casted_pointer));
        gluTessCallback(tobj, GLU_TESS_BEGIN,  Some(beginCallback_casted_pointer));
        gluTessCallback(tobj, GLU_TESS_END,  Some(endCallback_casted_pointer));
        gluTessCallback(tobj, GLU_TESS_ERROR, Some(errorCallback_casted_pointer));
        gluTessCallback(tobj, GLU_TESS_COMBINE, Some(combineCallback_casted_pointer));
    }
}

pub fn init_viewport_gl() {
    let SCR_TEX_W =	512;
    let SCR_TEX_H = 256;

    unsafe {
        glDisable (GL_CULL_FACE);
        glShadeModel (GL_FLAT);
        glDisable (GL_DEPTH_TEST);
        glClearColor (0f32, 0f32, 0f32, 0f32);

        glMatrixMode (GL_PROJECTION);
        glLoadIdentity ();

        /* aspect ratio of frontier's 3d view is 320/168 = 1.90 */
        gluPerspective (36.5, 1.9, 1.0, 10_000_000_000.0);

        let screen_tex_mut_ptr: *mut u32 = &mut screen_tex;
        let tex_pixels: *const u32 = &0;
        let tex_pixels_cvoid = tex_pixels as *const c_void;


        glEnable (GL_TEXTURE_2D);
        glGenTextures (1, screen_tex_mut_ptr);
        glBindTexture (GL_TEXTURE_2D, screen_tex);
        glTexImage2D (GL_TEXTURE_2D, 0, GL_RGBA as i32, SCR_TEX_W, SCR_TEX_H, 0, GL_RGBA, GL_INT, tex_pixels_cvoid);
        glTexParameterf (GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_LINEAR as f32);
        glTexParameterf (GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_LINEAR as f32);
        glTexParameterf (GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as f32);
        glTexParameterf (GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as f32);

        glBlendFunc (GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        glDisable (GL_TEXTURE_2D);

        glClear (GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode (GL_MODELVIEW);
        glLoadIdentity ();
        glDisable (GL_DEPTH_TEST);
    }
}

pub fn c_FlagException(num: i32) {
    unsafe {
        FlagException(num)
    }
}

pub fn c_Input_MousePress(button: i32) {
    unsafe {
        Input_MousePress(button);
    }
}

pub fn c_Input_MouseRelease(button: i32) {
    unsafe {
        Input_MouseRelease(button);
    }
}

pub fn c_Input_PressSTKey(st_scancode: usize, press: bool) {
    unsafe {
        Input_PressSTKey(st_scancode as u8, if press { TRUE as i32 } else { FALSE as i32 });
    }
}

pub fn update_mouse_input(motion_x: isize, motion_y: isize, abs_x: usize, abs_y: usize) {
    unsafe {
        input.motion_x += motion_x as i32;
        input.motion_y += motion_y as i32;
        input.abs_x = abs_x as i32;
        input.abs_y = abs_y as i32;
    }
}

pub fn unsafe_nu_draw_screen(window: &Window) {
    /* build RGB palettes */
    let (main_rgb_palette, main_palette, main_palette_len) = unsafe {
        (&mut MainRGBPalette, &MainPalette, len_main_palette as usize)
    };

    let (ctrl_rgb_palette, ctrl_palette) = unsafe {
        (&mut CtrlRGBPalette, &CtrlPalette)
    };

    screen::build_rgb_palette(main_rgb_palette, main_palette, main_palette_len);
    screen::build_rgb_palette(ctrl_rgb_palette, ctrl_palette, 16);

    unsafe {
        //fprintf (stderr, "Render: ");
        if !znode_cur.is_null() {
            end_node();
        }

        //printf ("Frame: %d znodes.\n", znode_buf_pos);
        draw_3dview(znode_start);
        //fprintf (stderr, "\n");
        //
        // if (mouse_shown) {
        //     SDL_ShowCursor (SDL_ENABLE);
        //     mouse_shown = 0;
        // } else {
        //     SDL_ShowCursor (SDL_DISABLE);
        // }

        draw_control_panel();
        glFlush();
    }

    window.gl_swap_window();

    /* frontier background color... */
    unsafe {
        if use_renderer == RENDERERS_R_GLWIRE {
            glClearColor (0.0, 0.0, 0.0, 0.0);
        } else {
            set_gl_clear_col(MainRGBPalette[fe2_bgcol as usize] as i32);
        }

        glMatrixMode(GL_MODELVIEW);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glLoadIdentity();
        set_main_viewport();
    }
}

// Main module C interface

#[no_mangle]
extern "C" fn Main_EventHandler() {
    crate::event_handler()
}

#[no_mangle]
extern "C" fn Call_Idle() {
	crate::idle()
}

// Audio module C interface

#[no_mangle]
extern "C" fn Call_PlaySFX() {
    let (sample, channel) = unsafe {
    	(GetReg(REG_D0 as i32), GetReg(REG_D1 as i32))
    };

    audio::play_sfx(sample.try_into().unwrap(), channel.try_into().unwrap())
}

#[no_mangle]
extern "C" fn Call_PlayMusic() {
    println!("entered play music handler!");

    let (reg0, reg1, reg2) = unsafe {
        (GetReg(0) as u32, GetReg(1) as u32, GetReg(2) as u32)
    };

    /* Playing mode in d0:
     * -2 = play random track once
     * -1 = play random tracks continuously
     * 0+ = play specific track once
     * d1:d2 is a mask of enabled tracks
     */
    let music_mode: isize = reg0 as isize;

    let mut enabled_tracks: usize = 0;

    if reg1 & 0xff00_0000 != 0 {
        enabled_tracks |= 0x1;
    }

    if reg1 & 0x00ff_0000 != 0 {
        enabled_tracks |= 0x2;
    }

    if reg1 & 0x0000_ff00 != 0 {
        enabled_tracks |= 0x4;
    }

    if reg1 & 0x0000_00ff != 0 {
        enabled_tracks |= 0x8;
    }

    if reg2 & 0xff00_0000 != 0 {
        enabled_tracks |= 0x10;
    }

    if reg2 & 0x00ff_0000 != 0 {
        enabled_tracks |= 0x20;
    }

    if reg2 & 0x0000_ff00 != 0 {
        enabled_tracks |= 0x40;
    }

    if reg2 & 0x0000_00ff != 0 {
        enabled_tracks |= 0x80;
    }

    audio::play_music(music_mode, enabled_tracks);

    println!("left play music handler!");
}

#[no_mangle]
extern "C" fn Call_StopMusic() {
    println!("entered stop music handler!");

    audio::stop_music();

    println!("left stop music handler!");
}

#[no_mangle]
extern "C" fn Call_IsMusicPlaying() {
    let is_music_playing = audio::is_music_playing();

    unsafe {
        SetReg(0, if is_music_playing { 1 } else { 0 });
    }
}


// Input module C interface
// #[no_mangle]
// extern "C" fn Call_GetMouseInput() {
//
// }
//
// #[no_mangle]
// extern "C" fn Call_GetKeyboardEvent() {
//
// }


// Screen module C interface
#[no_mangle]
extern "C" fn Nu_DrawScreen() {
    screen::nu_draw_screen();
}
