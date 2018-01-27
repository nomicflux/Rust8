pub struct Display([u64; 32]);

impl Display {
    pub fn init() -> Display {
        Display([0; 32])
    }

    pub fn clear(&mut self) {
        self.0 = [0; 32];
    }

    fn set_sprite_row(&mut self, row: u8, col: u8, sprite_row: u8) -> bool {
        assert!(row < 32);
        assert!(col < 64 - 8);
        let oldrow = self.0[row as usize].clone();
        let newrow = self.0[row as usize] ^ ((sprite_row as u64) << col);
        self.0[row as usize] = newrow;
        (newrow | oldrow) > newrow
    }

    pub fn set_sprite(&mut self, row: u8, col: u8, sprite: &[u8]) -> bool {
        assert!(row + (sprite.len() as u8) < 32);
        assert!(col < 64 - 8);
        let mut collision = false;
        for (i, &sprite_row) in sprite.into_iter().enumerate() {
            collision = self.set_sprite_row(row + (i as u8), col, sprite_row) || collision;
        }
        collision
    }

    pub fn is_collision(&self, row: u8, col: u8) -> bool {
        (self.0[row as usize] & (1 << col)) != 0
    }

    pub fn get_display(&self) -> [u64; 32] {
        self.0
    }
}

#[test]
fn test_init() {
    assert_eq!(Display::init().0[0], 0);
    assert_eq!(Display::init().0[31], 0);
}

#[test]
fn test_at_origin() {
    let mut display = Display::init();
    display.set_sprite_row(0, 0, 0xFF);
    assert_eq!(display.0[0], 0xFF);
}

#[test]
fn test_at_12() {
    let mut display = Display::init();
    display.set_sprite_row(1, 2, 0xFF);
    assert_eq!(display.0[0], 0);
    assert_eq!(display.0[1], 0xFF << 2);
}

#[test]
fn test_overwrite() {
    let mut display = Display::init();
    display.set_sprite_row(0,0,0x0F);
    display.set_sprite_row(0,0,0x01);
    assert_eq!(display.0[0], 0x0E);
}

#[test]
fn test_sprite() {
    let mut display = Display::init();
    display.set_sprite(0,0, &[0xAB, 0xCD]);
    assert_eq!(display.0[0], 0xAB);
    assert_eq!(display.0[1], 0xCD);
}

#[test]
fn test_collision_on_empty() {
    let display = Display::init();
    assert!(!display.is_collision(0,1));
}

#[test]
fn test_non_collision_direct() {
    let mut display = Display::init();
    display.set_sprite(0,0,&[0x01]);
    assert!(!display.is_collision(0,1));
}

#[test]
fn test_collision_direct() {
    let mut display = Display::init();
    display.set_sprite(0,0,&[0xFF]);
    assert!(display.is_collision(0,1));
}

#[test]
fn test_non_collision_drawing() {
    let mut display = Display::init();
    assert!(!display.set_sprite(0,0,&[0x01]));
}

#[test]
fn test_collision_drawing() {
    let mut display = Display::init();
    display.set_sprite(0,0,&[0xFF]);
    assert!(display.set_sprite(0,0,&[0x01]));
}
