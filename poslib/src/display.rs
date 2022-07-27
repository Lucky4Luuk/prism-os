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
        }
    }

    pub fn with_func<F: Fn(usize, usize) -> u8>(&mut self, f: F) {
        for x in 0..BUFFER_WIDTH {
            for y in 0..BUFFER_HEIGHT {
                let _ = self.set(x, y, f(x,y)); // We should not be able to trigger an error here, so we just ignore it :)
            }
        }
    }

    pub fn get(&mut self, x: usize, y: usize) -> u8 {
        let i = x + y * BUFFER_WIDTH;
        self.buf[i]
    }

    pub fn set(&mut self, x: usize, y: usize, color: u8) -> Result<(), DrawError> {
        if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
            // return Err(DrawError::SetOutOfRange);
            return Ok(());
        }
        let i = x + y * BUFFER_WIDTH;
        self.buf[i] = color;
        Ok(())
    }

    pub fn draw_image(&mut self, x: usize, y: usize, bmp: &DynamicTga<Rgb888>) {
        Image::new(bmp, Point::new(x as i32, y as i32)).draw(&mut self.color_converted()).unwrap();
    }

    pub fn draw_text<S: Into<String>>(&mut self, x: usize, y: usize, text: S) {
        use bitmap_font::{tamzen::FONT_5x9, TextStyle};
        use embedded_graphics::{prelude::*, text::Text};
        let text = text.into();
        let drawtext = Text::new(&text, Point::new(x as i32, y as i32), TextStyle::new(&FONT_5x9, BinaryColor::On));
        let _ = drawtext.draw(&mut self.color_converted());
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
            self.set(x,y, find_palette(c.r(), c.g(), c.b()))?;
        }
        Ok(())
    }
}

const PALETTE: [[u8; 3]; 29] = [
    [  0,  0,  0], // black
    [204, 36, 29], // red
    [152,151, 26], // green
    [215,153, 33], // yellow
    [ 69,133,136], // blue
    [177, 98,134], // purple
    [104,157,106], // aqua
    [214, 93, 14], // orange

    [251, 73, 52], // light_red
    [184,187, 38], // light_green
    [250,189, 47], // light_yellow
    [131,165,152], // light_blue
    [211,134,155], // light_purple
    [142,192,124], // light_aqua
    [254,128, 25], // light_orange

    [ 40, 40, 40], // bg0
    [ 60, 56, 54], // bg1
    [ 80, 73, 69], // bg2
    [102, 92, 84], // bg3
    [124,111,100], // bg4
    [168,153,132], // gray0
    [146,131,116], // gray1

    [168,153,132], // fg4
    [189,174,147], // fg3
    [213,196,161], // fg2
    [235,219,178], // fg1
    [251,241,199], // fg0,

    [ 29, 32, 33], // bg0_hard
    [ 50, 48, 47], // bg0_soft
];

pub fn find_palette(r: u8, g: u8, b: u8) -> u8 {
    fn delta(a: [u8; 3], b: [u8; 3]) -> f32 {
        let mut delta = 0f32;
        for i in 0..3 {
            delta += (a[i] as f32 - b[i] as f32).abs();
        }
        delta / 3f32
    }

    let mut closest = 0;
    let mut closest_delta = delta([r,g,b], PALETTE[closest]);
    for i in 1..PALETTE.len() {
        let d = delta([r,g,b], PALETTE[i]);
        if d < closest_delta {
            closest = i;
            closest_delta = d;
        }
    }
    closest as u8
}
