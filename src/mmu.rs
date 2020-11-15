use crate::gpu::GPU;
use crate::timer::Timer;
use crate::joypad::Joypad;
use crate::serial::Serial;
use crate::mbc;

const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7f;
pub struct MMU {
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
    mbc: Box<dyn mbc::MBC>,
    gpu: GPU,
    timer: Timer,
    joypad: Joypad,
    serial: Serial,
}

impl MMU {
    pub fn new(cart_path: &str) -> Self {
        Self {
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            mbc: mbc::create_mbc(cart_path),
            interrupt_enable: 0,
            interrupt_flag: 0,
            gpu: GPU::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x00..=0x7fff => {self.mbc.read_rom(addr)}, // 16kb rom bank 00
            0x8000..=0x9fff => self.gpu.read_byte(addr), // 16kb rom bank 01-nn
            0xa000..=0xbfff => self.mbc.read_ram(addr), // external ram
            0xc000..=0xdfff => self.wram[(addr - 0xc000) as usize], // work ram
            0xe000..=0xfdff => self.wram[(addr - 0xe000) as usize], // echo ram
            0xfe00..=0xfe9f => self.gpu.read_byte(addr), // oam ram
            0xfea0..=0xfeff => 0, // usable
            0xff00 => self.joypad.read_byte(), // joypad
            0xff01..=0xff02 => self.serial.read_byte(addr), // serial
            0xff03 => 0,
            0xff04..=0xff07 => self.timer.read_byte(addr),
            0xff08..=0xff0e => 0,
            0xff0f => self.interrupt_flag, // interrupt flag
            0xff10..=0xff3f => 0, // sound
            0xff40..=0xff4b => self.gpu.read_byte(addr), // lcd registers
            0xff4c..= 0xff7f => 0,
            0xff80..=0xfffe => self.hram[(addr - 0xff80) as usize], // high ram
            0xffff => self.interrupt_enable,
            // _ => 0
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8)
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x00..=0x7fff => self.mbc.write_rom(addr, val), // 16kb rom bank 00
            0x8000..=0x9fff => self.gpu.write_byte(addr, val), // 16kb rom bank 01-nn
            0xa000..=0xbfff => self.mbc.write_ram(addr, val), // external ram
            0xc000..=0xdfff => self.wram[(addr - 0xc000) as usize] = val, // work ram
            0xe000..=0xfdff => self.wram[(addr - 0xe000) as usize] = val, // echo ram
            0xfe00..=0xfe9f => self.gpu.write_byte(addr, val), // oam ram
            0xfea0..=0xfeff => (), // usable
            0xff00 => self.joypad.write_byte(val), // joypad
            0xff01..=0xff02 => self.serial.write_byte(addr, val), // serial
            0xff03 => (), // nothing
            0xff04..=0xff07 => self.timer.write_byte(addr, val),
            0xff08..=0xff0e => (), // nothing 
            0xff0f => self.interrupt_flag = val, // interrupt flag
            0xff10..=0xff3f => (), // sound
            0xff40..=0xff4b => self.gpu.write_byte(addr, val), // lcd registers
            0xff4c..= 0xff7f => (), // nothing
            0xff80..=0xfffe => self.hram[(addr - 0xff80) as usize] = val, // high ram
            0xffff => self.interrupt_enable = val,
        }
    }

    pub fn write_word(&mut self, addr: u16, val: u16) {
        self.write_byte(addr, (val & 0xff) as u8);
        self.write_byte(addr + 1, (val >> 8) as u8);
    }
}
