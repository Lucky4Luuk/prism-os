use embedded_graphics::pixelcolor::*;
use lazy_static::lazy_static;

use tinytga::*;

pub static SPLASHSCREEN_TGA_DATA: &'static [u8] = include_bytes!("../splashscreen.tga");

lazy_static! {
    pub static ref SPLASHSCREEN_TGA: DynamicTga<'static, Rgb888> = DynamicTga::from_slice(SPLASHSCREEN_TGA_DATA).expect("Failed to load splashscreen tga!");
}
