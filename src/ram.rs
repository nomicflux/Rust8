pub struct RAM([u8; 4000]);

impl RAM {
    pub fn init() -> RAM {
        RAM([0; 4000])
    }

    pub fn set_mem8(&mut self, pos: usize, val: u8) {
        self.0[pos] = val;
    }

    pub fn set_mem16(&mut self, pos: usize, val: u16) {
        self.0[pos] = (val >> 8) as u8;
        self.0[pos + 1] = (val & 0xFF) as u8;
    }

    pub fn get_mem8(&self, pos: usize) -> u8 {
        self.0[pos]
    }

    pub fn get_mem16(&self, pos: usize) -> u16 {
        let a = (self.0[pos] as u16) << 8;
        let b = self.0[pos + 1] as u16;
        a | b
    }
}

#[test]
fn test_empty() {
    assert_eq!(RAM::init().get_mem8(0), 0);
    assert_eq!(RAM::init().get_mem8(2000), 0);
    assert_eq!(RAM::init().get_mem8(3999), 0);
}

#[test]
fn test_set_get8() {
    let mem_val = 5;
    let mut mem = RAM::init();
    mem.set_mem8(42, mem_val);
    assert_eq!(mem.get_mem8(42), mem_val);
}

#[test]
fn test_double_set8_same() {
    let mut mem = RAM::init();
    mem.set_mem8(42, 1);
    mem.set_mem8(42, 2);
    assert_eq!(mem.get_mem8(42), 2);
}

#[test]
fn test_double_set8_diff() {
    let mut mem = RAM::init();
    mem.set_mem8(42, 1);
    mem.set_mem8(43, 2);
    assert_eq!(mem.get_mem8(42), 1);
    assert_eq!(mem.get_mem8(43), 2);
}

#[test]
fn test_set_get16() {
    let mem_val = 0xABCD;
    let mut mem = RAM::init();
    mem.set_mem16(42, mem_val);
    assert_eq!(mem.get_mem16(42), mem_val);
}

#[test]
fn test_double_set16() {
    let mut mem = RAM::init();
    mem.set_mem16(42, 0x1234);
    mem.set_mem16(43, 0xABCD);
    assert_eq!(mem.get_mem16(42), 0x12AB);
    assert_eq!(mem.get_mem16(43), 0xABCD);
}
