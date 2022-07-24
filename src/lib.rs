#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

mod display;
pub mod console;
mod assets;
pub mod input;
mod cli;
// mod runtime;

use display::Display;
use console::Console;
use cli::Cli;

const BUFFER_WIDTH: usize = 336;
const BUFFER_HEIGHT: usize = 144;
const BUFFER_LEN: usize = BUFFER_WIDTH * BUFFER_HEIGHT * 4;

#[derive(Default)]
pub struct FrameInfo<'frame> {
    buf: &'frame mut [u8],
}

impl FrameInfo<'_> {
    pub fn new(mem_addr: u32) -> Self {
        Self {
            buf: unsafe { std::slice::from_raw_parts_mut(mem_addr as *mut u8, 336*144*4) },
        }
    }
}

lazy_static! {
    pub static ref FRAME_INFO: Mutex<FrameInfo<'static>> = Mutex::new(FrameInfo::new(0x80));
}

enum State {
    Init,
    Splashscreen,
    CommandLineInterface,
}

struct Os {
    input: u64,
    total_time: f32,
    state: State,

    pub display: Display,
    pub console: Console,
    pub cli: Cli,
}

impl Os {
    const fn new() -> Self {
        Self {
            input: 0, // No keys pressed
            total_time: 0f32,
            state: State::Init,

            display: Display::new(),
            console: Console::new(),
            cli: Cli::new(),
        }
    }

    fn initialize(&mut self) {
        self.state = State::CommandLineInterface;
    }

    fn update_input(&mut self, input: u64) {
        self.input = input;
        self.cli.input(self.input);
    }

    fn update(&mut self, delta_s: f32) {
        self.total_time += delta_s;

        match self.state {
            State::Init => self.initialize(),
            State::Splashscreen => if self.total_time > 4.5 {
                self.state = State::CommandLineInterface;
            },
            _ => {},
        }
    }

    fn draw(&mut self) {
        match self.state {
            State::Splashscreen =>  {
                self.display.clear_black();

                // A poor mans fade
                self.display.draw_image(0,0, &assets::SPLASHSCREEN_TGA);
                let mut fade = self.total_time.max(0.0).min(1.0);
                if self.total_time > 3.8 {
                    fade = 0.0;
                } else if self.total_time > 2.8 {
                    fade = 1.0 - (self.total_time - 2.8).max(0.0).min(1.0);
                }
                for x in 0..336 {
                    for y in 0..144 {
                        let c = self.display.get(x,y);
                        let c = [c[0], c[1], c[2], c[3]];
                        let _ = self.display.set(x,y, [(c[0] as f32 * fade) as u8, (c[1] as f32 * fade) as u8, (c[2] as f32 * fade) as u8, (c[3] as f32 * fade) as u8]);
                    }
                }
            },
            State::CommandLineInterface => {
                self.display.clear_black();
                self.console.clear();
                self.console.print(format!("raw:\n{:#064b}", self.input));
                self.console.print(format!("buttons:\n{:?}", input::input_to_vec(self.input)));
                self.console.print(format!("\ninput: {}", self.cli.input_buf));
                self.console.flush_to_display(&mut self.display);
            }
            _ => {},
        }
        if let Ok(mut lock) = FRAME_INFO.lock() {//.expect("Failed to get FRAME_INFO lock!");
            self.display.flush(lock.buf);
        }
    }
}

static mut OS: Os = Os::new();

#[no_mangle]
pub extern "C" fn tick(input: u64, delta_s: f32) {
    let prog = poslib::run_program("edit.wasm").expect("Failed to load code!");
    panic!("poslib!!! prog id: {}", prog.id);

    unsafe {
        OS.update_input(input);
        OS.update(delta_s);
        OS.draw();
    }
}
