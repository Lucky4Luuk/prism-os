use crate::Console;
use crate::input::*;

use fixed_queue::VecDeque;

pub struct Cli {
    prev_input: u64,

    pub input_buf: String,
    pub console: Console<15>,

    command_history: VecDeque<String, 32>,
    command_history_index: u8, // 0 = current command
}

impl Cli {
    pub const fn new() -> Self {
        Self {
            prev_input: 0,

            input_buf: String::new(),
            console: Console::new(),

            command_history: VecDeque::new(),
            command_history_index: 0,
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
                Key::Return => self.execute(),
                Key::ArrowUp => self.command_history_index = (self.command_history_index + 1).min(15),
                Key::ArrowDown => if self.command_history_index > 0 {
                    self.command_history_index = self.command_history_index - 1;
                } else {
                    self.input_buf = String::new();
                },
                _ => if let Some(c) = key.if_letter_get(is_key_down(new_input, Key::Shift)) {
                    self.input_buf.push(c);
                },
            }
        }

        if self.command_history_index > 0 {
            if let Some(v) = self.command_history.get((self.command_history_index - 1) as usize) {
                self.input_buf = v.to_string();
            }
        }
    }

    pub fn execute(&mut self) {
        if self.input_buf.is_empty() { return; } // Do nothing when there's no input

        if self.command_history.is_full() { self.command_history.pop_back(); }
        let _ = self.command_history.push_front(self.input_buf.clone());
        self.command_history_index = 0;

        self.console.print(format!("> {}", self.input_buf));
        if self.input_buf.starts_with(".") {
            self.console.print("Running local executables not supported yet!");
        } else {
            let split: Vec<String> = self.input_buf.split(" ").map(|s| s.to_owned()).collect();
            if let Err(e) = unsafe { crate::OS.start_program(&format!("bin/{}.wasm", split[0])) } {
                self.console.print(format!("{:?}", e));
            }
        }
        self.input_buf.clear();
    }

    pub fn flush_to_display(&mut self, display: &mut crate::Display) {
        if let Some(s) = poslib::stdout_fetch(128) {
            self.console.print(s);
        }
        self.console.flush_to_display(display);
    }
}
