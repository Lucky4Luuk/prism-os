#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

use bitmap_font::{tamzen::FONT_5x9, TextStyle};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};

mod display;
mod console;
mod assets;

use display::Display;
use console::Console;

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

    tick_timer: f32,
    const_tick_counter: usize,
    const_tick_rate: usize, // In hz

    pub display: Display,
    pub console: Console,

    test: String,
}

impl Os {
    const fn new() -> Self {
        Self {
            input: 0, // No keys pressed
            total_time: 0f32,
            state: State::Init,

            tick_timer: 0.0,
            const_tick_counter: 0,
            const_tick_rate: 2,

            display: Display::new(),
            console: Console::new(),

            test: String::new(),
        }
    }

    fn initialize(&mut self) {
        self.test = std::fs::read_to_string("disk/hello.txt").unwrap().trim().to_string();
        self.state = State::Splashscreen;
    }

    fn update_input(&mut self, input: u64) {
        self.input = input;
    }

    fn update(&mut self, delta_s: f32) {
        self.total_time += delta_s;
        self.tick_timer += delta_s;

        while self.tick_timer > 1f32 / self.const_tick_rate as f32 {
            self.const_tick_counter += 1;
            self.fixed_update();
            self.tick_timer -= 1f32 / self.const_tick_rate as f32;
        }

        match self.state {
            State::Init => self.initialize(),
            _ => {},
        }
    }

    fn fixed_update(&mut self) {
        match self.state {
            State::Splashscreen => {
                if self.total_time < 3.8 {
                    self.const_tick_counter = 0;
                } else {
                    const STARTUP_LINES: [&'static str; 3] = [
                        "Loading hyper-assets...",
                        "Generating quantum registers...",
                        "Quantifying the prism...",
                    ];
                    let i = self.const_tick_counter - 1;
                    if i < STARTUP_LINES.len() {
                        self.console.print(STARTUP_LINES[i]);
                    } else if self.const_tick_counter > STARTUP_LINES.len() { // If-statement only here to delay by 1 extra second
                        self.state = State::CommandLineInterface;
                        self.const_tick_rate = 30;
                    }
                }
            }
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

                self.console.flush_to_display(&mut self.display);
            },
            State::CommandLineInterface => {
                self.display.clear_black();
                let cur_dir: String = std::env::current_dir().ok().map(|p| p.display().to_string()).unwrap_or(String::from("FAILED"));
                let t = format!("time: {}\ncur_dir: {}\ndisk/hello.txt: {}", self.total_time, cur_dir, self.test);
                let text = Text::new(&t, Point::zero(), TextStyle::new(&FONT_5x9, BinaryColor::On));
                text.draw(&mut self.display.color_converted()).expect("Failed to draw text!");
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
    unsafe {
        OS.update_input(input);
        OS.update(delta_s);
        OS.draw();
    }
}
