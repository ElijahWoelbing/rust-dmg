mod cpu;
mod gameboy;
mod joypad;
mod mbc;
mod mmu;
mod ppu;
mod serial;
mod timer;
mod utills;
mod minifb_display;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
use gameboy::Gameboy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Missing file path")
    }
    let display = minifb_display::MinifbDisplay::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut gameboy = Gameboy::new(&args[1], display);
    gameboy.emulate();

}