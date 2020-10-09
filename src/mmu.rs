pub struct MMU {
    memory: [u8; 0x10000]
}

impl MMU {
    pub fn new() -> Self{
        Self {memory: [0; 0x10000]}
    }

    pub fn read_byte(&self, address: u16) -> u8{
        self.memory[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8) 
    }

    pub fn write_byte(&mut self, address: u16, val: u8){
        self.memory[address as usize] = val
    }

    pub fn write_word(&mut self, address: u16, val: u16){
        self.write_byte(address, (val & 0xff) as u8);
        self.write_byte(address + 1, (val >> 8) as u8);
    }
}