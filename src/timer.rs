pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8
}

impl Timer {
  pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff04 => self.div,
            0xff05 => self.tima,
            0xff06 => self.tma,
            0xff07 => self.tac,
            n=> panic!("address {} is not handled by timer", n)
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xff04 => self.div = val,
            0xff05 => self.tima = val,
            0xff06 => self.tma = val,
            0xff07 => self.tac = val,
            n=> panic!("address {} is not handled by timer", n)
        }
    }
}