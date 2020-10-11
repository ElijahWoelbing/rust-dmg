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
        self.f = (val & 0xf0) as u8; // only upper nibble used
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
                let address = self.hli();
                self.mmu.write_byte(address, self.a);
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
                let address = self.hld();
                self.mmu.write_byte(address, self.a);
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
            0x36 => {
                let d8 = self.fetch_byte();
                self.mmu.write_byte(self.read_hl(), d8);
                12
            }
            0x37 => {
                self.flag(Flag::N, false);
                self.flag(Flag::H, false);
                self.flag(Flag::C, true);
                4
            }
            0x38 => {
                if self.get_flag(Flag::C) {
                    self.jr();
                    12
                } else {
                    self.pc += 1;
                    8
                }
            }
            0x39 => {
                self.add16(self.sp);
                8
            }
            0x3a => {
                let address = self.hld();
                self.a = self.mmu.read_byte(address);
                8
            }
            0x3b => {
                self.sp -= 1;
                8
            }
            0x3c => {
                self.a = self.inc(self.a);
                4
            }
            0x3d => {
                self.a = self.dec(self.a);
                4
            }
            0x3e => {
                self.a = self.fetch_byte();
                8
            }
            0x3f => {
                self.flag(Flag::N, false);
                self.flag(Flag::H, false);
                self.flag(Flag::C, !self.get_flag(Flag::C));
                4
            }
            0x40 => {
                self.b = self.b;
                4
            }
            0x41 => {
                self.b = self.c;
                4
            }
            0x42 => {
                self.b = self.e;
                4
            }
            0x43 => {
                self.b = self.h;
                4
            }
            0x44 => {
                self.b = self.h;
                4
            }
            0x45 => {
                self.b = self.l;
                4
            }
            0x46 => {
                self.b = self.mmu.read_byte(self.read_hl());
                8
            }
            0x47 => {
                self.b = self.a;
                4
            }
            0x48 => {
                self.c = self.b;
                4
            }
            0x49 => {
                self.c = self.c;
                4
            }
            0x4a => {
                self.c = self.d;
                4
            }
            0x4b => {
                self.c = self.e;
                4
            }
            0x4c => {
                self.c = self.h;
                4
            }
            0x4d => {
                self.c = self.l;
                4
            }
            0x4e => {
                self.c = self.mmu.read_byte(self.read_hl());
                8
            }
            0x4f => {
                self.c = self.a;
                4
            }
            0x50 => {
                self.d = self.b;
                4
            }
            0x51 => {
                self.d = self.c;
                4
            }
            0x52 => {
                self.d = self.d;
                4
            }
            0x53 => {
                self.d = self.e;
                4
            }
            0x54 => {
                self.d = self.h;
                4
            }
            0x55 => {
                self.d = self.l;
                4
            }
            0x56 => {
                self.d = self.mmu.read_byte(self.read_hl());
                8
            }
            0x57 => {
                self.d = self.a;
                4
            }
            0x58 => {
                self.e = self.b;
                4
            }
            0x59 => {
                self.e = self.c;
                4
            }
            0x5a => {
                self.e = self.d;
                4
            }
            0x5b => {
                self.e = self.e;
                4
            }
            0x5c => {
                self.e = self.h;
                4
            }
            0x5d => {
                self.e = self.l;
                4
            }
            0x5e => {
                self.e = self.mmu.read_byte(self.read_hl());
                8
            }
            0x5f => {
                self.e = self.a;
                4
            }
            0x60 => {
                self.h = self.b;
                4
            }
            0x61 => {
                self.h = self.c;
                4
            }
            0x62 => {
                self.h = self.d;
                4
            }
            0x63 => {
                self.h = self.e;
                4
            }
            0x64 => {
                self.h = self.h;
                4
            }
            0x65 => {
                self.h = self.l;
                4
            }
            0x66 => {
                self.h = self.mmu.read_byte(self.read_hl());
                8
            }
            0x67 => {
                self.h = self.a;
                4
            }
            0x68 => {
                self.l = self.b;
                4
            }
            0x69 => {
                self.l = self.c;
                4
            }
            0x6a => {
                self.l = self.d;
                4
            }
            0x6b => {
                self.l = self.e;
                4
            }
            0x6c => {
                self.l = self.h;
                4
            }
            0x6d => {
                self.l = self.l;
                4
            }
            0x6e => {
                self.l = self.mmu.read_byte(self.read_hl());
                8
            }
            0x6f => {
                self.l = self.a;
                4
            }
            0x70 => {
                self.mmu.write_byte(self.read_hl(), self.b);
                8
            }
            0x71 => {
                self.mmu.write_byte(self.read_hl(), self.c);
                8
            }
            0x72 => {
                self.mmu.write_byte(self.read_hl(), self.d);
                8
            }
            0x73 => {
                self.mmu.write_byte(self.read_hl(), self.e);
                8
            }
            0x74 => {
                self.mmu.write_byte(self.read_hl(), self.h);
                8
            }
            0x75 => {
                self.mmu.write_byte(self.read_hl(), self.l);
                8
            }
            0x76 => 4,
            0x77 => {
                self.mmu.write_byte(self.read_hl(), self.a);
                8
            }
            0x78 => {
                self.a = self.b;
                4
            }
            0x79 => {
                self.a = self.c;
                4
            }
            0x7a => {
                self.a = self.d;
                4
            }
            0x7b => {
                self.a = self.e;
                4
            }
            0x7c => {
                self.a = self.h;
                4
            }
            0x7d => {
                self.a = self.l;
                4
            }
            0x7e => {
                self.a = self.mmu.read_byte(self.read_hl());
                8
            }
            0x7f => {
                self.a = self.a;
                4
            }
            0x80 => {
                self.add(self.b, false);
                4
            }
            0x81 => {
                self.add(self.c, false);
                4
            }
            0x82 => {
                self.add(self.d, false);
                4
            }
            0x83 => {
                self.add(self.e, false);
                4
            }
            0x84 => {
                self.add(self.h, false);
                4
            }
            0x85 => {
                self.add(self.l, false);
                4
            }
            0x86 => {
                self.add(self.mmu.read_byte(self.read_hl()), false);
                8
            }
            0x87 => {
                self.add(self.a, false);
                4
            }
            0x88 => {
                self.add(self.b, true);
                4
            }
            0x89 => {
                self.add(self.c, true);
                4
            }
            0x8a => {
                self.add(self.d, true);
                4
            }
            0x8b => {
                self.add(self.e, true);
                4
            }
            0x8c => {
                self.add(self.h, true);
                4
            }
            0x8d => {
                self.add(self.l, true);
                4
            }
            0x8e => {
                self.add(self.mmu.read_byte(self.read_hl()), true);
                8
            }
            0x8f => {
                self.add(self.a, true);
                4
            }
            0x90 => {
                self.sub(self.b, false);
                4
            }
            0x91 => {
                self.sub(self.c, false);
                4
            }
            0x92 => {
                self.sub(self.d, false);
                4
            }
            0x93 => {
                self.sub(self.e, false);
                4
            }
            0x94 => {
                self.sub(self.h, false);
                4
            }
            0x95 => {
                self.sub(self.l, false);
                4
            }
            0x96 => {
                self.sub(self.mmu.read_byte(self.read_hl()), false);
                8
            }
            0x97 => {
                self.sub(self.a, false);
                4
            }
            0x98 => {
                self.sub(self.b, true);
                4
            }
            0x99 => {
                self.sub(self.c, true);
                4
            }
            0x9a => {
                self.sub(self.d, true);
                4
            }
            0x9b => {
                self.sub(self.e, true);
                4
            }
            0x9c => {
                self.sub(self.h, true);
                4
            }
            0x9d => {
                self.sub(self.l, true);
                4
            }
            0x9e => {
                self.sub(self.mmu.read_byte(self.read_hl()), true);
                8
            }
            0x9f => {
                self.sub(self.a, true);
                4
            }
            0xa0 => {
                self.and(self.b);
                4
            }
            0xa1 => {
                self.and(self.c);
                4
            }
            0xa2 => {
                self.and(self.d);
                4
            }
            0xa3 => {
                self.and(self.e);
                4
            }
            0xa4 => {
                self.and(self.h);
                4
            }
            0xa5 => {
                self.and(self.l);
                4
            }
            0xa6 => {
                self.and(self.mmu.read_byte(self.read_hl()));
                8
            }
            0xa7 => {
                self.and(self.a);
                4
            }
            0xa8 => {
                self.xor(self.b);
                4
            }
            0xa9 => {
                self.xor(self.c);
                4
            }
            0xaa => {
                self.xor(self.d);
                4
            }
            0xab => {
                self.xor(self.e);
                4
            }
            0xac => {
                self.xor(self.h);
                4
            }
            0xad => {
                self.xor(self.l);
                4
            }
            0xae => {
                self.xor(self.mmu.read_byte(self.read_hl()));
                8
            }
            0xaf => {
                self.xor(self.a);
                4
            }
            0xb0 => {
                self.or(self.b);
                4
            }
            0xb1 => {
                self.or(self.c);
                4
            }
            0xb2 => {
                self.or(self.d);
                4
            }
            0xb3 => {
                self.or(self.e);
                4
            }
            0xb4 => {
                self.or(self.h);
                4
            }
            0xb5 => {
                self.or(self.l);
                4
            }
            0xb6 => {
                self.or(self.mmu.read_byte(self.read_hl()));
                8
            }
            0xb7 => {
                self.or(self.a);
                4
            }
            0xb8 => {
                self.cp(self.b);
                4
            }
            0xb9 => {
                self.cp(self.c);
                4
            }
            0xba => {
                self.cp(self.d);
                4
            }
            0xbb => {
                self.cp(self.e);
                4
            }
            0xbc => {
                self.cp(self.h);
                4
            }
            0xbd => {
                self.cp(self.l);
                4
            }
            0xbe => {
                self.cp(self.mmu.read_byte(self.read_hl()));
                8
            }
            0xbf => {
                self.cp(self.a);
                4
            }
            0xc0 => {
                if !self.get_flag(Flag::Z) {
                    self.ret();
                    20
                } else {
                    8
                }
            }
            0xc1 => {
                let poped = self.pop();
                self.write_bc(poped);
                12
            }
            0xc2 => {
                if !self.get_flag(Flag::Z) {
                    self.jp();
                    16
                } else {
                    12
                }
            }
            0xc3 => {
                self.jp();
                16
            }
            0xc4 => {
                if !self.get_flag(Flag::Z) {
                    self.call();
                    24
                } else {
                    12
                }
            }
            0xc5 => {
                self.push(self.read_bc());
                16
            }
            0xc6 => {
                let d8 = self.fetch_byte();
                self.add(d8, false);
                8
            }
            0xc7 => {
                self.rst(0x00);
                16
            }
            0xc8 => {
                if self.get_flag(Flag::Z) {
                    self.ret();
                    20
                } else {
                    8
                }
            }
            0xc9 => {
                self.ret();
                16
            }
            0xca => {
                if self.get_flag(Flag::Z) {
                    self.jp();
                    16
                } else {
                    12
                }
            }
            0xcb => self.cb(),
            0xcc => {
                if self.get_flag(Flag::Z) {
                    self.call();
                    24
                } else {
                    12
                }
            }
            0xcd => {
                self.call();
                24
            }
            0xce => {
                let d8 = self.fetch_byte();
                self.add(d8, true);
                8
            }
            0xcf => {
                self.rst(0x08);
                16
            }
            0xd0 => {
                if !self.get_flag(Flag::C) {
                    self.ret();
                    20
                } else {
                    8
                }
            }
            0xd1 => {
                let poped = self.pop();
                self.write_de(poped);
                12
            }
            0xd2 => {
                if !self.get_flag(Flag::C) {
                    self.jp();
                    16
                } else {
                    12
                }
            }
            0xd4 => {
                if !self.get_flag(Flag::C) {
                    self.call();
                    24
                } else {
                    12
                }
            }
            0xd5 => {
                self.push(self.read_de());
                16
            }
            0xd6 => {
                let d8 = self.fetch_byte();
                self.sub(d8, false);
                8
            }
            0xd7 => {
                self.rst(0x10);
                16
            }
            0xd8 => {
                if self.get_flag(Flag::C) {
                    self.ret();
                    20
                } else {
                    8
                }
            }
            0xd9 => {
                self.ret();
                self.ime = true;
                16
            }
            0xda => {
                if self.get_flag(Flag::C) {
                    self.jp();
                    16
                } else {
                    12
                }
            }
            0xdc => {
                if self.get_flag(Flag::C) {
                    self.call();
                    24
                } else {
                    12
                }
            }
            0xde => {
                let d8 = self.fetch_byte();
                self.sub(d8, true);
                8
            }
            0xdf => {
                self.rst(0x18);
                16
            }
            0xe0 => {
                let a8 = 0xff00 + self.fetch_byte() as u16;
                self.mmu.write_byte(a8, self.a);
                12
            }
            0xe1 => {
                let poped = self.pop();
                self.write_hl(poped);
                12
            }
            0xe2 => {
                self.mmu.write_byte(0xff00 + self.c as u16, self.a);
                8
            }
            0xe5 => {
                self.push(self.read_hl());
                16
            }
            0xe6 => {
                let d8 = self.fetch_byte();
                self.and(d8);
                8
            }
            0xe7 => {
                self.rst(0x20);
                16
            }
            0xe8 => {
                todo!() // what to name function
            }
            0xe9 => {
                self.pc = self.read_hl();
                4
            }
            0xea => {
                let a16 = self.fetch_word();
                self.mmu.write_byte(a16, self.a);
                16
            }
            0xee=> {
                let d8 = self.fetch_byte();
                self.xor(d8);
                8
            }
            0xef => {
                self.rst(0x28);
                16
            }
            0xf0 => {
                let a8 = 0xff00 + self.fetch_byte() as u16;
                self.a = self.mmu.read_byte(a8);
                12
            }
            0xf1 => {
                let poped = self.pop();
                self.write_af(poped);
                12
            }
            0xf2 => {
                self.a = self.mmu.read_byte(0xff00 + self.c as u16);
                8
            }
            _ => 0,
        }
    }

    fn cb(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => 0,
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

    fn add(&mut self, val: u8, carry: bool) {
        let a = self.a;
        let c = if carry && self.get_flag(Flag::C) {
            1
        } else {
            0
        };
        let sum = self.a.wrapping_add(val).wrapping_add(c);
        self.flag(Flag::N, false);
        self.flag(Flag::Z, sum == 0);
        self.flag(Flag::H, (a & 0xf) + (val & 0xf) + (c & 0xf) > 0xf);
        self.flag(Flag::C, (a as u16) + (val as u16) + (c as u16) > 0xff);
        self.a = sum;
    }

    fn sub(&mut self, val: u8, carry: bool) {
        let a = self.a;
        let c = if carry && self.get_flag(Flag::C) {
            1
        } else {
            0
        };
        let sum = self.a.wrapping_sub(val).wrapping_sub(c);
        self.flag(Flag::N, true);
        self.flag(Flag::Z, sum == 0);
        self.flag(Flag::H, (a & 0x0f) < (val & 0x0f) + c);
        self.flag(Flag::C, (a as u16) < (val as u16) + (c as u16));
        self.a = sum;
    }

    fn and(&mut self, val: u8) {
        self.a &= val;
        self.flag(Flag::Z, self.a == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, true);
        self.flag(Flag::C, false);
    }

    fn or(&mut self, val: u8) {
        self.a |= val;
        self.flag(Flag::Z, self.a == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, false);
        self.flag(Flag::C, false);
    }

    fn xor(&mut self, val: u8) {
        self.a ^= val;
        self.flag(Flag::Z, self.a == 0);
        self.flag(Flag::N, false);
        self.flag(Flag::H, false);
        self.flag(Flag::C, false);
    }

    fn cp(&mut self, val: u8) {
        let a = self.a;
        self.sub(val, false); // set flags
        self.a = a; // don't store the result
    }

    fn add16(&mut self, val: u16) {
        let hl = self.read_hl();
        self.flag(Flag::N, false);
        self.flag(Flag::H, (hl & 0xfff) + (val & 0xfff) > 0xfff);
        self.flag(Flag::C, hl > 0xffff - val);
        self.write_hl(hl.wrapping_add(val));
    }

    fn jr(&mut self) {
        let r8 = self.fetch_byte() as i8;
        self.pc = self.pc.wrapping_add(r8 as u16);
    }

    fn jp(&mut self) {
        self.pc = self.fetch_word();
    }

    fn call(&mut self) {
        self.push(self.pc);
        self.jp();
    }

    fn rst(&mut self, vector: u16) {
        self.push(self.pc);
        self.pc = vector;
    }

    fn push(&mut self, val: u16) {
        self.mmu.write_word(self.sp, val);
        self.sp -= 2;
    }

    fn pop(&mut self) -> u16 {
        let poped = self.mmu.read_word(self.sp);
        self.sp += 2;
        poped
    }

    fn ret(&mut self) {
        self.pc = self.pop();
    }
}
