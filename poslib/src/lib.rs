mod display;
pub use display::*;

pub const BUFFER_WIDTH: usize = 336;
pub const BUFFER_HEIGHT: usize = 144;
pub const BUFFER_LEN: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

extern "C" {
    fn spawn_runtime(ptr: u64, len: u64) -> u64;
    fn read_stdout(buf_ptr: u64, len: u64) -> u64;
}

#[derive(Default)]
pub struct FrameInfo<'frame> {
    pub buf: &'frame mut [u8],
}

impl FrameInfo<'_> {
    pub fn new(mem_addr: u32) -> Self {
        Self {
            buf: unsafe { std::slice::from_raw_parts_mut(mem_addr as *mut u8, 336*144) },
        }
    }
}

pub struct Program {
    pub id: u64,
    wasm_bytes: Vec<u8>,
}

pub fn run_program<S: Into<String>>(path: S) -> Result<Program, std::io::Error> {
    let wasm_bytes = std::fs::read(path.into())?;
    let mut prog = Program {
        id: 0,
        wasm_bytes: wasm_bytes,
    };
    prog.id = unsafe {
        spawn_runtime(prog.wasm_bytes.as_ptr() as u64, prog.wasm_bytes.len() as u64)
    };
    Ok(prog)
}

pub fn stdout_fetch(bytes: usize) -> Option<String> {
    let buf = vec![0u8; bytes]; // Read N bytes every time
    let bytes_read = unsafe { read_stdout(buf.as_ptr() as u64, buf.len() as u64) } as usize;
    if bytes_read > 0 {
        std::str::from_utf8(&buf[..bytes_read]).map(|s| s.to_string()).ok()
    } else {
        None
    }
}
