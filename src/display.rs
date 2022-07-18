pub struct Display {
    pub buf: [u8; 168*72*4],
}

impl Display {
    pub const fn new() -> Self {
        Self {
            buf: [0; 168*72*4],
        }
    }

    pub fn flush(&self, dest: &mut [u8]) {
        dest.copy_from_slice(&self.buf);
    }

    pub fn clear(&mut self) {
        for i in 0..168*72*4 {
            self.buf[i] = 0;
            if i % 3 == 3 {
                self.buf[i] = 255;
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: [u8; 4]) {
        let i = x + y * 168;
        self.buf[i*4..i*4+4].copy_from_slice(&color);
    }

    pub fn with_func<F: Fn(usize, usize) -> [u8; 4]>(&mut self, f: F) {
        for x in 0..168 {
            for y in 0..72 {
                self.set(x, y, f(x,y));
            }
        }
    }
}
