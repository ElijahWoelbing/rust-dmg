use crate::gpu::GPU;
use crate::io::IO;
use crate::mbc;
const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7f;
pub struct MMU {
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    ie: u8,
    mbc: Box<dyn mbc::MBC>,
    gpu: GPU,
    io: IO,
}

impl MMU {
    pub fn new(rom_path: &str) -> Self {
        Self {
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            mbc: mbc::create_mbc(rom_path),
            ie: 0,
            gpu: GPU::new(),
            io: IO::new(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x00..=0x7fff => self.mbc.read_rom(addr),
            0x8000..=0x9fff => self.gpu.read_byte(addr),
            0xa000..=0xbfff => self.mbc.read_ram(addr),
            0xc000..=0xdfff => self.wram[(addr - 0xc000) as usize],
            0xe000..=0xfdff => self.wram[(addr - 0xe000) as usize],
            0xfe00..=0xfe9f => self.gpu.read_byte(addr),
            0xfea0..=0xfeff => 0,
            0xff00..=0xff7f => self.io.read_byte(addr),
            0xff80..=0xfffe => self.hram[(addr - 0xff80) as usize],
            0xffff => self.ie,
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8)
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x00..=0x7fff => self.mbc.write_rom(addr, val),
            0x8000..=0x9fff => self.gpu.write_byte(addr, val),
            0xa000..=0xbfff => self.mbc.write_ram(addr, val),
            0xc000..=0xdfff => self.wram[(addr - 0xc000) as usize] = val,
            0xe000..=0xfdff => self.wram[(addr - 0xe000) as usize] = val,
            0xfe00..=0xfe9f => self.gpu.write_byte(addr, val),
            0xfea0..=0xfeff => (),
            0xff00..=0xff7f => self.io.write_byte(addr, val),
            0xff80..=0xfffe => self.hram[(addr - 0xff80) as usize] = val,
            0xffff => self.ie = val,
        }
    }

    pub fn write_word(&mut self, addr: u16, val: u16) {
        self.write_byte(addr, (val & 0xff) as u8);
        self.write_byte(addr + 1, (val >> 8) as u8);
    }
}
