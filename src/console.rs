use bitmap_font::{tamzen::FONT_5x9, TextStyle};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};

use fixed_queue::VecDeque;

pub struct Console {
    pub lines: VecDeque<String, 16>,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            lines: VecDeque::new(),
        }
    }

    pub fn print<S: Into<String>>(&mut self, s: S) {
        let s = s.into();
        if self.lines.is_full() {
            self.lines.pop_back();
        }
        let _ = self.lines.push_front(s);
    }

    pub fn flush_to_display(&self, display: &mut crate::Display) {
        for i in 0..16 {
            if let Some(line) = self.lines.get(i) {
                let text = Text::new(line, Point::new(0, i as i32 * 9), TextStyle::new(&FONT_5x9, BinaryColor::On));
                let _ = text.draw(&mut display.color_converted());
            } else {
                break;
            }
        }
    }
}