pub struct Serial {
    sb: u8,
    sc: u8,
}

impl Serial {
    pub fn new() -> Self{
        Self {
            sb: 0,
            sc: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff01 => self.sb,
            0xff02 => self.sc,
            n => unreachable!("address {:#x} is not handled by serial", n)
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8){
        if addr == 0xFF02 && val == 0x81 {
            print!("{}", self.read_byte(0xff01) as char);
        }
        match addr {
            0xff01 => self.sb = val,
            0xff02 => self.sc = val,
            n => unreachable!("address {:#x} is not handled by serial", n)
        }
    }
}