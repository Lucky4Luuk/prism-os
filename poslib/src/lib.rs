extern "C" {
    fn spawn_runtime(ptr: u64, len: u64) -> u64;
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
