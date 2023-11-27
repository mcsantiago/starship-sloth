use std::collections::HashMap;

use winit::event::VirtualKeyCode;

pub struct InputManager {
    keys_pressed: HashMap<VirtualKeyCode, bool>
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashMap::new()
        }
    }

    pub fn set_key_pressed(&mut self, key: VirtualKeyCode, pressed: bool) {
        self.keys_pressed.insert(key, pressed);
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        *self.keys_pressed.get(&key).unwrap_or(&false)
    }
}

