mod cpu;
mod mmu;
mod gpu;
mod mbc;
mod io;
mod helpers;

// use std::env;


fn main(){
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {panic!("Missing file path")}
    let mut cpu = cpu::CPU::new("./test_roms/cpu_instrs/individual/01-special.gb");
    cpu.tick();
}
