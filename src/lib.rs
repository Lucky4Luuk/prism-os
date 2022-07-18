mod display;

use display::Display;

#[derive(Default)]
pub struct FrameInfo<'frame> {
    buf: &'frame mut [u8],
}

impl FrameInfo<'_> {
    pub fn new(mem_addr: u32) -> Self {
        Self {
            buf: unsafe { std::slice::from_raw_parts_mut(mem_addr as *mut u8, 168*72*4) },
        }
    }
}

struct Os {
    input: u64,
    total_time: f32,

    pub display: Display,
}

impl Os {
    const fn new() -> Self {
        Self {
            input: 0, // No keys pressed
            total_time: 0f32,

            display: Display::new(),
        }
    }

    fn update_input(&mut self, input: u64) {
        self.input = input;
    }

    fn update(&mut self, delta_s: f32) {
        self.total_time += delta_s;
    }

    fn draw(&mut self, info: FrameInfo) {
        self.display.clear();
        for x in 0..56 {
            for y in 0..24 {
                let presence = ((self.total_time * 50f32) as usize + x + y) % 56;
                if presence < 6 {
                    for ix in 0..3 {
                        for iy in 0..3 {
                            self.display.set(x*3+ix, y*3+iy, [255; 4]);
                        }
                    }
                } else if presence < 36 {
                    self.display.set(x*3+1, y*3+1, [255; 4]);
                }
            }
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
