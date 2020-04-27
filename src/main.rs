extern crate sdl2;
extern crate lewton;
extern crate rand;

#[macro_use]
mod macros;

mod c_lib;
mod screen;
mod keymap;
mod audio;
mod input;

use std::env;
use std::process::exit;
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;

use sdl2::Sdl;
use sdl2::event::Event;

use input::MouseInput;

thread_local! {
    static SDL_CONTEXT: RefCell<Option<SdlContext>> = RefCell::default();
}

use c_lib::{
    c_Start680x0, c_Init680x0, c_FlagException,
    c_Input_MousePress, c_Input_MouseRelease,
};

pub struct GameConfig {
    use_fullscreen: bool,
    nosound: bool,
    screen_w: u32,
    screen_h: u32
}

impl GameConfig {
    fn new() -> Self {
        Self {
            use_fullscreen: false,
            nosound: false,
            screen_w: 640,
            screen_h: 480,
        }
    }
}

pub struct SdlContext {
    base: Sdl,
    video: sdl2::VideoSubsystem,
    timer: sdl2::TimerSubsystem,
    event_pump: sdl2::EventPump,
    mouse: sdl2::mouse::MouseUtil,
    audio: sdl2::AudioSubsystem,
    event: sdl2::EventSubsystem,
}

impl SdlContext {
    pub fn base(&self) -> &sdl2::Sdl {
        &self.base
    }

    pub fn base_mut(&mut self) -> &mut sdl2::Sdl {
        &mut self.base
    }

    pub fn video(&self) -> &sdl2::VideoSubsystem {
        &self.video
    }

    pub fn timer(&self) -> &sdl2::TimerSubsystem {
        &self.timer
    }

    pub fn event_pump(&mut self) -> &sdl2::EventPump {
        &self.event_pump
    }

    pub fn event_pump_mut(&mut self) -> &mut sdl2::EventPump {
        &mut self.event_pump
    }

    pub fn mouse(&self) -> &sdl2::mouse::MouseUtil {
        &self.mouse
    }

    pub fn audio(&self) -> &sdl2::AudioSubsystem {
        &self.audio
    }

    pub fn event(&self) -> &sdl2::EventSubsystem {
        &self.event
    }

    fn init() -> Self {
        let base = sdl2::init().expect("unable to init SDL");
        let video = base.video().expect("unable to init SDL Video");
        let timer = base.timer().expect("unable to init SDL Timer");
        let event_pump = base.event_pump().expect("unable to init SDL Event Pump");
        let audio = base.audio().expect("unable to init SDL Audio");
        let mouse = base.mouse();
        let event = base.event().expect("unable to init SDL event");

        Self { base, video, timer, event_pump, mouse, audio, event }
    }
}

fn read_parameters(args: Vec<String>) -> GameConfig {
    let mut config = GameConfig::new();
    let mut args = args.into_iter();

    // skip first arg, it's our binary name
    args.next();

    while let Some(arg) = args.next() {
        if arg.is_empty() {
            continue;
        }

        match &*arg {
            "--help" | "-h" =>  {
                println!("Usage:\n frontier [options]\n
                       Where options are:\n
                          --help or -h          Print this help text and exit.\n
                          --fullscreen or -f    Try to use fullscreen mode.\n
                          --nosound             Disable sound (faster!).\n
                          --size w            Start at specified window size.\n"
                      );

                exit(0);
            }

            "--fullscreen" | "-f" => {
                config.use_fullscreen = true;
            }

            "--nosound" => {
                config.nosound = true;
            }

            "--size" => {
                if let Some(value) = args.next() {
                    config.screen_w = u32::from_str_radix(&value, 10).expect("size musst be a number!");
                }
            }

            _ => println!("Illegal parameter: {}", arg)
        }
    };

    config
}

fn init(context: &mut SdlContext, config: &GameConfig) {
    screen::init(context, config);

    // Init CPU emulation
    c_Init680x0();
    audio::init(context, config);
    keymap::init(context);
}

fn main() {
    let args = env::args();

    /* Check for any passed parameters */
    let config = read_parameters(args.collect());
    let mut sdl_context = SdlContext::init();

    init(&mut sdl_context, &config);

    // let _timer = sdl_context.timer().add_timer(20, Box::new(vbl_callback));

    SDL_CONTEXT.with(|ref_cell| {
        *ref_cell.borrow_mut() = Some(sdl_context);
    });

    /* Run emulation */
    println!("starting 680x0...");
    c_Start680x0();
}

pub fn event_handler() {
    with_static_ref_option!([SDL_CONTEXT => sdl_context] {
        while let Some(event) = sdl_context.event_pump_mut().poll_event() {
            println!("handle sdl event {:?}", event);

            match event {
                Event::Quit { .. } => {
                    println!("trying to quit application!");
                    exit(0);
                },

                // Read/Update internal mouse position
                Event::MouseMotion { xrel, yrel, x, y, .. } => {
                    let mouse_input = MouseInput {
                        motion_x: xrel as isize,
                        motion_y: yrel as isize,
                        abs_x: x as usize,
                        abs_y: y as usize,
                    };

                    input::set_mouse(mouse_input);
                },

                Event::MouseButtonDown { mouse_btn, .. } => {
                    c_Input_MousePress(mouse_btn as i32);
                },

                Event::MouseButtonUp { mouse_btn, .. } => {
                    c_Input_MouseRelease(mouse_btn as i32);
                },

                Event::KeyDown { keycode, scancode, keymod, repeat, .. } => {
                    if keycode.is_none() || repeat {
                        continue;
                    }

                    keymap::key_down(keycode.unwrap(), scancode.unwrap(), keymod)
                },

                Event::KeyUp { keycode, scancode, keymod, .. } => {
                    if keycode.is_none() {
                        continue;
                    }

                    keymap::key_up(keycode.unwrap(), scancode.unwrap(), keymod);
                },

                _ => {
                    println!("ignoring SDL event {:?}", event);
                }
            }
        }
    } or {
        println!("Main: SDL context not available yet!");
    });
}

pub fn idle() {
    c_FlagException(0);

    sleep(Duration::from_millis(20));
}
