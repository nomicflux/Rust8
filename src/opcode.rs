#[derive(PartialEq, Debug)]
pub struct Opcode(u16);

impl Opcode {
    pub fn from_rom(a: u16) -> Opcode {
        Opcode(a)
    }

    pub fn op(&self) -> u8 {
        (self.0 >> 12) as u8
    }

    pub fn data(&self) -> u16 {
        self.0 & 0x0FFF
    }

    pub fn to_string(&self) -> String {
        format!("0x{:X}", self.0)
    }
}

#[test]
fn test_clear_screen() {
    let opcode = Opcode(0x00E0);
    assert_eq!(opcode.op(), 0x00);
    assert_eq!(opcode.data(), 0x00E0);
}

#[test]
fn test_set_i() {
    let opcode = Opcode(0xA123);
    assert_eq!(opcode.op(), 0x0A);
    assert_eq!(opcode.data(), 0x0123);
}
