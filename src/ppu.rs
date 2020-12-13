use crate::utills::{check_bit, get_bit_value};
const VRAM_SIZE: usize = 0x2000;
const OAM_RAM_SIZE: usize = 0xa0;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
#[derive(Clone, Copy)]
enum Mode {
    HBlank,
    VBlank,
    OAMSearch,
    LCDTransfer,
}

use Mode::{HBlank, LCDTransfer, OAMSearch, VBlank};
pub struct PPU {
    vram: [u8; VRAM_SIZE],
    oam_ram: [u8; OAM_RAM_SIZE],
    pub screen_data: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    lcdc: u8,
    lcds: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    wy: u8,
    wx: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    dma: u8,
    mode: Mode,
    clocks: u32,
    pub status_interrupt: u8,
    pub vblank_interrupt: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam_ram: [0; OAM_RAM_SIZE],
            screen_data: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
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
            dma: 0,
            mode: OAMSearch,
            clocks: 0,
            status_interrupt: 0,
            vblank_interrupt: 0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9fff => self.vram[(address - 0x8000) as usize], // TODO Restrict access in mode 3
            0xfe00..=0xfe9f => self.oam_ram[(address - 0xfe00) as usize], // TODO Restrict access in mode 2,3
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

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9fff => self.vram[(address - 0x8000) as usize] = value, // TODO
            0xfe00..=0xfe9f => self.oam_ram[(address - 0xfe00) as usize] = value, // TODO
            0xff40 => self.lcdc = value,
            0xff41 => self.lcds = value,
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff44 => self.ly = value,
            0xff45 => self.lyc = value,
            0xff46 => self.dma = value,
            0xff47 => self.bgp = value,
            0xff48 => self.obp0 = value,
            0xff49 => self.obp1 = value,
            0xff4a => self.wy = value,
            0xff4b => self.wx = value,
            _ => panic!("address {:#x} is not handled by gpu", address),
        }
    }

    pub fn tick(&mut self, clocks: u32) {
        // reset interrrupts
        self.status_interrupt = 0;
        self.vblank_interrupt = 0;
        self.clocks += clocks;
        match self.mode {
            // 80 dots
            OAMSearch => {
                if self.clocks >= 80 {
                    self.clocks -= 80;
                    self.set_mode(LCDTransfer);
                }
            }
            // 168 to 291 dots
            LCDTransfer => {
                if self.clocks >= 172 {
                    self.clocks -= 172;
                    self.set_mode(HBlank);
                    self.buffer_scanline();
                }
            }
            // 85 to 208 dots
            HBlank => {
                if self.clocks >= 204 {
                    self.clocks -= 204;
                    self.update_ly();
                    if self.ly == 144 {
                        self.set_mode(VBlank);
                        self.vblank_interrupt = 0x1;
                    } else {
                        self.set_mode(OAMSearch);
                    }
                }
            }
            // 4560 dots
            VBlank => {
                if self.clocks >= 456 {
                    self.clocks -= 456;
                    self.update_ly();
                    if self.ly > 153 {
                        self.ly = 0;
                        self.set_mode(OAMSearch);
                    }
                }
            }
        }
    }

    fn update_ly(&mut self) {
        self.ly += 1;
        if self.ly == self.lyc {
            self.lcds |= 0x4; // set coincidence Flag
            if check_bit(self.lcds, 6) {
                // is coincidence interrupt enabled
                self.status_interrupt = 0x2;
            }
        } else {
            self.lcds &= 0xfb; // reset coincidence flag
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.lcds &= 0xfc;
        self.lcds |= mode as u8;

        match mode {
            HBlank => self.lcds |= 0x8,
            VBlank => self.lcds |= 0x10,
            OAMSearch => self.lcds |= 0x20,
            LCDTransfer => return,
        }
        // raise status fl in if
        self.status_interrupt = 0x2;
    }

    fn buffer_scanline(&mut self) {
        self.buffer_tiles();
    }

    fn buffer_tiles(&mut self) {
        let ly = self.ly as u16;
        let scx = self.scx as u16;
        let scy = self.scy as u16;
        let wx = self.wx.wrapping_sub(7) as u16;
        let wy = self.wy as u16;


        if check_bit(self.lcdc, 0) {
            let mut using_window = false;

            if check_bit(self.lcdc, 5) {
                if wy <= ly {
                    using_window = true;
                }
            }

            // which tile data are we using?
            let tile_base_pointer = match check_bit(self.lcdc, 4) {
                true => 0x8000,
                false => 0x9000,
            };

            // 32 * 32 byte gird, contains tile numbers
            let background_map: u16;
            
            if using_window {
                background_map = match check_bit(self.lcdc, 6) {
                    true => 0x9C00,
                    false => 0x9800,
                };
            } else {
                background_map = match check_bit(self.lcdc, 3) {
                    true => 0x9C00,
                    false => 0x9800,
                };
            }

            // current relitive scanline
            let y_pos = match using_window {
                true => wy + ly,
                false => scy + ly,
            };

            // the row that the tile number is on
            let tile_row = y_pos / 8;
            for pixel in 0..160 {
                // current pixel
                let mut x_pos = pixel + scx;
                if using_window {
                    if pixel >= wx {
                        x_pos = pixel - wx;
                    }
                }
                // the column the tile number is on
                let tile_col = x_pos / 8;

                let tile_number_address = background_map + tile_row + tile_col;
                // tile number can be signed or unsigned
                let tile_number = self.read_byte(tile_number_address);
                // pointer to first byte of tile
                let tile_location = match tile_base_pointer {
                    0x9000 => (tile_base_pointer as i32 + (tile_number as i8 as i16 as i32)) as u16,
                    0x8000 => tile_base_pointer + (tile_number as u16 * 16),
                    _ => unreachable!(),
                };

                // find the correct vertical line we're on of the
                // tile to get the tile data
                // from in memory
                let line = (y_pos % 8) * 2;
                let data1 = self.read_byte(tile_location + line);
                let data2 = self.read_byte(tile_location + line + 1);

                // For each line, the first byte defines the least significant bits of the color numbers
                // for each pixel, and the second byte defines the upper bits of the color numbers.
                // In either case, Bit 7 is the leftmost pixel, and Bit 0 the rightmost.
                let mut color_bit = x_pos as i32 % 8;
                color_bit -= 7;
                color_bit *= -1;
                let mut color_number = get_bit_value(data2, color_bit as u8) << 1;
                color_number |= get_bit_value(data1, color_bit as u8);
                let color = match self.map_color(color_number) {
                    0 => 0xFFFFFF, // white
                    1 => 0xD3D3D3, // light grey
                    2 => 0xA9A9A9, // dark grey
                    3 => 0x0, // black 
                    _ => unreachable!()
                };
                self.screen_data[self.ly as usize + pixel as usize] = color;
            }
        }
    }

    fn map_color(&self, color_number: u8) -> u8 {
        let mut hi_bit = 0;
        let mut lo_bit = 0;

        match color_number {
            0 => {
                hi_bit = 1;
                lo_bit = 0;
            }
            1 => {
                hi_bit = 3;
                lo_bit = 2;
            }
            2 => {
                hi_bit = 5;
                lo_bit = 4;
            }
            3 => {
                hi_bit = 7;
                lo_bit = 6;
            }
            _ => unreachable!(),
        }

        let mut color = 0;
        color = get_bit_value(self.bgp, hi_bit) << 1;
        color |= get_bit_value(self.bgp, lo_bit);

        color
    }
}
