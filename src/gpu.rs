const VRAM_SIZE: usize = 0x2000;
const OAM_RAM_SIZE: usize = 0xa0;

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    oam_ram: [u8; OAM_RAM_SIZE],
    lcdc: u8,
    lcds: u8,
    scy: u8,
    scx: u8,
    ly:u8,
    lyc: u8,
    wy: u8,
    wx: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    dma: u8
}

impl GPU {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam_ram: [0; OAM_RAM_SIZE],
            lcdc: 0,
            lcds: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            dma: 0
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram[(addr - 0x8000) as usize],
            0xfe00..=0xfe9f => self.oam_ram[(addr - 0xfe00) as usize],
            0xff40 => self.lcdc,
            0xff41 => self.lcds,
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff46 => self.dma,
            0xff47 => self.bgp,
            0xff48 => self.obp0,
            0xff49 => self.obp1,
            0xff4a => self.wy,
            0xff4b => self.wx,
            n => panic!("address {:#x} is not handled by gpu", n),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9fff => self.vram[(addr - 0x8000) as usize] = val,
            0xfe00..=0xfe9f => self.oam_ram[(addr - 0xfe00) as usize] = val,
            0xff40 => self.lcdc = val,
            0xff41 => self.lcds = val,
            0xff42 => self.scy = val,
            0xff43 => self.scx = val,
            0xff44 => self.ly = val,
            0xff45 => self.lyc = val,
            0xff46 => self.dma = val,
            0xff47 => self.bgp = val,
            0xff48 => self.obp0 = val,
            0xff49 => self.obp1 = val,
            0xff4a => self.wy = val,
            0xff4b => self.wx = val,
            n => panic!("address {:#x} is not handled by gpu", n),
        }
    }
}
