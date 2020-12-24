use crate::lcd::LCD;
use crate::utills::{check_bit, get_bit_value};
const VRAM_SIZE: usize = 0x2000;
const OAM_RAM_SIZE: usize = 0xa0;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    HBlank,
    VBlank,
    OAMSearch,
    LCDTransfer,
}
use Mode::{HBlank, LCDTransfer, OAMSearch, VBlank};
#[derive(PartialEq)]
enum Color {
    White = 0x9bbc0f,
    LightGrey = 0x8bac0f,
    DarkGrey = 0x306230,
    Black = 0x0f380f,
}

use Color::{Black, DarkGrey, LightGrey, White};

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    oam_ram: [u8; OAM_RAM_SIZE],
    pub screen_data: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
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
    mode: Mode,
    clocks: u32,
    lcd: LCD,
    pub status_interrupt: u8,
    pub vblank_interrupt: u8,
    display_enabled: bool,
    window_tilemap: u16,
    window_enabled: bool,
    tilebase: u16,
    background_tilemap: u16,
    sprite_size: u8,
    sprites_enabled: bool,
    bg_win_priority: bool,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam_ram: [0; OAM_RAM_SIZE],
            screen_data: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
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
            mode: OAMSearch,
            clocks: 0,
            lcd: LCD::new(),
            status_interrupt: 0,
            vblank_interrupt: 0,
            display_enabled: false,
            window_tilemap: 0x1c00,
            window_enabled: false,
            tilebase: 0,
            background_tilemap: 0x1c00,
            sprite_size: 8,
            sprites_enabled: false,
            bg_win_priority: false,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9fff => self.vram[(address - 0x8000) as usize], // TODO Restrict access in mode 3
            0xfe00..=0xfe9f => self.oam_ram[(address - 0xfe00) as usize],
            0xff40 => {
                (if self.display_enabled { 0x80 } else { 0 })
                    | (if self.window_tilemap == 0x1C00 {
                        0x40
                    } else {
                        0
                    })
                    | (if self.window_enabled { 0x20 } else { 0 })
                    | (if self.tilebase == 0 { 0x10 } else { 0 })
                    | (if self.background_tilemap == 0x1C00 {
                        0x08
                    } else {
                        0
                    })
                    | (if self.sprite_size == 16 { 0x04 } else { 0 })
                    | (if self.sprites_enabled { 0x02 } else { 0 })
                    | (if self.bg_win_priority { 0x01 } else { 0 })
            }
            0xff41 => self.lcds,
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff46 => 0,
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
            0x8000..=0x9fff => self.vram[(address - 0x8000) as usize] = value,
            0xfe00..=0xfe9f => self.oam_ram[(address - 0xfe00) as usize] = value,
            0xff40 => {
                self.display_enabled = value & 0x80 == 0x80;
                self.window_tilemap = if value & 0x40 == 0x40 { 0x1C00 } else { 0x1800 };
                self.window_enabled = value & 0x20 == 0x20;
                self.tilebase = if value & 0x10 == 0x10 { 0 } else { 0x1000 };
                self.background_tilemap = if value & 0x08 == 0x08 { 0x1C00 } else { 0x1800 };
                self.sprite_size = if value & 0x04 == 0x04 { 16 } else { 8 };
                self.sprites_enabled = value & 0x02 == 0x02;
                self.bg_win_priority = value & 0x01 == 0x01;
            }
            0xff41 => self.lcds = value,
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff44 => self.ly = value,
            0xff45 => self.lyc = value,
            0xff46 => unreachable!("dma transfer"),
            0xff47 => self.bgp = value,
            0xff48 => self.obp0 = value,
            0xff49 => self.obp1 = value,
            0xff4a => self.wy = value,
            0xff4b => self.wx = value,
            _ => panic!("address {:#x} is not handled by gpu", address),
        }
    }

    pub fn tick(&mut self, clocks: u32) {
        // TODO account for variable number of dot clocks
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
                        self.lcd
                            .window
                            .update_with_buffer(&self.screen_data, SCREEN_WIDTH, SCREEN_HEIGHT)
                            .unwrap();
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
        self.scanline_background();
        self.scanline_sprites();
    }

    fn scanline_background(&mut self) {
        // TODO less casting
        if !self.display_enabled {
            return;
        }

        let bg_y = self.scy.wrapping_add(self.ly);
        let bg_tile_row = (bg_y / 8) as u16 * 32;
        for pixel in 0..SCREEN_WIDTH {
            let x = self.scx.wrapping_add(pixel as u8);
            let bg_tile_col = (x / 8) as u16;
            let tile_num =
                self.vram[(self.background_tilemap + bg_tile_row + bg_tile_col) as usize];
            let data_address = match self.tilebase {
                0x0000 => self.tilebase + (tile_num as u16 * 16),
                0x1000 => (self.tilebase as i32 + (tile_num as i8 as i16 as i32 * 16)) as u16,
                _ => unreachable!("not a valid tile base"),
            };

            let line = ((bg_y % 8) * 2) as u16;
            let line_byte1 = self.vram[(data_address + line) as usize];
            let line_byte2 = self.vram[(data_address + line + 1) as usize];
            let color_bit = 7 - (x % 8);
            let mut color_number = get_bit_value(line_byte2, color_bit) << 1;
            color_number |= get_bit_value(line_byte1, color_bit);
            let color = self.map_color_pattel(color_number, self.bgp);
            self.screen_data[self.ly as usize * SCREEN_WIDTH as usize + pixel] = color as u32;
        }
    }

    fn scanline_sprites(&mut self) {
        if !self.sprites_enabled {
            return;
        };
        // search through oam for visable sprites
        let bytes = self.oam_ram.chunks(4);
        for byte in bytes {
            let sprite_y = byte[0] as i32 - 16;
            let sprite_x = byte[1] as i32 - 8;
            // sprite not visable
            if sprite_y == -16 || sprite_y > 143 || sprite_x == -8 || sprite_x > 159 {
                continue;
            }
            let sprite_size = self.sprite_size;
            // does sprite intercept scanline
            let ly = self.ly as i32;
            if sprite_y > ly || sprite_y + sprite_size as i32 <= ly {
                continue;
            }
            let tile_number = byte[2] as u16;
            let attributes = byte[3];
            let y_flip = check_bit(attributes, 6);
            let x_flip = check_bit(attributes, 5);
            let palette = match check_bit(attributes, 4) {
                true => self.obp1,
                false => self.obp0,
            };

            let mut line = (ly - sprite_y) as u16;
            if y_flip {
                line = (self.sprite_size as u16 - 1) - line;
            }
            line *= 2;
            let data1 = self.vram[((tile_number * 16) + line) as usize];
            let data2 = self.vram[((tile_number * 16) + line + 1) as usize];

            for pixel in 0..8 {
                let mut tile_pixel = pixel;
                if !x_flip {
                    tile_pixel = 7 - pixel;
                }
                let mut color_num = get_bit_value(data2, tile_pixel) << 1;
                color_num |= get_bit_value(data1, tile_pixel);

                let color = self.map_color_pattel(color_num, palette);
                // transparent
                if color == White {
                    continue;
                }

                let x_pix = sprite_x + pixel as i32;
                let current_pixel_address = self.ly as usize * SCREEN_WIDTH + x_pix as usize;
                // if bit is set priority is behind 1-3
                if check_bit(attributes, 7) {
                    let current_pixel_color = self.screen_data[current_pixel_address];
                    if current_pixel_color == 0x8bac0f
                        || current_pixel_color == 0x306230
                        || current_pixel_color == 0x0f380f
                    {
                        continue;
                    }
                }
                self.screen_data[current_pixel_address] = color as u32;
            }
        }
    }

    fn map_color_pattel(&self, color_number: u8, palette: u8) -> Color {
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
        color = get_bit_value(palette, hi_bit) << 1;
        color |= get_bit_value(palette, lo_bit);

        match color {
            0 => White,
            1 => LightGrey,
            2 => DarkGrey,
            _ => Black,
        }
    }

    pub fn dma_transfer(&mut self, address: u16, value: u8) {
        self.oam_ram[address as usize] = value;
    }
}
