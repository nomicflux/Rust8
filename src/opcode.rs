pub struct Opcode(u16);

impl Opcode {
    pub fn op(&self) -> u8 {
        (self.0 >> 12) as u8
    }

    pub fn data(&self) -> u16 {
        self.0 & 0x0FFF
    }
}

// enum Ops {
//     Call(u16),
//     Clear,
//     Return,
//     Jump(u16),
//     CallSub(u16),
//     Eq(u8, u16),
//     Neq(u8, u16),
//     EqReg(u8, u8),
//     Set(u8, u16),
//     Add(u8, u16),
//     SetReg(u8, u8),
//     OrReg(u8, u8),
//     AndReg(u8, u8),
//     XorReg(u8, u8),
//     AddReg(u8, u8),
//     SubReg(u8, u8),
//     SRReg(u8, u8),
//     SubFromReg(u8, u8),
//     SLReg(u8, u8),
//     NeqReg(u8, u8),
//     SetI(u16),
//     JumpU0(u16),
//     Rand(u8, u16),
//     Draw(u8, u8, u8),
//     SkipKey(u8),
//     SkipNotKey(u8),
//     SetDelay(u8),
//     SetSound(u8),
//     AddI(u8),
//     SetLoc(u8),
//     StoreDec(u8),
//     RegDump(u8),
//     RegLoad(u8)
// }

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
