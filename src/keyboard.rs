pub struct Keyboard {
    keys: [bool; 16],
    pub last_key: Option<u8>,
}

impl Keyboard {
    pub fn init() -> Keyboard {
        Keyboard {
            keys: [false; 16],
            last_key: None,
        }
    }

    pub fn reset_last_key(&mut self) {
        self.last_key = None;
    }

    pub fn push_key(&mut self, key: usize) {
        assert!(key < 16);
        self.keys[key] = true;
        self.last_key = Some(key as u8);
    }

    pub fn release_key(&mut self, key: usize) {
        assert!(key < 16);
        self.keys[key] = true;
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        assert!(key < 16);
        self.keys[key]
    }
}
