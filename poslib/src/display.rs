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
            let (pal, _) = find_palette(c.r(), c.g(), c.b());
            self.set(x,y, pal)?;
        }
        Ok(())
    }
}

const PALETTE: [[u8; 3]; 76] = [
	[9,9,14],
	[26,28,38],
	[60,72,81],
	[97,112,119],
	[153,165,167],
	[203,210,217],
	[255,255,255],
	[33,27,33],
	[107,88,83],
	[171,104,76],
	[206,166,95],
	[231,212,148],
	[249,243,192],
	[36,19,29],
	[64,40,48],
	[94,66,60],
	[128,99,82],
	[161,148,119],
	[189,187,147],
	[76,7,23],
	[129,11,11],
	[168,43,18],
	[212,92,29],
	[227,133,36],
	[235,171,76],
	[241,194,86],
	[246,221,122],
	[3,18,31],
	[15,52,63],
	[26,85,86],
	[44,125,99],
	[75,162,69],
	[148,204,71],
	[234,242,87],
	[2,16,23],
	[11,59,68],
	[23,117,110],
	[48,163,135],
	[80,205,144],
	[106,226,145],
	[201,232,161],
	[23,9,46],
	[21,21,86],
	[17,63,130],
	[52,102,176],
	[113,181,219],
	[158,228,239],
	[209,251,240],
	[38,22,70],
	[85,45,114],
	[136,75,147],
	[172,108,162],
	[197,143,170],
	[223,178,198],
	[237,209,214],
	[20,3,51],
	[70,21,101],
	[123,37,132],
	[169,75,132],
	[208,116,130],
	[222,158,140],
	[123,13,105],
	[164,16,87],
	[195,67,92],
	[225,118,118],
	[243,191,173],
	[60,19,59],
	[107,46,90],
	[170,85,124],
	[202,134,122],
	[242,205,170],
	[250,248,219],
	[138,64,40],
	[179,121,77],
	[218,181,128],
	[243,231,168],
];
const PALETTE_SIZE: usize = 76;

pub fn find_palette(r: u8, g: u8, b: u8) -> (u8, f32) {
    fn delta(left: [u8; 3], right: [u8; 3]) -> f32 {
        let mut delta = 0f32;
        delta += (left[0] as f32 - right[0] as f32).powf(2.0) * 0.299;
        delta += (left[1] as f32 - right[1] as f32).powf(2.0) * 0.587;
        delta += (left[2] as f32 - right[2] as f32).powf(2.0) * 0.114;
        delta
    }

    let mut closest = 0;
    let mut closest_delta = delta([r,g,b], PALETTE[closest]);
    for i in 1..PALETTE_SIZE {
        let d = delta([r,g,b], PALETTE[i]);
        if d < closest_delta {
            closest = i;
            closest_delta = d;
        }
    }
    (closest as u8, closest_delta)
}
