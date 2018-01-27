pub struct CPU {
    pc: u16,
    i: u16,
    reg: [u8; 16],
    ram: super::RAM,
}

impl CPU {
    fn load_reg(&mut self, r: u8, val: u8) {
        assert!(r < 0xF);
        self.reg[r as usize] = val
    }

    fn dump_reg(&self, r: u8) -> u8 {
        assert!(r < 0xF);
        self.reg[r as usize]
    }

    fn inc_pc(& mut self) {
        self.pc += 1;
    }
}
