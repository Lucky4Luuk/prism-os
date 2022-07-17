#[macro_use] extern crate lazy_static;

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

#[no_mangle]
pub extern "C" fn tick(mem_addr: u32, input: u64, delta_s: f32) {
    draw(FrameInfo::new(mem_addr))
}

fn draw(info: FrameInfo) {
    for x in 0..168 {
        for y in 0..72 {
            let uvx = x as f32 / 168f32;
            let uvy = y as f32 / 72f32;

            let i = (x + y * 168) * 4;
            info.buf[i  ] = (uvx * 255.0) as u8;
            info.buf[i+1] = (uvy * 255.0) as u8;
            info.buf[i+2] = 0;
            info.buf[i+3] = 255;
        }
    }
}
