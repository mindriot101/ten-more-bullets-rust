use sdl2::keyboard::Keycode;

pub(crate) struct KeyMap {
    pub(crate) keys: Vec<bool>,
}

impl KeyMap {
    pub(crate) fn new() -> Self {
        let mut keys = Vec::new();
        /* This must match the number of used keys */
        keys.push(false);

        KeyMap { keys: keys }
    }

    pub(crate) fn is_pressed(&self, key: Keycode) -> bool {
        if let Some(index) = KeyMap::get_index(key) {
            self.keys[index]
        } else {
            false
        }
    }

    pub(crate) fn mark(&mut self, key: Keycode) {
        if let Some(index) = KeyMap::get_index(key) {
            self.keys[index] = true;
        }
    }

    pub(crate) fn clear(&mut self, key: Keycode) {
        if let Some(index) = KeyMap::get_index(key) {
            self.keys[index] = false;
        }
    }

    fn get_index(key: Keycode) -> Option<usize> {
        match key {
            Keycode::Space => Some(0),
            _ => None,
        }
    }
}
