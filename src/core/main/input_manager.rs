use std::collections::HashSet;
use sdl2::{keyboard::Keycode, EventPump};

pub struct InputManager {
    current_buffer: HashSet<Keycode>,
    previous_buffer: HashSet<Keycode>
}

#[allow(dead_code)]
impl InputManager {
    pub fn new() -> InputManager {
        return InputManager{
            current_buffer: HashSet::new(),
            previous_buffer: HashSet::new()
        }
    }
    
    pub fn poll_events(&mut self, event_queue: &mut EventPump) {
        /* follow example @
            https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/keyboard-state.rs */
        self.previous_buffer = self.current_buffer.clone();

        self.current_buffer = event_queue
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect(); 
    }

    pub fn get_key_pressed(&self, key_code: Keycode) -> bool{ 
        for key in &self.current_buffer {
            if *key == key_code {
                return true;
            }
        }
        return false;
    }

    pub fn get_key_just_pressed(&self, key_code: Keycode) -> bool{ 
        for key in &self.current_buffer {
            if *key == key_code {
                for prev_key in &self.previous_buffer {
                    if *prev_key == key_code {
                        return false;
                    }
                }
                return true;
            }
        }
    
        return false;
    }

    pub fn get_key_just_released(&self, key_code: Keycode) -> bool{  
        for key in &self.previous_buffer {
            if *key == key_code {
                for prev_key in &self.current_buffer {
                    if *prev_key == key_code {
                        return false;
                    }
                }
                return true;
            }
        }
    
        return false;
    }
}
