use embedded_graphics::pixelcolor::*;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::image::Image;
use embedded_graphics::prelude::*;

use tinytga::*;

use crate::{BUFFER_WIDTH, BUFFER_HEIGHT, BUFFER_LEN};

#[derive(Debug)]
pub enum DrawError {
    SetOutOfRange,
}

pub struct Display {
    pub buf: [u8; BUFFER_LEN],
}

impl Display {
    pub const fn new() -> Self {
        Self {
            buf: [0; BUFFER_LEN],
        }
    }

    pub fn flush(&self, dest: &mut [u8]) {
        dest.copy_from_slice(&self.buf);
    }

    pub fn clear_black(&mut self) {
        for i in 0..BUFFER_LEN {
            self.buf[i] = 0;
            if i % 3 == 3 {
                self.buf[i] = 255;
            }
        }
    }

    pub fn with_func<F: Fn(usize, usize) -> [u8; 4]>(&mut self, f: F) {
        for x in 0..BUFFER_WIDTH {
            for y in 0..BUFFER_HEIGHT {
                let _ = self.set(x, y, f(x,y)); // We should not be able to trigger an error here, so we just ignore it :)
            }
        }
    }

    pub fn get(&mut self, x: usize, y: usize) -> &[u8] {
        let i = x + y * BUFFER_WIDTH;
        &self.buf[i*4..i*4+4]
    }

    pub fn set(&mut self, x: usize, y: usize, color: [u8; 4]) -> Result<(), DrawError> {
        if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
            // return Err(DrawError::SetOutOfRange);
            return Ok(());
        }
        let i = x + y * BUFFER_WIDTH;
        self.buf[i*4..i*4+4].copy_from_slice(&color);
        Ok(())
    }

    pub fn draw_image(&mut self, x: usize, y: usize, bmp: &DynamicTga<Rgb888>) {
        Image::new(bmp, Point::new(x as i32, y as i32)).draw(&mut self.color_converted()).unwrap();
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(BUFFER_WIDTH as u32, BUFFER_HEIGHT as u32)
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;
    type Error = DrawError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> where I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>> {
        for pixel in pixels {
            let x = pixel.0.x as usize;
            let y = pixel.0.y as usize;
            let c = pixel.1;
            self.set(x,y, [c.r(), c.g(), c.b(), 255])?;
        }
        Ok(())
    }
}
