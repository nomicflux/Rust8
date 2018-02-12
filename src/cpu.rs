extern crate rand;

use std::sync::Arc;
use std::sync::Mutex;
use std::fs::File;
use std::io::Write;

use opcode::Opcode;
use keyboard::Keyboard;
use display::Display;
use ram::RAM;

pub struct CPU<'a> {
    sound_reg: u8,
    delay_reg: u8,
    stack: Vec<u16>,
    pc: u16,
    i: u16,
    reg: [u8; 16],
    ram: &'a mut RAM,
    display: &'a mut Arc<Mutex<Display>>,
    keyboard: &'a mut Arc<Mutex<Keyboard>>,
    logfile: &'a mut File,
}

impl<'a> CPU<'a> {
    pub fn init(ram: &'a mut RAM,
                display: &'a mut Arc<Mutex<Display>>,
                keyboard: &'a mut Arc<Mutex<Keyboard>>,
                logfile: &'a mut File)
                -> CPU<'a> {
        CPU {
            sound_reg: 0,
            delay_reg: 0,
            stack: Vec::with_capacity(16),
            pc: 0x200,
            i: 0,
            reg: [0; 16],
            ram,
            display,
            keyboard,
            logfile,
        }
    }

    pub fn get_display(&self) -> [u64; 32] {
        self.display.lock().unwrap().get_display()
    }

    pub fn get_reg(&self, x: usize) -> u8 {
        self.reg[x]
    }

    pub fn get_carry(&self) -> u8 {
        self.get_reg(15)
    }

    pub fn get_delay(&self) -> u8 {
        self.delay_reg
    }

