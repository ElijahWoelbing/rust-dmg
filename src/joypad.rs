pub struct Joypad {
    p1: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { p1: 0 }
    }

    pub fn read_byte(&self) -> u8 {
        self.p1
    }

    pub fn write_byte(&mut self, val: u8){
        self.p1 = val;
    }
}
