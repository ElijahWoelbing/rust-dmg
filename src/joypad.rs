pub struct Joypad {
    p1: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { p1: 0 }
    }

    pub fn rb(&self) -> u8 {
        self.p1
    }

    pub fn wb(&mut self, value: u8){
        self.p1 = value;
    }
}
