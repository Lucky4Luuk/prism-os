use crate::input::*;

pub struct Cli {
    prev_input: u64,

    pub input_buf: String,
}

impl Cli {
    pub const fn new() -> Self {
        Self {
            prev_input: 0,

            input_buf: String::new(),
        }
    }

    pub fn input(&mut self, new_input: u64) {
        //XOR only checks if the state changed
        let pressed_input = self.prev_input ^ new_input;
        //If a key was not pressed last frame, it becomes 1
        //So if we have detected a state change, and the key was not 1 before, we know it was pressed
        let pressed_input = pressed_input & !self.prev_input;
        self.prev_input = new_input;

        let pressed = input_to_vec(pressed_input);
        for key in &pressed {
            match key {
                Key::Back => { self.input_buf.pop(); },
                Key::Space => self.input_buf.push(' '),
                _ => if let Some(c) = key.if_letter_get() {
                    self.input_buf.push(c);
                },
            }
        }
    }
}
