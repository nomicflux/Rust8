extern crate rust8;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender,channel};
use std::thread::{spawn,sleep};
use std::time::Duration;

use rust8::keyboard::Keyboard;
use rust8::display::Display;
use rust8::ram::RAM;

use rust8::cpu::*;

fn cpu_tester<F>(test: &mut F)
where F: FnMut(&mut CPU, &Sender<u8>) {

    let (sender, receiver) = channel();
    let mut display = Display::init();
    let mut keyboard = Arc::new(Mutex::new(Keyboard::init(receiver)));
    let mut ram: RAM = RAM::init();
    let mut cpu = CPU::init(&mut ram, &mut display, &mut keyboard);
    (move || test(&mut cpu, &sender))();
}

#[test]
fn test_00e0() { // Clear Screen
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x00, 0xE0,  // Clear screen
                   0x60, 0x00,  // Set x0 to 0
                   0xF0, 0x29,  // Load fontset for 0
                   0xD0, 0x01,  // Draw image in x0
                   0x00, 0xE0];  // Clear screen
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_display()[0], 0);

        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        assert!(cpu.get_display()[0] != 0);

        cpu.run_cycle();
        assert_eq!(cpu.get_display()[0], 0);
    })
}

#[test]
fn test_6xnn() { // Set Address
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x60, 0xAB,
                   0x60, 0xCC,
                   0x6E, 0x42];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xAB);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xCC);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(14), 0x42);
    })
}

#[test]
fn test_7xnn() { // Add To Reg
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x70, 0x11,
                   0x70, 0x22,
                   0x70, 0xCE];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x11);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x33);

        // Test no-carry
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
        assert_eq!(cpu.get_carry(), 0x00);
    })
}

#[test]
fn test_8xy0() { // Set X to Y
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x10,
                   0x61, 0xAB,
                   0x80, 0x10];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xAB);
        assert_eq!(cpu.get_reg(1), 0xAB);
    })
}

#[test]
fn test_8xy1() { // X |= Y
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x11,
                   0x60, 0x01,
                   0x61, 0x02,
                   0x80, 0x11];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x02);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x03);
        assert_eq!(cpu.get_reg(1), 0x02);
    })
}

#[test]
fn test_8xy2() { // X &= Y
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x12,
                   0x60, 0x06,
                   0x61, 0x03,
                   0x80, 0x12];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x06);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x03);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x02);
        assert_eq!(cpu.get_reg(1), 0x03);
    })
}

#[test]
fn test_8xy3() { // X ^= Y
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x13,
                   0x60, 0x03,
                   0x61, 0x01,
                   0x80, 0x13];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x03);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x01);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x02);
        assert_eq!(cpu.get_reg(1), 0x01);
    })
}

#[test]
fn test_8xy4() { // X += Y (with carry)
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x14,
                   0x60, 0x11,
                   0x61, 0x22,
                   0x80, 0x14,
                   0x61, 0xCE,
                   0x80, 0x14];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x11);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x22);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x33);
        assert_eq!(cpu.get_reg(1), 0x22);
        assert_eq!(cpu.get_carry(), 0x00);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0xCE);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
        assert_eq!(cpu.get_carry(), 0x01);
    })
}

#[test]
fn test_8xy5() { // X -= Y (with borrow)
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x15,
                   0x60, 0x22,
                   0x61, 0x11,
                   0x80, 0x15,
                   0x61, 0x12,
                   0x80, 0x15];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x22);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x11);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x11);
        assert_eq!(cpu.get_reg(1), 0x11);
        assert_eq!(cpu.get_carry(), 0x00);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x12);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xFF);
        assert_eq!(cpu.get_carry(), 0x01);
    })
}

#[test]
fn test_8xy6() { // X >>= Y (with spillover)
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x16,
                   0x61, 0x06,
                   0x80, 0x16,
                   0x61, 0x03,
                   0x80, 0x16];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x06);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x03);
        assert_eq!(cpu.get_reg(1), 0x06);
        assert_eq!(cpu.get_carry(), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x03);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
        assert_eq!(cpu.get_reg(1), 0x03);
        assert_eq!(cpu.get_carry(), 0x01);
    })
}

#[test]
fn test_8xy7() { // X = Y - X (with borrow)
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x17,
                   0x60, 0x11,
                   0x61, 0x22,
                   0x80, 0x17,
                   0x61, 0x10,
                   0x80, 0x17];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x11);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x22);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x11);
        assert_eq!(cpu.get_reg(1), 0x22);
        assert_eq!(cpu.get_carry(), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x10);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xFF);
        assert_eq!(cpu.get_carry(), 0x01);
    })
}

#[test]
fn test_8xye() { // X <<= Y (with spillover)
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x80, 0x1E,
                   0x61, 0x7F,
                   0x80, 0x1E,
                   0x61, 0xFE,
                   0x80, 0x1E];
        cpu.load_rom(&rom);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x00);
        assert_eq!(cpu.get_reg(1), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0x7F);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xFE);
        assert_eq!(cpu.get_reg(1), 0x7F);
        assert_eq!(cpu.get_carry(), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(1), 0xFE);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xFC);
        assert_eq!(cpu.get_reg(1), 0xFE);
        assert_eq!(cpu.get_carry(), 0x01);
    })
}

#[test]
fn test_delay_timer() { // FX07, FX15
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0xF0, 0x07,
                   0x60, 0x03,
                   0xF0, 0x15,
                   0xF0, 0x07];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_delay(), 0);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x03);
        cpu.run_cycle();
        assert_eq!(cpu.get_delay(), 0x02);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x02);
    });
}

