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

    pub fn rb(&self, address: u16) -> u8 {
        match address {
            0xff01 => self.sb,
            0xff02 => self.sc,
            n => unreachable!("address {:#x} is not handled by serial", n)
        }
    }

    pub fn wb(&mut self, address: u16, value: u8){
        if address == 0xFF02 && value == 0x81 {
            print!("{}", self.rb(0xff01) as char);
        }
        match address {
            0xff01 => self.sb = value,
            0xff02 => self.sc = value,
            n => unreachable!("address {:#x} is not handled by serial", n)
        }
    }
}