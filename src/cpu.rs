use crate::mmu::MMU;
pub struct CPU {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    sp: u16,
    pc: u16,
    ime: bool,
    mmu: MMU,
}

enum Flag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xfffe,
            ime: false,
            mmu: MMU::new(),
        }
    }

    fn read_af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xf0) as u16)
    }

    fn read_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn read_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    fn read_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn write_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xf0) as u8;
    }

    fn write_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xff) as u8;
    }

    fn write_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xff) as u8;
    }

    fn write_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xff) as u8;
    }

    fn hli(&mut self) -> u16 {
        let hl = self.read_hl();
        self.write_hl(self.read_hl() + 1);
        hl
    }

    fn hld(&mut self) -> u16 {
        let hl = self.read_hl();
        self.write_hl(self.read_hl() - 1);
        hl
    }

    fn flag(&mut self, flag: Flag, set: bool) {
        let mask = flag as u8;
        if set {
            self.f |= mask;
        } else {
            self.f &= !mask
        }
        self.f &= 0xf0;
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let mask = flag as u8;
        self.f & mask == mask
    }

    pub fn execute_opcode(&mut self) -> u32 {
        let opcode: u8 = self.fetch_byte();
        match opcode {
            0x00 => 4,
            0x01 => {
                let d16 = self.fetch_word();
                self.write_bc(d16);
                12
            }
            0x02 => {
                self.mmu.write_byte(self.read_bc(), self.a);
                8
            }
            0x03 => {
                self.write_bc(self.read_bc() + 1);
                8
            }
            0x04 => {
                self.b = self.inc(self.b);
                4
            }
            0x05 => {
                self.b = self.dec(self.b);
                4
            }
            0x06 => {
                self.b = self.fetch_byte();
                8
            }
            0x07 => {
                self.a = self.rlc(self.a);
                self.flag(Flag::Z, false);
                4
            }
            0x08 => {
                let a16 = self.fetch_word();
                self.mmu.write_word(a16, self.sp);
                20
            }
            0x09 => {
                self.add16(self.read_bc());
                8
            }
            0x0a => {
                self.a = self.mmu.read_byte(self.read_bc());
                8
            }
            0x0b => {
                self.write_bc(self.read_bc() - 1);
                8
            }
            0x0c => {
                self.c = self.inc(self.c);
                4
            }
            0x0d => {
                self.c = self.dec(self.c);
                4
            }
            0xe => {
                self.c = self.fetch_byte();
                8
            }
            0xf => {
                self.a = self.rrc(self.a);
                self.flag(Flag::Z, false);
                4
            }
            0x10 => 4,
            0x11 => {
                let d16 = self.fetch_word();
                self.write_de(d16);
                12
            }
            0x12 => {
                self.mmu.write_byte(self.read_de(), self.a);
                8
            }
            0x13 => {
                self.write_de(self.read_de() + 1);
                8
            }
            0x14 => {
                self.d = self.inc(self.d);
                4
            }
            0x15 => {
                self.d = self.dec(self.d);
                4
            }
            0x16 => {
                self.d = self.fetch_byte();
                8
            }
            0x17 => {
                self.a = self.rl(self.a);
                self.flag(Flag::Z, false);
                4
            }
            0x18 => {
                self.jr();
                12
            }
            0x19 => {
                self.add16(self.read_de());
                8
            }
            0x1a => {
                self.a = self.mmu.read_byte(self.read_de());
                8
            }
            0x1b => {
                self.write_de(self.read_de() - 1);
                8
            }
            0x1c => {
                self.e = self.inc(self.e);
                4
            }
            0x1d => {
                self.e = self.dec(self.e);
                4
            }
            0x1e => {
                self.e = self.fetch_byte();
                8
            }
            0x1f => {
                self.a = self.rr(self.a);
                self.flag(Flag::Z, false);
                4
            }
            0x20 => {
                if !self.get_flag(Flag::Z) {
                    self.jr();
                    12
                } else {
                    self.pc += 1;
                    8
                }
            }
            0x21 => {
                let d16 = self.fetch_word();
                self.write_de(d16);
                12
            }
            0x22 => {
                let hl = self.hli();
                self.mmu.write_byte(hl, self.a);
                8
            }
            0x23 => {
                self.write_hl(self.read_hl() + 1);
                8
            }
            0x24 => {
                self.h = self.inc(self.h);
                8
            }
            0x25 => {
                self.h = self.dec(self.h);
                8
            }
            0x26 => {
                self.h = self.fetch_byte();
                8
            }
            0x27 => 0,
            0x28 => {
                if self.get_flag(Flag::Z) {
                    self.jr();
                    12
                } else {
                    self.pc += 1;
                    8
                }
            }
            0x29 => {
                self.add16(self.read_hl());
                8
            }
            0x2a => {
                let hl = self.hli();
                self.a = self.mmu.read_byte(hl);
                8
            }
            0x2b => {
                self.write_hl(self.read_hl() - 1);
                8
            }
            0x2c => {
                self.l = self.inc(self.l);
                4
            }
            0x2d => {
                self.l = self.dec(self.l);
                4
            }
            0x2e => {
                self.l = self.fetch_byte();
                8
            }
            0x2f => {
                self.a = !self.a;
                self.flag(Flag::H, true);
                self.flag(Flag::N, true);
                4
            }
            0x30 => {
                if self.get_flag(Flag::C) {
                    self.jr();
                    12
                } else {
                    self.pc += 1;
                    8
                }
            }
            0x31 => {
                self.sp = self.fetch_word();
                12
            }
            0x32 => {
                let hl = self.hld();
                self.mmu.write_byte(hl, self.a);
                8
            }
            0x33 => {
                self.sp += 1;
                8
            }
            0x34 => {
                let address = self.read_hl();
                let val = self.mmu.read_byte(address);
                let incremented = self.inc(val);
                self.mmu.write_byte(address, incremented);
                12
            }
            0x35 => {
                let address = self.read_hl();
                let val = self.mmu.read_byte(address);
                let decremented = self.dec(val);
                self.mmu.write_byte(address, decremented);
                12
            }
            _ => 0,
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.read_byte(self.pc);
        self.pc += 1;
        byte
    }
    fn fetch_word(&mut self) -> u16 {
        let word = self.mmu.read_word(self.pc);
        self.pc += 2;
        word
    }

    fn inc(&mut self, val: u8) -> u8 {
        let val_inc = val.wrapping_add(1);
        self.flag(Flag::Z, val_inc == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, (val_inc & 0xF) > 0x10);
        val_inc
    }

    fn dec(&mut self, val: u8) -> u8 {
        let val_dec = val.wrapping_sub(1);
        self.flag(Flag::Z, val_dec == 0);
        self.flag(Flag::N, true);
        self.flag(Flag::H, (val & 0xF) == 0);
        val_dec
    }

    // rotates and shifts
    fn rotate_left(&self, val: u8) -> u8 {
        ((val << 1) | (val >> 7)) & 0xff
    }

    fn rotate_right(&self, val: u8) -> u8 {
        ((val >> 1) | (val << 7)) & 0xff
    }

    fn rl(&mut self, val: u8) -> u8 {
        let rotate = self.rotate_left(val) | if self.get_flag(Flag::C) { 1 } else { 0 };
        self.flag(Flag::Z, rotate == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, false);
        self.flag(Flag::C, val >= 0x80);
        rotate
    }

    fn rlc(&mut self, val: u8) -> u8 {
        let rotate = self.rotate_left(val);
        self.flag(Flag::Z, rotate == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, false);
        self.flag(Flag::C, val >= 0x80);
        rotate
    }

    fn rr(&mut self, val: u8) -> u8 {
        let rotate = self.rotate_left(val) | if self.get_flag(Flag::C) { 0x80 } else { 0 };
        self.flag(Flag::Z, rotate == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, false);
        self.flag(Flag::C, val & 0x1 == 0x1);
        rotate
    }

    fn rrc(&mut self, val: u8) -> u8 {
        let rotate = self.rotate_right(val);
        self.flag(Flag::Z, rotate == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, false);
        self.flag(Flag::C, val & 0x1 == 0x1);
        rotate
    }
    // 16 bit addition
    fn add16(&mut self, val: u16) {
        let hl = self.read_hl();
        self.flag(Flag::N, false);
        self.flag(Flag::H, (hl & 0xfff) + (val & 0xfff) > 0xfff);
        self.flag(Flag::C, hl > 0xffff - val);
        self.write_hl(hl.wrapping_add(val));
    }
    // jumps
    fn jr(&mut self) {
        let r8 = self.fetch_byte() as i8;
        self.pc = self.pc.wrapping_add(r8 as u16);
    }
}
