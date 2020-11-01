use crate::mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC for MBC0 {
    fn read_byte(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, val: u8) {/*not used for mbc0*/}
}

impl MBC0 {
    pub fn new(rom_buffer: Vec<u8>) -> Self {
        Self { rom: rom_buffer }
    }
}