#[test]
fn test_basic_subroutine_flow() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x22, 0x04,
                   0x60, 0x01,
                   0x60, 0x03,
                   0x00, 0xEE];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x03);
        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
    });
}

#[test]
fn test_3xnn() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x30, 0x01,
                   0x60, 0x03,
                   0x30, 0x03,
                   0x60, 0x04,
                   0x60, 0x05];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 3);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 3);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 5);
    });
}

#[test]
fn test_4xnn() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x40, 0x00,
                   0x60, 0x03,
                   0x40, 0x01,
                   0x60, 0x04,
                   0x60, 0x05];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 3);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 3);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 5);
    });
}

#[test]
fn test_5xy0() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x60, 0x01,
                   0x50, 0x10,
                   0x61, 0x01,
                   0x50, 0x10,
                   0x60, 0x04,
                   0x60, 0x05];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
        assert_eq!(cpu.get_reg(1), 0x00);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
        assert_eq!(cpu.get_reg(1), 0x00);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);
        assert_eq!(cpu.get_reg(1), 0x01);

        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x05);
    });
}

#[test]
fn test_annn() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0xA0, 0x42];
        cpu.load_rom(&rom);

        assert_eq!(cpu.get_i(), 0);

        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 0x42);
    });
}

#[test]
fn test_bnnn() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0xB2, 0x04,
                   0x60, 0x01,
                   0x60, 0x06,
                   0xB2, 0x04,
                   0x60, 0x02,
                   0x60, 0x03];

        cpu.load_rom(&rom);

        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x06);

        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x03);
    });
}

#[test]
fn test_ex9e() {
    cpu_tester(&mut |cpu, sender| {
        let rom = [0xE0, 0x9E,
                   0x60, 0x01,
                   0xE0, 0x9E,
                   0x60, 0x02,
                   0x60, 0x03];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x01);

        let _ = sender.send('2' as u8);
        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x03);
    });
}

#[test]
fn test_exa1() {
    cpu_tester(&mut |cpu, sender| {
        let rom = [0x60, 0x01,
                   0xE0, 0xA1,
                   0x60, 0x02,
                   0xE0, 0xA1,
                   0x60, 0x03,
                   0x60, 0x04];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        let _ = sender.send('2' as u8);
        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x02);
        let _ = sender.send('1' as u8);
        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x04);
    });
}

#[test]
fn test_fx0a() {
    cpu_tester(&mut |cpu, sender| {
        let rom = [0xF0, 0x0A];
        cpu.load_rom(&rom);

        assert_eq!(cpu.get_reg(0), 0x00);
        let sender_clone = sender.clone();
        spawn(move || {
            sleep(Duration::new(1,0));
            let _ = sender_clone.send('3' as u8);
        });
        assert_eq!(cpu.get_reg(0), 0x00);
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0x02);
    });
}

#[test]
fn test_fx1e() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x60, 0x01,
                   0xF0, 0x1E];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 0);
        assert_eq!(cpu.get_reg(0), 1);

        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 1);
        assert_eq!(cpu.get_reg(0), 1);
    });
}

#[test]
fn test_fx33() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0x60, 0xFE,
                   0xF0, 0x33,
                   0x60, 0x01,
                   0xF0, 0x1E,
                   0xF0, 0x1E];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xFE);
        assert_eq!(cpu.get_i(), 0);

        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 0);
        assert_eq!(cpu.get_at_i(), 2);

        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 1);
        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 1);
        assert_eq!(cpu.get_at_i(), 5);

        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 2);
        assert_eq!(cpu.get_at_i(), 4);
    });
}

#[test]
fn test_fx55() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0xAF, 0x00,
                   0x60, 0x00,
                   0x61, 0x01,
                   0x62, 0x02,
                   0x63, 0x03,
                   0x64, 0x04,
                   0x65, 0x05,
                   0x66, 0x06,
                   0x67, 0x07,
                   0x68, 0x08,
                   0xF7, 0x55,
                   0xAF, 0x00,
                   0xF1, 0x1E,
                   0xF1, 0x1E,
                   0xF1, 0x1E,
                   0xF1, 0x1E,
                   0xF1, 0x1E,
                   0xF1, 0x1E,
                   0xF1, 0x1E,
                   0xF1, 0x1E];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 0x0F00);
        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 0x0F08);
        cpu.run_cycle();
        assert_eq!(cpu.get_i(), 0x0F00);
        assert_eq!(cpu.get_at_i(), 0);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 1);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 2);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 3);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 4);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 5);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 6);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 7);
        cpu.run_cycle();
        assert_eq!(cpu.get_at_i(), 0);
    });
}

#[test]
fn test_fx65() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0xA0, 0x00,
                   0xF7, 0x65];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        cpu.run_cycle();
        assert_eq!(cpu.get_reg(0), 0xF0);
        assert_eq!(cpu.get_reg(1), 0x90);
        assert_eq!(cpu.get_reg(2), 0x90);
        assert_eq!(cpu.get_reg(3), 0x90);
        assert_eq!(cpu.get_reg(4), 0xF0);
        assert_eq!(cpu.get_reg(5), 0x20);
        assert_eq!(cpu.get_reg(6), 0x60);
        assert_eq!(cpu.get_reg(7), 0x20);
        assert_eq!(cpu.get_reg(8), 0x00);
    });
}

#[test]
fn test_collision() {
    cpu_tester(&mut |cpu, _sender| {
        let rom = [0xD0, 0x01,
                   0xD0, 0x01];
        cpu.load_rom(&rom);

        cpu.run_cycle();
        assert_eq!(cpu.get_carry(), 0x00);

        cpu.run_cycle();
        assert_eq!(cpu.get_carry(), 0x01);
    });
}
