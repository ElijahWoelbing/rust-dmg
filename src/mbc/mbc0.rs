use crate::mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC for MBC0 {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_rom(&mut self, address: u16, value: u8) { 
        () // not used
    }

    fn read_ram(&self, address: u16) -> u8 {
        0 // not used    
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        () // not used
    }
}

impl MBC0 {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }
}
