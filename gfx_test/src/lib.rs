#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

use poslib::{BUFFER_WIDTH, BUFFER_HEIGHT, FrameInfo, Display};

lazy_static! {
    pub static ref FRAME_INFO: Mutex<FrameInfo<'static>> = Mutex::new(FrameInfo::new(0x80));
}

pub struct App {
    total_time: f32,
    display: Display,
}

impl App {
    const fn new() -> Self {
        Self {
            total_time: 0f32,
            display: Display::new(),
        }
    }

    fn draw(&mut self) {
        self.display.clear_black();
        self.display.with_func(|x,y| {
            ((x + y) % 29) as u8
        });
        if let Ok(mut lock) = FRAME_INFO.lock() {
            self.display.flush(lock.buf);
        }
    }
}

pub static mut APP: App = App::new();

#[no_mangle]
pub extern "C" fn tick(input: u64, delta_s: f32) -> u32 {
    unsafe {
        APP.total_time += delta_s;
        APP.draw();
    }
    0
}
