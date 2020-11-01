const VRAM_SIZE: usize = 0x2000;
const OAM_RAM_SIZE: usize = 0xa0;

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    oam_ram: [u8; OAM_RAM_SIZE],
}

impl GPU {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam_ram: [0; OAM_RAM_SIZE],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram[(addr - 0x8000) as usize],
            0xfe00..=0xfe9f => self.oam_ram[(addr - 0xfe00) as usize],
            _ => panic!("address not handled by gpu"),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9fff => self.vram[(addr - 0x8000) as usize] = val,
            0xfe00..=0xfe9f => self.oam_ram[(addr - 0xfe00) as usize] = val,
            _ => panic!("address not handled by gpu"),
        }
    }
}
