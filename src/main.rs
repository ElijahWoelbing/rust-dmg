mod cpu;
mod mmu;
mod gpu;
mod mbc;
mod timer;
mod joypad;
mod serial;
mod utils;

// use std::env;


fn main(){
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {panic!("Missing file path")}
    let mut cpu = cpu::CPU::new("./test_roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb");
    cpu.tick();
}
