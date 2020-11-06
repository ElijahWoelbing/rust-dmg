use crate::mmu::MMU;
pub struct CPU<> {
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

use Flag::{C,H,N,Z};

impl CPU {
    pub fn new(rom_path:&str) -> Self {
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
            mmu: MMU::new(rom_path)
        }
    }

    pub fn tick(){

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

    fn write_flag(&mut self, flag: Flag, set: bool) {
        let mask = flag as u8;
        if set {
            self.f |= mask;
        } else {
            self.f &= !mask
        }
        self.f &= 0xf0;
    }

    fn read_flag(&self, flag: Flag) -> bool {
        let mask = flag as u8;
        self.f & mask == mask
    }

    fn raise_shift_and_rotate_flags(&mut self, result: u8, is_carry: bool) {
        self.write_flag(Z, result == 0);
        self.write_flag(N, false);
        self.write_flag(H, false);
        self.write_flag(C, is_carry);
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
                self.write_flag(Z, false);
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
                self.write_flag(Z, false);
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
                self.write_flag(Z, false);
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
                self.write_flag(Z, false);
                4
            }
            0x20 => {
                if !self.read_flag(Z) {
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
                let addr = self.hli();
                self.mmu.write_byte(addr, self.a);
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
            0x27 => {
                self.daa();
                4
            }
            0x28 => {
                if self.read_flag(Z) {
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
                self.write_flag(H, true);
                self.write_flag(N, true);
                4
            }
            0x30 => {
                if self.read_flag(C) {
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
                let addr = self.hld();
                self.mmu.write_byte(addr, self.a);
                8
            }
            0x33 => {
                self.sp += 1;
                8
            }
            0x34 => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let incremented = self.inc(val);
                self.mmu.write_byte(addr, incremented);
                12
            }
            0x35 => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let decremented = self.dec(val);
                self.mmu.write_byte(addr, decremented);
                12
            }
            0x36 => {
                let d8 = self.fetch_byte();
                self.mmu.write_byte(self.read_hl(), d8);
                12
            }
            0x37 => {
                self.write_flag(N, false);
                self.write_flag(H, false);
                self.write_flag(C, true);
                4
            }
            0x38 => {
                if self.read_flag(C) {
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
                let addr = self.hld();
                self.a = self.mmu.read_byte(addr);
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
                self.write_flag(N, false);
                self.write_flag(H, false);
                self.write_flag(C, !self.read_flag(C));
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
                if !self.read_flag(Z) {
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
                if !self.read_flag(Z) {
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
                if !self.read_flag(Z) {
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
                if self.read_flag(Z) {
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
                if self.read_flag(Z) {
                    self.jp();
                    16
                } else {
                    12
                }
            }
            0xcb => self.execute_perfixed_opcode() + 4,
            0xcc => {
                if self.read_flag(Z) {
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
                if !self.read_flag(C) {
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
                if !self.read_flag(C) {
                    self.jp();
                    16
                } else {
                    12
                }
            }
            0xd4 => {
                if !self.read_flag(C) {
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
                if self.read_flag(C) {
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
                if self.read_flag(C) {
                    self.jp();
                    16
                } else {
                    12
                }
            }
            0xdc => {
                if self.read_flag(C) {
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
                self.sp = self.add_sp_r8();
                16
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
            0xee => {
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
            0xf3 => {
                self.ime = false;
                4
            }
            0xf5 => {
                self.push(self.read_af());
                16
            }
            0xf6 => {
                let d8 = self.fetch_byte();
                self.or(d8);
                8
            }
            0xf7 => {
                self.rst(0x30);
                16
            }
            0xf8 => {
                let spr8 = self.add_sp_r8();
                self.write_hl(spr8);
                12
            }
            0xf9 => {
                self.sp = self.read_hl();
                8
            }
            0xfa => {
                let a16 = self.fetch_word();
                self.a = self.mmu.read_byte(a16);
                16
            }
            0xfb => {
                self.ime = true;
                4
            }
            0xfe => {
                let d8 = self.fetch_byte();
                self.cp(d8);
                8
            }
            0xff => {
                self.rst(0x38);
                16
            }
            _ => {
                println!("invalid opcode");
                0
            }
        }
    }

    fn execute_perfixed_opcode(&mut self) -> u32 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => {
                self.b = self.rlc(self.b);
                8
            }
            0x01 => {
                self.c = self.rlc(self.c);
                8
            }
            0x02 => {
                self.d = self.rlc(self.d);
                8
            }
            0x03 => {
                self.e = self.rlc(self.e);
                8
            }
            0x04 => {
                self.h = self.rlc(self.h);
                8
            }
            0x05 => {
                self.l = self.rlc(self.l);
                8
            }
            0x06 => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let rotated = self.rlc(val);
                self.mmu.write_byte(addr, rotated);
                16
            }
            0x07 => {
                self.a = self.rlc(self.a);
                8
            }
            0x08 => {
                self.b = self.rrc(self.b);
                8
            }
            0x09 => {
                self.c = self.rrc(self.c);
                8
            }
            0x0a => {
                self.d = self.rrc(self.d);
                8
            }
            0x0b => {
                self.e = self.rrc(self.e);
                8
            }
            0x0c => {
                self.h = self.rr(self.h);
                8
            }
            0x0d => {
                self.l = self.rr(self.l);
                8
            }
            0x0e => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let rotated = self.rrc(val);
                self.mmu.write_byte(addr, rotated);
                16
            }
            0x0f => {
                self.a = self.rrc(self.a);
                8
            }
            0x10 => {
                self.b = self.rl(self.b);
                8
            }
            0x11 => {
                self.c = self.rl(self.c);
                8
            }
            0x12 => {
                self.d = self.rl(self.d);
                8
            }
            0x13 => {
                self.e = self.rl(self.e);
                8
            }
            0x14 => {
                self.h = self.rl(self.h);
                8
            }
            0x15 => {
                self.l = self.rl(self.l);
                8
            }
            0x16 => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let rotated = self.rl(val);
                self.mmu.write_byte(addr, rotated);
                16
            }
            0x17 => {
                self.a = self.rl(self.a);
                8
            }
            0x18 => {
                self.b = self.rr(self.b);
                8
            }
            0x19 => {
                self.c = self.rr(self.c);
                8
            }
            0x1a => {
                self.d = self.rr(self.d);
                8
            }
            0x1b => {
                self.e = self.rr(self.e);
                8
            }
            0x1c => {
                self.h = self.rr(self.h);
                8
            }
            0x1d => {
                self.l = self.rr(self.l);
                8
            }
            0x1e => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let rotated = self.rr(val);
                self.mmu.write_byte(addr, rotated);
                16
            }
            0x1f => {
                self.a = self.rr(self.a);
                8
            }
            0x20 => {
                self.b = self.sla(self.b);
                8
            }
            0x21 => {
                self.c = self.sla(self.c);
                8
            }
            0x22 => {
                self.d = self.sla(self.d);
                8
            }
            0x23 => {
                self.e = self.sla(self.e);
                8
            }
            0x24 => {
                self.h = self.sla(self.h);
                8
            }
            0x25 => {
                self.l = self.sla(self.l);
                8
            }
            0x26 => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let shifted = self.sla(val);
                self.mmu.write_byte(addr, shifted);
                16
            }
            0x27 => {
                self.a = self.sla(self.a);
                8
            }
            0x28 => {
                self.b = self.sra(self.b);
                8
            }
            0x29 => {
                self.c = self.sra(self.c);
                8
            }
            0x2a => {
                self.d = self.sra(self.d);
                8
            }
            0x2b => {
                self.e = self.sra(self.e);
                8
            }
            0x2c => {
                self.h = self.sra(self.h);
                8
            }
            0x2d => {
                self.l = self.sra(self.l);
                8
            }
            0x2e => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let shifted = self.sra(val);
                self.mmu.write_byte(addr, shifted);
                16
            }
            0x2f => {
                self.a = self.sra(self.a);
                8
            }
            0x30 => {
                self.b = self.swap(self.b);
                8
            }
            0x31 => {
                self.c = self.swap(self.c);
                8
            }
            0x32 => {
                self.d = self.swap(self.d);
                8
            }
            0x33 => {
                self.e = self.swap(self.e);
                8
            }
            0x34 => {
                self.h = self.swap(self.h);
                8
            }
            0x35 => {
                self.l = self.swap(self.l);
                8
            }
            0x36 => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let swaped = self.swap(val);
                self.mmu.write_byte(addr, swaped);
                16
            }
            0x37 => {
                self.a = self.swap(self.a);
                8
            }
            0x38 => {
                self.b = self.srl(self.b);
                8
            }
            0x39 => {
                self.c = self.srl(self.c);
                8
            }
            0x3a => {
                self.d = self.srl(self.d);
                8
            }
            0x3b => {
                self.e = self.srl(self.e);
                8
            }
            0x3c => {
                self.h = self.srl(self.h);
                8
            }
            0x3d => {
                self.l = self.srl(self.l);
                8
            }
            0x3e => {
                let addr = self.read_hl();
                let val = self.mmu.read_byte(addr);
                let shifted = self.srl(val);
                self.mmu.write_byte(addr, shifted);
                16
            }
            0x3f => {
                self.a = self.srl(self.a);
                8
            }
            0x40 => {
                self.bit(self.b, 0);
                8
            }
            0x41 => {
                self.bit(self.c, 0);
                8
            }
            0x42 => {
                self.bit(self.d, 0);
                8
            }
            0x43 => {
                self.bit(self.e, 0);
                8
            }
            0x44 => {
                self.bit(self.h, 0);
                8
            }
            0x45 => {
                self.bit(self.l, 0);
                8
            }
            0x46 => {
                self.bit_at_hl(0);
                12
            }
            0x47 => {
                self.bit(self.a, 0);
                8
            }
            0x48 => {
                self.bit(self.b, 1);
                8
            }
            0x49 => {
                self.bit(self.c, 1);
                8
            }
            0x4a => {
                self.bit(self.d, 1);
                8
            }
            0x4b => {
                self.bit(self.e, 1);
                8
            }
            0x4c => {
                self.bit(self.h, 1);
                8
            }
            0x4d => {
                self.bit(self.l, 1);
                8
            }
            0x4e => {
                self.bit_at_hl(1);
                12
            }
            0x4f => {
                self.bit(self.a, 1);
                8
            }
            0x50 => {
                self.bit(self.b, 2);
                8
            }
            0x51 => {
                self.bit(self.c, 2);
                8
            }
            0x52 => {
                self.bit(self.d, 2);
                8
            }
            0x53 => {
                self.bit(self.e, 2);
                8
            }
            0x54 => {
                self.bit(self.h, 2);
                8
            }
            0x55 => {
                self.bit(self.l, 2);
                8
            }
            0x56 => {
                self.bit_at_hl(2);
                12
            }
            0x57 => {
                self.bit(self.a, 2);
                8
            }
            0x58 => {
                self.bit(self.b, 3);
                8
            }
            0x59 => {
                self.bit(self.c, 3);
                8
            }
            0x5a => {
                self.bit(self.d, 3);
                8
            }
            0x5b => {
                self.bit(self.e, 3);
                8
            }
            0x5c => {
                self.bit(self.h, 3);
                8
            }
            0x5d => {
                self.bit(self.l, 3);
                8
            }
            0x5e => {
                self.bit_at_hl(3);
                12
            }
            0x5f => {
                self.bit(self.a, 3);
                8
            }
            0x60 => {
                self.bit(self.b, 4);
                8
            }
            0x61 => {
                self.bit(self.c, 4);
                8
            }
            0x62 => {
                self.bit(self.d, 4);
                8
            }
            0x63 => {
                self.bit(self.e, 4);
                8
            }
            0x64 => {
                self.bit(self.h, 4);
                8
            }
            0x65 => {
                self.bit(self.l, 4);
                8
            }
            0x66 => {
                self.bit_at_hl(4);
                12
            }
            0x67 => {
                self.bit(self.a, 4);
                8
            }
            0x68 => {
                self.bit(self.b, 5);
                8
            }
            0x69 => {
                self.bit(self.c, 5);
                8
            }
            0x6a => {
                self.bit(self.d, 5);
                8
            }
            0x6b => {
                self.bit(self.e, 5);
                8
            }
            0x6c => {
                self.bit(self.h, 5);
                8
            }
            0x6d => {
                self.bit(self.l, 5);
                8
            }
            0x6e => {
                self.bit_at_hl(5);
                12
            }
            0x6f => {
                self.bit(self.a, 5);
                8
            }
            0x70 => {
                self.bit(self.b, 6);
                8
            }
            0x71 => {
                self.bit(self.c, 6);
                8
            }
            0x72 => {
                self.bit(self.d, 6);
                8
            }
            0x73 => {
                self.bit(self.e, 6);
                8
            }
            0x74 => {
                self.bit(self.h, 6);
                8
            }
            0x75 => {
                self.bit(self.l, 6);
                8
            }
            0x76 => {
                self.bit_at_hl(6);
                12
            }
            0x77 => {
                self.bit(self.a, 6);
                8
            }
            0x78 => {
                self.bit(self.b, 7);
                8
            }
            0x79 => {
                self.bit(self.c, 7);
                8
            }
            0x7a => {
                self.bit(self.d, 7);
                8
            }
            0x7b => {
                self.bit(self.e, 7);
                8
            }
            0x7c => {
                self.bit(self.h, 7);
                8
            }
            0x7d => {
                self.bit(self.l, 7);
                8
            }
            0x7e => {
                self.bit_at_hl(7);
                12
            }
            0x7f => {
                self.bit(self.a, 7);
                8
            }
            0x80 => {
                self.b = self.res(self.b, 0);
                8
            }
            0x81 => {
                self.c = self.res(self.c, 0);
                8
            }
            0x82 => {
                self.d = self.res(self.d, 0);
                8
            }
            0x83 => {
                self.e = self.res(self.e, 0);
                8
            }
            0x84 => {
                self.h = self.res(self.h, 0);
                8
            }
            0x85 => {
                self.l = self.res(self.l, 0);
                8
            }
            0x86 => {
                self.res_at_hl(0);
                16
            }
            0x87 => {
                self.a = self.res(self.a, 0);
                8
            }
            0x88 => {
                self.b = self.res(self.b, 1);
                8
            }
            0x89 => {
                self.c = self.res(self.c, 1);
                8
            }
            0x8a => {
                self.d = self.res(self.d, 1);
                8
            }
            0x8b => {
                self.e = self.res(self.e, 1);
                8
            }
            0x8c => {
                self.h = self.res(self.h, 1);
                8
            }
            0x8d => {
                self.l = self.res(self.l, 1);
                8
            }
            0x8e => {
                self.res_at_hl(1);
                16
            }
            0x8f => {
                self.a = self.res(self.a, 1);
                8
            }
            0x90 => {
                self.b = self.res(self.b, 2);
                8
            }
            0x91 => {
                self.c = self.res(self.c, 2);
                8
            }
            0x92 => {
                self.d = self.res(self.d, 2);
                8
            }
            0x93 => {
                self.e = self.res(self.e, 2);
                8
            }
            0x94 => {
                self.h = self.res(self.h, 2);
                8
            }
            0x95 => {
                self.l = self.res(self.l, 2);
                8
            }
            0x96 => {
                self.res_at_hl(2);
                16
            }
            0x97 => {
                self.a = self.res(self.a, 2);
                8
            }
            0x98 => {
                self.b = self.res(self.b, 3);
                8
            }
            0x99 => {
                self.c = self.res(self.c, 3);
                8
            }
            0x9a => {
                self.d = self.res(self.d, 3);
                8
            }
            0x9b => {
                self.e = self.res(self.e, 3);
                8
            }
            0x9c => {
                self.h = self.res(self.h, 3);
                8
            }
            0x9d => {
                self.l = self.res(self.l, 3);
                8
            }
            0x9e => {
                self.res_at_hl(3);
                16
            }
            0x9f => {
                self.a = self.res(self.a, 3);
                8
            }
            0xa0 => {
                self.b = self.res(self.b, 4);
                8
            }
            0xa1 => {
                self.c = self.res(self.c, 4);
                8
            }
            0xa2 => {
                self.d = self.res(self.d, 4);
                8
            }
            0xa3 => {
                self.e = self.res(self.e, 4);
                8
            }
            0xa4 => {
                self.h = self.res(self.h, 4);
                8
            }
            0xa5 => {
                self.l = self.res(self.l, 4);
                8
            }
            0xa6 => {
                self.res_at_hl(4);
                16
            }
            0xa7 => {
                self.a = self.res(self.a, 4);
                8
            }
            0xa8 => {
                self.b = self.res(self.b, 5);
                8
            }
            0xa9 => {
                self.c = self.res(self.c, 5);
                8
            }
            0xaa => {
                self.d = self.res(self.d, 5);
                8
            }
            0xab => {
                self.e = self.res(self.e, 5);
                8
            }
            0xac => {
                self.h = self.res(self.h, 5);
                8
            }
            0xad => {
                self.l = self.res(self.l, 5);
                8
            }
            0xae => {
                self.res_at_hl(5);
                16
            }
            0xaf => {
                self.a = self.res(self.a, 5);
                8
            }
            0xb0 => {
                self.b = self.res(self.b, 6);
                8
            }
            0xb1 => {
                self.c = self.res(self.c, 6);
                8
            }
            0xb2 => {
                self.d = self.res(self.d, 6);
                8
            }
            0xb3 => {
                self.e = self.res(self.e, 6);
                8
            }
            0xb4 => {
                self.h = self.res(self.h, 6);
                8
            }
            0xb5 => {
                self.l = self.res(self.l, 6);
                8
            }
            0xb6 => {
                self.res_at_hl(6);
                16
            }
            0xb7 => {
                self.a = self.res(self.a, 6);
                8
            }
            0xb8 => {
                self.b = self.res(self.b, 7);
                8
            }
            0xb9 => {
                self.c = self.res(self.c, 7);
                8
            }
            0xba => {
                self.d = self.res(self.d, 7);
                8
            }
            0xbb => {
                self.e = self.res(self.e, 7);
                8
            }
            0xbc => {
                self.h = self.res(self.h, 7);
                8
            }
            0xbd => {
                self.l = self.res(self.l, 7);
                8
            }
            0xbe => {
                self.res_at_hl(7);
                16
            }
            0xbf => {
                self.a = self.res(self.a, 7);
                8
            }
            0xc0 => {
                self.b = self.set(self.b, 0);
                8
            }
            0xc1 => {
                self.c = self.set(self.c, 0);
                8
            }
            0xc2 => {
                self.d = self.set(self.d, 0);
                8
            }
            0xc3 => {
                self.e = self.set(self.e, 0);
                8
            }
            0xc4 => {
                self.h = self.set(self.h, 0);
                8
            }
            0xc5 => {
                self.l = self.set(self.l, 0);
                8
            }
            0xc6 => {
                self.set_at_hl(0);
                16
            }
            0xc7 => {
                self.a = self.set(self.a, 0);
                8
            }
            0xc8 => {
                self.b = self.set(self.b, 1);
                8
            }
            0xc9 => {
                self.c = self.set(self.c, 1);
                8
            }
            0xca => {
                self.d = self.set(self.d, 1);
                8
            }
            0xcb => {
                self.e = self.set(self.e, 1);
                8
            }
            0xcc => {
                self.h = self.set(self.h, 1);
                8
            }
            0xcd => {
                self.l = self.set(self.l, 1);
                8
            }
            0xce => {
                self.set_at_hl(1);
                16
            }
            0xcf => {
                self.a = self.set(self.a, 1);
                8
            }
            //
            0xd0 => {
                self.b = self.set(self.b, 2);
                8
            }
            0xd1 => {
                self.c = self.set(self.c, 2);
                8
            }
            0xd2 => {
                self.d = self.set(self.d, 2);
                8
            }
            0xd3 => {
                self.e = self.set(self.e, 2);
                8
            }
            0xd4 => {
                self.h = self.set(self.h, 2);
                8
            }
            0xd5 => {
                self.l = self.set(self.l, 2);
                8
            }
            0xd6 => {
                self.set_at_hl(2);
                16
            }
            0xd7 => {
                self.a = self.set(self.a, 2);
                8
            }
            0xd8 => {
                self.b = self.set(self.b, 3);
                8
            }
            0xd9 => {
                self.c = self.set(self.c, 3);
                8
            }
            0xda => {
                self.d = self.set(self.d, 3);
                8
            }
            0xdb => {
                self.e = self.set(self.e, 3);
                8
            }
            0xdc => {
                self.h = self.set(self.h, 3);
                8
            }
            0xdd => {
                self.l = self.set(self.l, 3);
                8
            }
            0xde => {
                self.set_at_hl(3);
                16
            }
            0xdf => {
                self.a = self.set(self.a, 3);
                8
            }
            //
            0xe0 => {
                self.b = self.set(self.b, 4);
                8
            }
            0xe1 => {
                self.c = self.set(self.c, 4);
                8
            }
            0xe2 => {
                self.d = self.set(self.d, 4);
                8
            }
            0xe3 => {
                self.e = self.set(self.e, 4);
                8
            }
            0xe4 => {
                self.h = self.set(self.h, 4);
                8
            }
            0xe5 => {
                self.l = self.set(self.l, 4);
                8
            }
            0xe6 => {
                self.set_at_hl(4);
                16
            }
            0xe7 => {
                self.a = self.set(self.a, 4);
                8
            }
            0xe8 => {
                self.b = self.set(self.b, 5);
                8
            }
            0xe9 => {
                self.c = self.set(self.c, 5);
                8
            }
            0xea => {
                self.d = self.set(self.d, 5);
                8
            }
            0xeb => {
                self.e = self.set(self.e, 5);
                8
            }
            0xec => {
                self.h = self.set(self.h, 5);
                8
            }
            0xed => {
                self.l = self.set(self.l, 5);
                8
            }
            0xee => {
                self.set_at_hl(5);
                16
            }
            0xef => {
                self.a = self.set(self.a, 5);
                8
            }
            0xf0 => {
                self.b = self.set(self.b, 6);
                8
            }
            0xf1 => {
                self.c = self.set(self.c, 6);
                8
            }
            0xf2 => {
                self.d = self.set(self.d, 6);
                8
            }
            0xf3 => {
                self.e = self.set(self.e, 6);
                8
            }
            0xf4 => {
                self.h = self.set(self.h, 6);
                8
            }
            0xf5 => {
                self.l = self.set(self.l, 6);
                8
            }
            0xf6 => {
                self.set_at_hl(6);
                16
            }
            0xf7 => {
                self.a = self.set(self.a, 6);
                8
            }
            0xf8 => {
                self.b = self.set(self.b, 7);
                8
            }
            0xf9 => {
                self.c = self.set(self.c, 7);
                8
            }
            0xfa => {
                self.d = self.set(self.d, 7);
                8
            }
            0xfb => {
                self.e = self.set(self.e, 7);
                8
            }
            0xfc => {
                self.h = self.set(self.h, 7);
                8
            }
            0xfd => {
                self.l = self.set(self.l, 7);
                8
            }
            0xfe => {
                self.set_at_hl(7);
                16
            }
            0xff => {
                self.a = self.set(self.a, 7);
                8
            }
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
        let incermented = val.wrapping_add(1);
        self.write_flag(Z, incermented == 0);
        self.write_flag(N, false);
        self.write_flag(H, (incermented & 0xF) > 0x10);
        incermented
    }

    fn dec(&mut self, val: u8) -> u8 {
        let decremented = val.wrapping_sub(1);
        self.write_flag(Z, decremented == 0);
        self.write_flag(N, true);
        self.write_flag(H, (val & 0xF) == 0);
        decremented
    }

    fn rotate_left(&self, val: u8) -> u8 {
        ((val << 1) | (val >> 7)) & 0xff
    }

    fn rotate_right(&self, val: u8) -> u8 {
        ((val >> 1) | (val << 7)) & 0xff
    }

    fn rl(&mut self, val: u8) -> u8 {
        let rotated = self.rotate_left(val) | if self.read_flag(C) { 1 } else { 0 };
        let carry = val >= 0x80;
        self.raise_shift_and_rotate_flags(rotated, carry);
        rotated
    }

    fn rlc(&mut self, val: u8) -> u8 {
        let rotated = self.rotate_left(val);
        let carry = val >= 0x80;
        self.raise_shift_and_rotate_flags(rotated, carry);
        rotated
    }

    fn rr(&mut self, val: u8) -> u8 {
        let rotated = self.rotate_left(val) | if self.read_flag(C) { 0x80 } else { 0 };
        let carry = val & 0x1 == 0x1;
        self.raise_shift_and_rotate_flags(rotated, carry);
        rotated
    }

    fn rrc(&mut self, val: u8) -> u8 {
        let rotated = self.rotate_right(val);
        let carry = val & 0x1 == 0x1;
        self.raise_shift_and_rotate_flags(rotated, carry);
        rotated
    }

    fn sla(&mut self, val: u8) -> u8 {
        let shifted = val << 1;
        let carry = val >= 0x80;
        self.raise_shift_and_rotate_flags(shifted, carry);
        shifted
    }

    fn srl(&mut self, val: u8) -> u8 {
        let shifted = val >> 1;
        let carry = val & 0x1 == 0x1;
        self.raise_shift_and_rotate_flags(shifted, carry);
        shifted
    }

    fn sra(&mut self, val: u8) -> u8 {
        let shifted = (val >> 1) | (val & 0x80);
        let carry = val & 0x1 == 0x1;
        self.raise_shift_and_rotate_flags(shifted, carry);
        shifted
    }

    fn swap(&mut self, val: u8) -> u8 {
        let swaped = (val >> 4) | (val << 4);
        self.write_flag(Z, swaped == 0);
        self.write_flag(N, false);
        self.write_flag(H, false);
        self.write_flag(C, false);
        swaped
    }

    fn bit(&mut self, val: u8, bit: u8) {
        let bit_set = val & (1 << bit) == 0;
        self.write_flag(Z, bit_set);
        self.write_flag(N, false);
        self.write_flag(H, true);
    }

    fn bit_at_hl(&mut self, bit: u8) {
        let val_at_hl = self.mmu.read_byte(self.read_hl());
        self.bit(val_at_hl, bit);
    }

    fn res(&mut self, val: u8, bit: u8) -> u8 {
        let mask = !(1 << bit);
        val & mask
    }

    fn res_at_hl(&mut self, bit: u8) {
        let addr = self.read_hl();
        let val_at_hl = self.mmu.read_byte(addr);
        let val_res = self.res(val_at_hl, bit);
        self.mmu.write_byte(addr, val_res);
    }

    fn set(&mut self, val: u8, bit: u8) -> u8 {
        let mask = 1 << bit;
        val | mask
    }

    fn set_at_hl(&mut self, bit: u8) {
        let addr = self.read_hl();
        let val_at_hl = self.mmu.read_byte(addr);
        let val_res = self.set(val_at_hl, bit);
        self.mmu.write_byte(addr, val_res);
    }

    fn add(&mut self, val: u8, carry: bool) {
        let a = self.a;
        let c = if carry && self.read_flag(C) {
            1
        } else {
            0
        };
        let sum = self.a.wrapping_add(val).wrapping_add(c);
        self.write_flag(N, false);
        self.write_flag(Z, sum == 0);
        self.write_flag(H, (a & 0xf) + (val & 0xf) + (c & 0xf) > 0xf);
        self.write_flag(C, (a as u16) + (val as u16) + (c as u16) > 0xff);
        self.a = sum;
    }

    fn sub(&mut self, val: u8, carry: bool) {
        let a = self.a;
        let c = if carry && self.read_flag(C) {
            1
        } else {
            0
        };
        let sum = self.a.wrapping_sub(val).wrapping_sub(c);
        self.write_flag(N, true);
        self.write_flag(Z, sum == 0);
        self.write_flag(H, (a & 0x0f) < (val & 0x0f) + c);
        self.write_flag(C, (a as u16) < (val as u16) + (c as u16));
        self.a = sum;
    }

    fn and(&mut self, val: u8) {
        self.a &= val;
        self.write_flag(Z, self.a == 0);
        self.write_flag(N, false);
        self.write_flag(H, true);
        self.write_flag(C, false);
    }

    fn or(&mut self, val: u8) {
        self.a |= val;
        self.write_flag(Z, self.a == 0);
        self.write_flag(N, false);
        self.write_flag(H, false);
        self.write_flag(C, false);
    }

    fn xor(&mut self, val: u8) {
        self.a ^= val;
        self.write_flag(Z, self.a == 0);
        self.write_flag(N, false);
        self.write_flag(H, false);
        self.write_flag(C, false);
    }

    fn cp(&mut self, val: u8) {
        let a = self.a;
        self.sub(val, false); // set flags
        self.a = a; // don't store the result
    }

    fn add16(&mut self, val: u16) {
        let hl = self.read_hl();
        self.write_flag(N, false);
        self.write_flag(H, (hl & 0xfff) + (val & 0xfff) > 0xfff);
        self.write_flag(C, hl > 0xffff - val);
        self.write_hl(hl.wrapping_add(val));
    }

    fn add_sp_r8(&mut self) -> u16 {
        let sp = self.sp;
        let r8 = self.fetch_byte() as i8 as u16;
        self.write_flag(Z, false);
        self.write_flag(N, false);
        self.write_flag(H, (sp & 0xf) + (r8 & 0xf) > 0xf);
        self.write_flag(C, (sp & 0xff) + r8 > 0xff);
        sp.wrapping_add(r8)
    }

    fn daa(&mut self) {
        let mut a = self.a;
        if !self.read_flag(N) {
            if self.read_flag(C) || a > 0x99 {
                a += 0x60;
                self.write_flag(C, true);
            }
            if self.read_flag(H) || a & 0xf > 9 {
                a += 0x6;
            }
        } else {
            if self.read_flag(C) {
                a -= 0x60;
            }
            if self.read_flag(H) {
                a -= 0x6;
            }
        }
        self.write_flag(Z, a == 0);
        self.write_flag(H, false);
        self.a = a;
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
