#[macro_use] extern crate lazy_static;

#[derive(Default)]
pub struct FrameInfo<'frame> {
    buf: &'frame mut [u8],
    resolution: (usize, usize),
}

impl FrameInfo<'_> {
    pub fn new(mem_addr: u32) -> Self {
        Self {
            buf: unsafe { std::slice::from_raw_parts_mut(mem_addr as *mut u8, 168*72*4) },
            resolution: (168, 72),
        }
    }
}

#[no_mangle]
pub extern "C" fn draw_unsafe(mem_addr: u32) {
    draw(FrameInfo::new(mem_addr))
}

fn draw(info: FrameInfo) {
    for x in 0..info.resolution.0 {
        for y in 0..info.resolution.1 {
            let i = (x + y * info.resolution.0) * 4;
            info.buf[i  ] = 255;
            info.buf[i+1] = 0;
            info.buf[i+2] = 0;
            info.buf[i+3] = 255;
        }
    }
}
