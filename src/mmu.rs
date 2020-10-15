pub struct MMU {
    memory: [u8; 0x10000]
}

impl MMU {
    pub fn new() -> Self{
        Self {memory: [0; 0x10000]}
    }

    pub fn read_byte(&self, addr: u16) -> u8{
        self.memory[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8) 
    }

    pub fn write_byte(&mut self, addr: u16, val: u8){
        self.memory[addr as usize] = val
    }

    pub fn write_word(&mut self, addr: u16, val: u16){
        self.write_byte(addr, (val & 0xff) as u8);
        self.write_byte(addr + 1, (val >> 8) as u8);
    }
}