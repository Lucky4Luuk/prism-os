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

            display: Display::new(),
            console: Console::new(),

            test: String::new(),
        }
    }

    fn initialize(&mut self) {
        // self.test = std::fs::read_to_string("disk/hello.txt").unwrap().trim().to_string();
        self.test = "shit".to_string();
        self.state = State::Splashscreen;
    }

    fn update_input(&mut self, input: u64) {
        self.input = input;
    }

    fn update(&mut self, delta_s: f32) {
        self.total_time += delta_s;

        match self.state {
            State::Init => self.initialize(),
            // State::Splashscreen => if self.total_time > 24.0 {
            //     self.state = State::CommandLineInterface;
            // },
            _ => {},
        }
    }

    fn draw(&mut self) {
        match self.state {
            State::Splashscreen =>  {
                self.display.clear_black();

                self.console.print(format!("t: {}", self.total_time));
                self.console.flush_to_display(&mut self.display);

                // let y = (144.0 - (self.total_time * 72.0).max(0.0).min(144.0)) as usize;
                // let y = y - (((self.total_time - 3.0) * 72.0).max(0.0).min(144.0)) as usize;
                // self.display.draw_image(0,y, &assets::SPLASHSCREEN_TGA);
                //
                // let text_y = 144 - ((self.total_time - 5.0) * 144.0 + 144.0).min(144.0) as usize;
                // let text_y = (text_y as f32 / 10.0) as usize * 10 + 10;
                //
                // let t = "Loading resources...\nSetting up CPU registers...";
                // let text = Text::new(&t, Point::new(0, text_y as i32), TextStyle::new(&FONT_5x9, BinaryColor::On));
                // let _ = text.draw(&mut self.display.color_converted());

                // A poor mans fade
                // let mut fade = self.total_time.max(0.0).min(1.0);
                // if self.total_time > 2.8 {
                //     fade = 1.0 - (self.total_time - 2.8).max(0.0).min(1.0);
                // }
                // for x in 0..336 {
                //     for y in 0..144 {
                //         let c = self.display.get(x,y);
                //         let c = [c[0], c[1], c[2], c[3]];
                //         let _ = self.display.set(x,y, [(c[0] as f32 * fade) as u8, (c[1] as f32 * fade) as u8, (c[2] as f32 * fade) as u8, (c[3] as f32 * fade) as u8]);
                //     }
                // }
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
