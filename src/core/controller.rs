use std::collections::HashSet;
use sdl2::keyboard::Keycode;

pub struct Controller {
    key_pressed: HashSet<Keycode>
}

impl Controller {
    pub fn new() -> Result<Controller, String> {
        Ok(Controller { key_pressed: HashSet::new() })
    }

    pub fn key_pressed(&mut self, key_code: Keycode) {
        self.key_pressed.replace(key_code);
    }

    pub fn key_released(&mut self, key_code: Keycode) {
        self.key_pressed.remove(&key_code);
    }

    pub fn is_key_pressed(&mut self, key_code: Keycode) -> bool {
        self.key_pressed.contains(&key_code)
    }
    
    pub fn is_key_pressed_once(&mut self, key_code: Keycode) -> bool {
        self.key_pressed.remove(&key_code)
    }
   
}