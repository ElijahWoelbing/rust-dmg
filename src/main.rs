mod cpu;
mod ppu;
mod joypad;
mod mbc;
mod mmu;
mod serial;
mod timer;
mod utills;
mod lcd;


use std::env;
use cpu::CPU;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing file path")
    }
    let mut cpu = CPU::new(&args[1]);
    loop {
        let clocks_per_frame = 70224;
        let mut clocks_this_update = 0;
        while clocks_this_update < clocks_per_frame {
            clocks_this_update += cpu.do_cycle();
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}


