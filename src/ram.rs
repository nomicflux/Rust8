pub struct RAM([u8; 4000]);

impl RAM {
    pub fn init() -> RAM {
        RAM([0; 4000])
    }

    pub fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for (i, &f) in fontset.into_iter().enumerate() {
            self.0[i] = f;
        }
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

    pub fn set_regs(&mut self, pos: usize, regs: &[u8; 16]) {
        for (i, &data) in regs.into_iter().enumerate() {
            self.0[pos + i] = data;
        }
    }

    pub fn get_regs(&self, pos: usize, regs: &mut [u8; 16]) {
        for (i, reg) in regs.into_iter().enumerate() {
            *reg = self.0[pos + i];
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &data) in rom.into_iter().enumerate() {
            self.set_mem8(0x200 + i, data);
        }
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

#[test]
fn test_load_rom() {
    let mut mem = RAM::init();
    let rom = [0xFF, 0xEE];
    mem.load_rom(&rom);
    assert_eq!(mem.get_mem16(0x200), 0xFFEE);
}

#[test]
fn test_set_regs() {
    let mut mem = RAM::init();
    let regs = [0xFF; 16];
    mem.set_regs(0, &regs);
    assert_eq!(mem.get_mem16(0x00), 0xFFFF);
    assert_eq!(mem.get_mem16(0x02), 0xFFFF);
    assert_eq!(mem.get_mem16(0x04), 0xFFFF);
    assert_eq!(mem.get_mem16(0x06), 0xFFFF);
    assert_eq!(mem.get_mem16(0x08), 0xFFFF);
    assert_eq!(mem.get_mem16(0x0A), 0xFFFF);
    assert_eq!(mem.get_mem16(0x0C), 0xFFFF);
    assert_eq!(mem.get_mem16(0x0E), 0xFFFF);
}

#[test]
fn test_get_regs() {
    let mem = RAM::init();
    let mut regs = [0xFF; 16];
    mem.get_regs(0, &mut regs);
    for &reg in regs.iter() {
        assert_eq!(reg, 0x00);
    }
}
