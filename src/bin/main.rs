extern crate rust8;
extern crate termios;

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time;

use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

use rust8::cpu::CPU;
use rust8::display::Display;
use rust8::displayimpl::{AsciiDisplay, DisplayImpl};
use rust8::keyboard::{Keyboard, EXIT_CHAR};
use rust8::ram::RAM;

// Yes, this is all hideously ugly - in flux as overall design for keyboard / display being put in place, after which the Refactoring will begin.
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

    let handle_keyboard = thread::spawn(move || loop {
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let stdout = io::stdout();
        let reader = io::stdin();
        let mut buffer = [0; 1];
        stdout.lock().flush().unwrap();
        let mut input = reader.take(1);
        let size = input.read(&mut buffer).unwrap();
        if size > 0 {
            let _ = sender.send(buffer[0]);
        }
        tcsetattr(stdin, TCSANOW, &termios).unwrap();
        if size > 0 && buffer[0] == EXIT_CHAR as u8 {
            break;
        }
    });

    let mut ram = RAM::init();
    let display = Arc::new(Mutex::new(Display::init()));
    let keyboard = Arc::new(Mutex::new(Keyboard::init(receiver)));

    let mut cpu_display = display.clone();
    let mut cpu_keyboard = keyboard.clone();
    let display_keyboard = keyboard.clone();

    let mut logfile = File::create("opcode_logfile.txt").unwrap();

    let mut cpu = CPU::init(&mut ram, &mut cpu_display, &mut cpu_keyboard, &mut logfile);
    cpu.load_rom(&rom);

    let display_hz: f64 = 60.0;
    let display_time = time::Duration::from_millis((1000.0 / display_hz).floor() as u64);

    let handle_display = thread::spawn(move || {
        let ascii_display = AsciiDisplay();
        loop {
            ascii_display.draw(&display.lock().unwrap(), &display_keyboard.lock().unwrap());
            sleep(display_time);
        }
    });

    let mut time = time::SystemTime::now();
    loop {
        keyboard.lock().unwrap().read_input();
        cpu.run_cycle();
        if keyboard.lock().unwrap().exit_key() {
            break;
        }
        if time::SystemTime::now() > time + display_time {
            cpu.dec_delay();
            time += display_time;
        }
    }

    handle_keyboard.join().unwrap();
    handle_display.join().unwrap();
}
