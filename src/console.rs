use bitmap_font::{tamzen::FONT_5x9, TextStyle};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};

use fixed_queue::VecDeque;

pub struct Console<const N: usize> {
    pub lines: VecDeque<String, N>,
}

impl<const N: usize> Console<N> {
    pub const fn new() -> Self {
        Self {
            lines: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn print<S: Into<String>>(&mut self, s: S) {
        let s = s.into();
        let s = s.trim_end();
        let split: Vec<String> = split_into_chunks(s.to_string(), 67).into_iter().map(|s| s.split('\n').map(|s| s.to_string()).collect::<Vec<String>>()).flatten().collect();
        while self.lines.len() > (N - split.len()) {
            self.lines.pop_front();
        }
        for part in split {
            let _ = self.lines.push_back(part.to_string());
        }
    }

    pub fn flush_to_display(&self, display: &mut crate::Display) {
        for i in 0..N {
            if let Some(line) = self.lines.get(i) {
                let text = Text::new(line, Point::new(0, i as i32 * 9), TextStyle::new(&FONT_5x9, BinaryColor::On));
                let _ = text.draw(&mut display.color_converted());
            } else {
                break;
            }
        }
    }
}

fn split_into_chunks(s: String, n: usize) -> Vec<String> {
    s.chars().collect::<Vec<char>>().chunks(n).map(|c| c.iter().collect::<String>()).collect()
}
