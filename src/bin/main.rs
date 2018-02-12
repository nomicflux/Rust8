extern crate rust8;
extern crate termios;

use std::time;
use std::thread;
use std::thread::sleep;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::Read;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

use rust8::keyboard::{Keyboard, EXIT_CHAR};
use rust8::display::Display;
use rust8::ram::RAM;
use rust8::cpu::CPU;

static BLANK_SCREEN: &'static str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

fn row_to_ascii(row: u64) -> String {
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

fn clear() {
    print!("{}", BLANK_SCREEN);
}

fn draw(screen: &[u64; 32]) {
    clear();

    for i in 0..32 {
        let s = row_to_ascii(screen[i]);
        println!("{}", s);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        writeln!(std::io::stderr(), "Usage: rust8 ROMFILE").unwrap();
        std::process::exit(1);
    }
    let mut file = File::open(&args[1]).expect("Couldn't load ROM file");
    let mut rom = [0u8; 4000 - 0x200];
    file.read(&mut rom).expect("Couldn't read ROM file");

    let (sender, receiver) = channel();

    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);

    let handle = thread::spawn(move || {
        loop {
            tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
            let stdout = io::stdout();
            let reader = io::stdin();
            let mut buffer = [0;1];
            stdout.lock().flush().unwrap();
            let mut input = reader.take(1);
            let size = input.read(&mut buffer).unwrap();
            if size > 0 {
                let _ = sender.send(buffer[0]);
            }
            tcsetattr(stdin, TCSANOW, & termios).unwrap();
            if size > 0 && buffer[0] == EXIT_CHAR as u8 {
                break;
            }
        }
    });

    let mut ram = RAM::init();
    let mut display = Display::init();
    let keyboard = Arc::new(Mutex::new(Keyboard::init(receiver)));
    let mut cpu_keyboard = keyboard.clone();

    let mut cpu = CPU::init(&mut ram, &mut display, &mut cpu_keyboard);
    cpu.load_rom(&rom);

    let hz: f64 = 60.0;
    let time = time::Duration::from_millis((1000.0 / hz).floor() as u64);

    loop {
        cpu.run_cycle();
        draw(&cpu.get_display());
        sleep(time);
        if keyboard.lock().unwrap().exit_key() {
            break;
        }
    }

    handle.join().unwrap();
}
