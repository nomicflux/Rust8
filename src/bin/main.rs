extern crate rust8;

use std::time;
use std::thread::sleep;
use std::fs::File;
use std::io::Write;
use std::io::Read;

use rust8::keyboard::Keyboard;
use rust8::display::Display;
use rust8::ram::RAM;
use rust8::cpu::CPU;

fn row_to_ascii(row: u64) -> String {
    let mut s = String::new();

    for i in 0..64 {
        if row & (1 << i) == 1 {
            s.push('*');
        } else {
            s.push(' ');
        }
    }
    s
}

fn clear() {
    for _i in 0..1000 {
        print!("\n");
    }
}

fn draw(screen: &[u64; 32]) {
    clear();

    for i in 0..32 {
        print!("{}\n", row_to_ascii(screen[i]));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        writeln!(std::io::stderr(), "Usage: rust8 ROMFILE").unwrap();
        std::process::exit(1);
    }
    let mut file = File::open(&args[1]).expect("Couldn't load ROM file");
    let mut rom = Vec::<u8>::with_capacity(4000);
    file.read_exact(&mut rom).expect("Couldn't read ROM file");

    let mut ram = RAM::init();
    let mut display = Display::init();
    let mut keyboard = Keyboard::init();

    let mut cpu = CPU::init(&mut ram, &mut display, &mut keyboard);
    cpu.load_rom(&rom);

    let hz: f64 = 60.0;
    let time = time::Duration::from_millis((1000.0 / hz).floor() as u64);

    loop {
        cpu.run_cycle();
        draw(&cpu.get_display());
        sleep(time);
    }
}
