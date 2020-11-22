pub struct Timer {
    div: u8, // divide register
    tima: u8, // timer counter
    tma: u8, // timer overflow modulo
    tac: u8, // timer control
    clocks_since_div_inc: u32,
    clocks_since_tima_inc: u32,
    enabled: bool,
    speed: u32,
    pub interrupt: u8
}

impl Timer {
  pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            clocks_since_div_inc: 0,
            clocks_since_tima_inc: 0,
            enabled: false,
            speed: 0,
            interrupt: 0
        }
    }

    pub fn rb(&self, address: u16) -> u8 {
        match address {
            0xff04 => self.div,
            0xff05 => self.tima,
            0xff06 => self.tma,
            0xff07 => self.tac,
            _=> unreachable!("address {} is not handled by timer", address)
        }
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        match address {
            0xff04 => self.div = 0, // div is reset when writen to
            0xff05 => self.tima = value,
            0xff06 => self.tma = value,
            0xff07 => {
                self.enabled = value & 0x4 == 0x4;
                self.speed = match value & 0x3 {
                    0 => 1024,
                    1 => 16,
                    2 => 64,
                    _ => 256
                };
                self.tac = value;
            },
            _=> unreachable!("address {} is not handled by timer", address)
        }
    }

    pub fn tick(&mut self, clocks: u32) {
        self.interrupt = 0; // reset interrupt
        self.clocks_since_div_inc += clocks; 
        if self.clocks_since_div_inc >= 256 {
            self.div = self.div.wrapping_add(1); // incs once every 256 cpu clocks
            self.clocks_since_div_inc -= 256;
        }

        if self.enabled {
            self.clocks_since_tima_inc += clocks;
            if self.clocks_since_tima_inc >= self.speed { 
                self.tima = self.tima.wrapping_add(1); // incs once every (self.speed) cpu clocks 
                self.clocks_since_tima_inc -= self.speed;
    
                if self.tima == 0 { // interrupt if overflow, should wait one cycle before interrupt is thrown
                    self.tima = self.tma;
                    self.interrupt = 4;
                }
            }
        }
    }
}