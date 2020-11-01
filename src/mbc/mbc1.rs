use crate::mbc::MBC;

pub struct MBC1 {
    ram_enabled: bool,
    mode_1: bool,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_size: u16,
}

impl MBC for MBC1 {
    fn read_rom(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3fff => self.rom[addr as usize],
            0x4000..=0x7fff => self.rom[(addr - 0x4000) as usize],
            0xa000..=0xbfff => self.rom[(addr - 0xa000) as usize],
            _ => {
                panic! {"address not used by mbc1"}
            }
        }
    }

    fn write_rom(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_enabled = val & 0x0f == 0x0a,
            0x2000..=0x3fff => {
                self.rom_bank = (self.rom_bank & 0x60)
                    | match val & 0x1f {
                        0 => 1,
                        n => n,
                    }
            }
            0x4000..=0x5fff => {
                match self.mode_1 {
                    true => {},
                    false => {
                        /* if number_of_rom_banks is > 31 
                        rom_bank &= (val & 0x3) << 5 else do nothing*/
                    
                    }
                }
            }
            0x6000..=0x7fff => self.mode_1 = val & 0x1 == 1, // mode 0 rom bank, mode 1 ram bank
            _ => (),
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        0
    }

    fn write_ram(&mut self, addr: u16, val: u8) {}
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram_size = match rom[149] {
            0 => 0,
            1 => 0x800,
            2 => 0x2000,
            3 => 0x8000,
            _ => panic!("unknown ram size"),
        };
        Self {
            ram_enabled: false,
            mode_1: false,
            rom,
            ram: vec![0; ram_size as usize],
            rom_bank: 1,
            ram_bank: 0,
            ram_size,
        }
    }
}
