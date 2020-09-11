use core::slice::Iter;
use ggez::event::KeyCode;
use std::collections::HashMap;

// Define some keys that we want for input!
// Currently only have keys for left and right movement.
// Put A, D and Left, Right, in case we want two players! 
#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum Key {
    A,
    D,
    Left,
    Right,
}

impl Key {

    // Returns an iterator for easy input state initialization.
    pub fn iter() -> Iter<'static, Key> {
        static KEYS: [Key; 4] = [Key::A, Key::D, Key::Left, Key::Right];
        KEYS.iter()
    }

    // To easily convert from ggez's KeyCode to our Key enum.
    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::A => Some(Key::A),
            KeyCode::D => Some(Key::D),
            KeyCode::Left => Some(Key::Left),
            KeyCode::Right => Some(Key::Right),
            _ => None
        }
    }
}

pub struct InputState {
    keys_pressed: HashMap<Key, bool>,
}

impl Default for InputState {
    fn default() -> Self {
        let mut keys_pressed: HashMap<Key, bool> = HashMap::new();

        for key in Key::iter() {
            keys_pressed.insert(*key, false);
        }

        InputState {
            keys_pressed: HashMap::new(),
        }
    }
}

impl InputState {
    // Is the given key currently pressed?
    pub fn is_key_pressed(&self, key: Key) -> bool {
        if let Some(is_key_pressed) = self.keys_pressed.get(&key) {
            *is_key_pressed
        } else {
            false
        }
    }

    // Set the pressed state of the given key.
    pub fn set_key_pressed(&mut self, keycode: KeyCode, value: bool) {
        if let Some(key) = Key::from_keycode(keycode) {
            self.keys_pressed.insert(key, value);
        }
    }
}
