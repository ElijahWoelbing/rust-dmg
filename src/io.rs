pub struct IO {
    registers: [u8; 0x80]
}

impl IO {
    pub fn new() -> Self {
        Self {
            registers: [0; 0x80]
        }
    }

    pub fn read_byte(&self, addr: u16)-> u8{
        self.registers[(addr - 0xff00) as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8){
        if addr == 0xff02 && val == 0x81 {
            print!("{}", self.read_byte(0xff01) as char);
        }
        self.registers[(addr - 0xff00) as usize] = val;
    }

}