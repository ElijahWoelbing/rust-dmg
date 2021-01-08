use crate::ppu::PPU;
use crate::joypad::Joypad;
use crate::mbc;
use crate::serial::Serial;
use crate::timer::Timer;

const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7f;
pub struct MMU {
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
    mbc: Box<dyn mbc::MBC>,
    pub ppu: PPU,
    timer: Timer,
    pub joypad: Joypad,
    serial: Serial,
}

impl MMU {
    pub fn new(cart_path: &str) -> Self {
        let mut mmu = Self {
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            mbc: mbc::create_mbc(cart_path),
            interrupt_enable: 0,
            interrupt_flag: 0,
            ppu: PPU::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),
        };
        mmu.initialize_memory();
        mmu  
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x00..=0x7fff => self.mbc.read_rom(address),
            0x8000..=0x9fff => self.ppu.read_byte(address),
            0xa000..=0xbfff => self.mbc.read_ram(address), // external ram
            0xc000..=0xdfff => self.wram[(address - 0xc000) as usize], // work ram
            0xe000..=0xfdff => self.wram[(address - 0xe000) as usize], // echo ram
            0xfe00..=0xfe9f => self.ppu.read_byte(address), // oam ram
            0xfea0..=0xfeff => 0,                     // usable
            0xff00 => self.joypad.read_byte(),        // joypad
            0xff01..=0xff02 => self.serial.read_byte(address), // serial
            0xff03 => 0,
            0xff04..=0xff07 => self.timer.read_byte(address),
            0xff08..=0xff0e => 0,
            0xff0f => self.interrupt_flag, // interrupt flag
            0xff10..=0xff3f => 0,          // sound
            0xff40..=0xff4b => self.ppu.read_byte(address), // lcd registers
            0xff4c..=0xff7f => 0,
            0xff80..=0xfffe => self.hram[(address - 0xff80) as usize], // high ram
            0xffff => self.interrupt_enable,
            // _ => 0
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x00..=0x7fff => self.mbc.write_rom(address, value), // 32kb rom bank 00
            0x8000..=0x9fff => self.ppu.write_byte(address, value),
            0xa000..=0xbfff => self.mbc.write_ram(address, value), // external ram
            0xc000..=0xdfff => self.wram[(address - 0xc000) as usize] = value, // work ram
            0xe000..=0xfdff => self.wram[(address - 0xe000) as usize] = value, // echo ram
            0xfe00..=0xfe9f => self.ppu.write_byte(address, value), // oam ram
            0xfea0..=0xfeff => (),                          // usable
            0xff00 => self.joypad.write_byte(value),          // joypad
            0xff01..=0xff02 => self.serial.write_byte(address, value), // serial
            0xff03 => (),                                   // nothing
            0xff04..=0xff07 => self.timer.write_byte(address, value),
            0xff08..=0xff0e => (),                             // nothing
            0xff0f => self.interrupt_flag = value,               // interrupt flag
            0xff10..=0xff3f => (),                             // sound
            0xff40..=0xff45 =>self.ppu.write_byte(address, value),
            0xff46 => self.dma_transfer(value),
            0xff47..=0xff4b => self.ppu.write_byte(address, value),
            0xff4c..=0xff7f => (),                             // nothing
            0xff80..=0xfffe => self.hram[(address - 0xff80) as usize] = value, // high ram
            0xffff => self.interrupt_enable = value,
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xff) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    pub fn tick(&mut self, clocks: u32){
        self.ppu.tick(clocks);
        self.interrupt_flag |= self.ppu.interrupt;
        self.timer.tick(clocks);
        self.interrupt_flag |= self.timer.interrupt;
        self.interrupt_flag |= self.joypad.interrupt;
    }
    // inital state after checksum
    fn initialize_memory(&mut self) {
        self.write_byte(0xff10, 0x80); // NR10
        self.write_byte(0xff11, 0xbf); // NR11
        self.write_byte(0xff12, 0xf3); // NR12
        self.write_byte(0xff14, 0xbf); // NR14
        self.write_byte(0xff16, 0x3f); // NR21
        self.write_byte(0xff19, 0xbf); // NR24
        self.write_byte(0xff1a, 0x7f); // NR30
        self.write_byte(0xff1b, 0xff); // NR31
        self.write_byte(0xff1c, 0x9f); // NR32
        self.write_byte(0xff1e, 0xbf); // NR34
        self.write_byte(0xff20, 0xff); // NR41
        self.write_byte(0xff23, 0xbf); // NR44
        self.write_byte(0xff24, 0x77); // NR50
        self.write_byte(0xff25, 0xf3); // NR51
        self.write_byte(0xff26, 0xf1); // NR52
        self.write_byte(0xff40, 0x91); // LCDC
        self.write_byte(0xff47, 0xfc); // BGP
        self.write_byte(0xff48, 0xff); // OBP0
        self.write_byte(0xff49, 0xff); // OBP1
    }

    fn dma_transfer(&mut self, data: u8) {
        let address = (data as u16) << 8 ; 
        for i in 0x0..0xA0
        {
          self.ppu.dma_transfer(i, self.read_byte(address+i));
        }
    }

}
