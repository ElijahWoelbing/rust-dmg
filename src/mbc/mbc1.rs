use crate::mbc::MBC;

pub struct MBC1 {
    ram_enabled: bool,
    ram_mode: bool,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
}

impl MBC for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => {self.rom[address as usize]}, // bank 00
            0x4000..=0x7fff => {self.rom[self.rom_bank * 0x4000 + (address as usize - 0x4000)]}, // switchable bank
            _ => {
                panic! {"address not used by mbc1"}
            }
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1fff => self.ram_enabled = value & 0x0f == 0x0a,
            0x2000..=0x3fff => {
                self.rom_bank = (self.rom_bank & 0x60)
                    | match value as usize & 0x1f {
                        0 => 1,
                        n => n,
                    }
            }
            0x4000..=0x5fff => match self.ram_mode {
                true => {
                    self.ram_bank = value as usize & 0x3;
                }
                false => self.rom_bank = (self.rom_bank & 0x1f) | ((value as usize & 0x3) << 5),
            },
            0x6000..=0x7fff => self.ram_mode = value & 0x1 == 1, // mode 0 rom bank, mode 1 ram bank
            _ => (),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        // if ram enabled, if ram mode and uses ram banking use ram bank number else use 00 bank
        if self.ram_enabled {
            let ram_bank = if self.ram_mode { self.ram_bank } else { 0 };
            self.ram[ram_bank * 0x2000 + (address as usize - 0xa000)]
        } else {
            0
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled {
            let ram_bank = if self.ram_mode { self.ram_bank } else { 0 };
            self.ram[ram_bank as usize * 0x2000 + (address as usize - 0xa000)] = value;
        }
    }
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram_size = ram_size(rom[149]);
        Self {
            ram_enabled: false,
            ram_mode: false,
            rom,
            ram: vec![0; ram_size as usize],
            rom_bank: 1,
            ram_bank: 0,
        }
    }
}

fn ram_size(value: u8) -> u32 {
    match value {
        0 => 0,
        1 => 0x800,
        2 => 0x2000,
        3 => 0x8000,
        _ => panic!("unsupported ram size"),
    }
}
