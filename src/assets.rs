use lazy_static::lazy_static;

use tinybmp::Bmp;
use embedded_graphics::pixelcolor::Rgb888;

pub const SPLASHSCREEN_BMP_DATA: &'static [u8] = include_bytes!("../splashscreen.bmp");

lazy_static! {
    pub static ref SPLASHSCREEN_BMP: Bmp<'static, Rgb888> = Bmp::from_slice(SPLASHSCREEN_BMP_DATA).expect("Failed to load splashscreen bmp!");
}