    pub fn get_sound(&self) -> u8 {
        self.sound_reg
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn get_key(&self, key: usize) -> bool {
        self.keyboard.lock().unwrap().is_pressed(key)
    }

    pub fn get_at_i(&self) -> u8 {
        self.ram.get_mem8(self.i as usize)
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram.load_fontset();
        self.ram.load_rom(rom);
    }

    fn fetch(&self) -> Opcode {
        Opcode::from_rom(self.ram.get_mem16(self.pc as usize))
    }

    fn inc_pc(&mut self) {
        self.pc += 2;
    }

    fn dec_delay(&mut self) {
        if self.sound_reg > 0 {
            self.sound_reg -= 1;
        }
        if self.delay_reg > 0 {
            self.delay_reg -= 1;
        }
    }

    fn set_carry(&mut self, carry: u8) {
        self.reg[15] = carry;
    }

    fn run_0(&mut self, data: u16) {
        match data {
            0xE0 => {
                self.display.lock().unwrap().clear();
                self.inc_pc();
            },
            0xEE => {
                self.pc = self.stack.pop().unwrap();
                self.inc_pc();
            }
            _ => panic!("Illegal data for 0x0_ op: {}", data),
        }
    }

    fn run_1(&mut self, data: u16) {
        self.pc = data;
    }

    fn run_2(&mut self, data: u16) {
        self.stack.push(self.pc);
        self.pc = data;
    }

    fn run_3(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let comp = (data & 0xFF) as u8;
        if self.reg[x] == comp {
            self.inc_pc();
        }
        self.inc_pc();
    }

    fn run_4(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let comp = (data & 0xFF) as u8;
        if self.reg[x] != comp {
            self.inc_pc();
        }
        self.inc_pc();
    }

    fn run_5(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let y = (data >> 4) as usize;
        print!("{} {} {}\n", x, y, data);
        if self.reg[x] == self.reg[y] {
            self.inc_pc();
        }

        self.inc_pc();
    }

    fn run_6(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let val = (data & 0xFF) as u8;
        self.reg[x] = val;

        self.inc_pc();
    }

    fn run_7(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let val = (data & 0xFF) as u8;
        self.reg[x] = self.reg[x].wrapping_add(val);

        self.inc_pc();
    }

    fn run_8(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let y = ((data >> 4) & 0x0F) as usize;
        let op = (data & 0x0F) as u8;
        match op {
            0 => self.reg[x] = self.reg[y],
            1 => self.reg[x] |= self.reg[y],
            2 => self.reg[x] &= self.reg[y],
            3 => self.reg[x] ^= self.reg[y],
            4 => {
                let (res, carry) = self.reg[x].overflowing_add(self.reg[y]);
                self.reg[x] = res;
                if carry { self.set_carry(1); } else { self.set_carry(0); }
            },
            5 => {
                let (res, carry) = self.reg[x].overflowing_sub(self.reg[y]);
                self.reg[x] = res;
                if carry { self.set_carry(1); } else { self.set_carry(0); }
            },
            6 => {
                let carry = self.reg[y] & 0x01;
                self.reg[x] = self.reg[y] >> 1;
                self.set_carry(carry)
            },
            7 => {
                let (res, carry) = self.reg[y].overflowing_sub(self.reg[x]);
                self.reg[x] = res;
                if carry { self.set_carry(1); } else { self.set_carry(0); }
            },
            0xE => {
                let carry = (self.reg[y] & 0x80) >> 7;
                self.reg[x] = self.reg[y] << 1;
                self.set_carry(carry)
            },
            _ => panic!("Illegal op for 8: {}", op),
        }

        self.inc_pc();
    }

    fn run_9(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let y = (data & 0xFF) as usize;
        if self.reg[x] != self.reg[y] {
            self.inc_pc();
        }
        self.inc_pc();
    }

    fn run_a(&mut self, data: u16) {
        self.i = data;

        self.inc_pc();
    }

    fn run_b(&mut self, data: u16) {
        self.pc = (self.reg[0] as u16) + data;
    }

    fn run_c(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let val = (data & 0xFF) as u8;
        self.reg[x] = val & rand::random::<u8>();
        self.inc_pc();
    }

    fn run_d(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let y = ((data >> 4) & 0x0F) as usize;
        let n = (data & 0x0F) as usize;
        let mut sprite = Vec::with_capacity(n);
        for i in 0..n {
            sprite.push(self.ram.get_mem8((self.i as usize) + i));
        }
        let carry = self.display.lock().unwrap().set_sprite(self.reg[y], self.reg[x], &sprite);
        self.set_carry(if carry { 1 } else { 0 });

        self.inc_pc();
    }

    fn run_e(&mut self, data: u16) {
        self.keyboard.lock().unwrap().read_input();
        let x = (data >> 8) as usize;
        let op = (data & 0xFF) as u8;
        let key = self.keyboard.lock().unwrap().is_pressed(self.reg[x] as usize);
        match op {
            0x9E => {
                if key {
                    self.inc_pc();
                }
            },
            0xA1 => {
                if !key {
                    self.inc_pc();
                }
            },
            _ => panic!("Illegal op for E {}", op),
        }

        self.inc_pc();
    }

    fn run_f(&mut self, data: u16) {
        let x = (data >> 8) as usize;
        let op = (data & 0xFF) as u8;
        match op {
            0x07 => self.reg[x] = self.delay_reg,
            0x0A => {
                self.keyboard.lock().unwrap().reset_last_key();
                loop {
                    self.keyboard.lock().unwrap().read_input();
                    if self.keyboard.lock().unwrap().last_key.is_some() {
                        break;
                    }
                }
                self.reg[x] = self.keyboard.lock().unwrap().last_key.unwrap();
            },
            0x15 => self.delay_reg = self.reg[x],
            0x18 => self.sound_reg = self.reg[x],
            0x1E => self.i += self.reg[x] as u16,
            0x29 => self.i = (self.reg[x] * 5) as u16,
            0x33 => {
                let val = self.reg[x];
                let hundreds = (val / 100) % 10;
                let tens = (val / 10) % 10;
                let ones = val % 10;
                self.ram.set_mem8(self.i as usize, hundreds);
                self.ram.set_mem8((self.i + 1) as usize, tens);
                self.ram.set_mem8((self.i + 2) as usize, ones);
            },
            0x55 => {
                self.ram.set_regs(self.i as usize, &self.reg, (data >> 8) as u8);
                self.i += 8;
            }
            0x65 => {
                self.ram.get_regs(self.i as usize, &mut self.reg, (data >> 8) as u8);
                self.i += 8;
            }
            _ => panic!("Illegal op for F {}", op),
        }

        self.inc_pc();
    }

    fn run_opcode(&mut self, opcode: Opcode) {
        match opcode.op() {
            0x00 => self.run_0(opcode.data()),
            0x01 => self.run_1(opcode.data()),
            0x02 => self.run_2(opcode.data()),
            0x03 => self.run_3(opcode.data()),
            0x04 => self.run_4(opcode.data()),
            0x05 => self.run_5(opcode.data()),
            0x06 => self.run_6(opcode.data()),
            0x07 => self.run_7(opcode.data()),
            0x08 => self.run_8(opcode.data()),
            0x09 => self.run_9(opcode.data()),
            0x0A => self.run_a(opcode.data()),
            0x0B => self.run_b(opcode.data()),
            0x0C => self.run_c(opcode.data()),
            0x0D => self.run_d(opcode.data()),
            0x0E => self.run_e(opcode.data()),
            0x0F => self.run_f(opcode.data()),
            _ => panic!("Opcode {} > 0x0F", opcode.op()),
        }
    }

    pub fn run_cycle(&mut self) {
        let opcode = self.fetch();
        self.logfile.write_all(&opcode.to_string().into_bytes()).unwrap();
        self.logfile.write_all(b"\n").unwrap();
        let _ = self.logfile.flush();
        self.run_opcode(opcode);
        self.dec_delay();
    }
}
