use bitmap_font::{tamzen::FONT_5x9, TextStyle};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};

mod display;

use display::Display;

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

    test: String,
}

impl Os {
    const fn new() -> Self {
        Self {
            input: 0, // No keys pressed
            total_time: 0f32,
            state: State::Init,

            display: Display::new(),

            test: String::new(),
        }
    }

    fn initialize(&mut self) {
        self.test = std::fs::read_to_string("disk/hello.txt").unwrap().trim().to_string();
        // self.test = "shit".to_owned();
        self.state = State::Splashscreen;
    }

    fn update_input(&mut self, input: u64) {
        self.input = input;
    }

    fn update(&mut self, delta_s: f32) {
        self.total_time += delta_s;

        match self.state {
            State::Init => self.initialize(),
            _ => {},
        }
    }

    fn draw(&mut self, info: FrameInfo) {
        match self.state {
            State::Splashscreen => {
                self.display.clear_black();
                let cur_dir: String = std::env::current_dir().ok().map(|p| p.display().to_string()).unwrap_or(String::from("FAILED"));
                let t = format!("time: {}\ncur_dir: {}\ndisk/hello.txt: {}", self.total_time, cur_dir, self.test);
                let text = Text::new(&t, Point::zero(), TextStyle::new(&FONT_5x9, BinaryColor::On));
                text.draw(&mut self.display.color_converted()).expect("Failed to draw text!");
            },
            _ => {},
        }
        self.display.flush(info.buf);
    }
}

static mut OS: Os = Os::new();

#[no_mangle]
pub extern "C" fn tick(mem_addr: u32, input: u64, delta_s: f32) {
    unsafe {
        OS.update_input(input);
        OS.update(delta_s);
        OS.draw(FrameInfo::new(mem_addr));
    }
}
