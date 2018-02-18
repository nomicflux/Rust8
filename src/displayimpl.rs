use super::display::Display;
use super::keyboard::Keyboard;

pub trait DisplayImpl {
    fn draw(&self, screen: &Display, keys: &Keyboard);
}

pub struct AsciiDisplay();

static BLANK_SCREEN: &'static str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

impl AsciiDisplay {
    fn row_to_ascii(&self, row: u64) -> String {
        let mut s = String::new();

        for i in 0..64 {
            if row & (1 << (63 - i)) != 0 {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s
    }

    fn keys_to_ascii(&self, keys: &[bool; 16]) -> String {
        let mut s = String::new();
        for i in 0..16 {
            if keys[i] {
                s.push('*');
            } else {
                s.push('_');
            }
        }
        s
    }

    fn clear(&self) {
        print!("{}", BLANK_SCREEN);
    }
}

impl DisplayImpl for AsciiDisplay {
    fn draw(&self, screen: &Display, keys: &Keyboard) {
        self.clear();
        let screen_rows = screen.get_display();
        let key_presses = keys.keys;

        for i in 0..32 {
            let s = self.row_to_ascii(screen_rows[i]);
            println!("{}", s);
        }
        println!("");
        println!("{}", self.keys_to_ascii(&key_presses));
    }
}
