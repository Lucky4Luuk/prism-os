#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

use poslib::*;

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
        // self.display.with_func(|x,y| {
        //     let r = (x * 255 / 336) as u8;
        //     let g = (y * 255 / 144) as u8;
        //     let b = 0;
        //     let (pal, _) = poslib::find_palette(r,g,b);
        //     pal
        // });
        self.display.with_func(|x,y| {
            let size = 24;
            let mut i = ((x / size) + (y / size) * (BUFFER_WIDTH / size));
            if i >= PALETTE_SIZE {
                i = 0;
            }
            i as u8
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
