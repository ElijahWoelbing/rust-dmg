mod cpu;
mod ppu;
mod joypad;
mod mbc;
mod mmu;
mod serial;
mod timer;
mod utills;

extern crate minifb;

use std::env;
use minifb::{Key, Window, WindowOptions};
use cpu::CPU;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing file path")
    }
    let mut cpu = CPU::new(&args[1]);
    run(&mut cpu);
}

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn run(cpu: &mut CPU) {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    loop {
        let clocks_per_frame = 70224;
        let mut clocks_this_update = 0;
        while clocks_this_update < clocks_per_frame {
            clocks_this_update += cpu.do_cycle();
        }
        let buffer = cpu.get_screen_buffer();
        window
        .update_with_buffer(&buffer, WIDTH, HEIGHT)
        .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
