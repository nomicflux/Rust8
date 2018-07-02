use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub const EXIT_CHAR: char = 'l';
const EXIT_VAL: u8 = 17;

lazy_static! {
    static ref KEY_MAP: HashMap<char, u8> = [
        ('1', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('\'', 4),
        (',', 5),
        ('.', 6),
        ('p', 7),
        ('a', 8),
        ('o', 9),
        ('e', 10),
        ('u', 11),
        (';', 12),
        ('q', 13),
        ('j', 14),
        ('k', 15),
        (EXIT_CHAR, EXIT_VAL),
    ].iter()
        .cloned()
        .collect();
}

pub struct Keyboard {
    pub keys: [bool; 16],
    input: Receiver<u8>,
    exit_flag: bool,
    pub last_key: Option<u8>,
}

impl Keyboard {
    pub fn init(input: Receiver<u8>) -> Keyboard {
        Keyboard {
            keys: [false; 16],
            input,
            exit_flag: false,
            last_key: None,
        }
    }

    pub fn read_input(&mut self) {
        let former_key = self.last_key.clone();

        match self.input.try_recv() {
            Ok(key) => {
                let res = KEY_MAP.get(&(key.into())).cloned();
                if res == Some(EXIT_VAL) {
                    self.exit_flag = true;
                    self.last_key = None;
                } else {
                    self.last_key = res;
                }
            }
            _ => self.last_key = None,
        }

        if self.last_key != former_key {
            for fk in former_key {
                if self.last_key != None {
                    self.release_key(fk.into());
                }
            }
            for lk in self.last_key {
                self.push_key(lk.into());
            }
        }
    }

    pub fn exit_key(&self) -> bool {
        self.exit_flag
    }

    pub fn reset_last_key(&mut self) {
        self.last_key = None;
    }

    pub fn push_key(&mut self, key: usize) {
        assert!(key < 16);
        self.keys[key] = true;
    }

    pub fn release_key(&mut self, key: usize) {
        assert!(key < 16);
        self.keys[key] = false;
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        assert!(key < 16);
        self.keys[key]
    }
}
