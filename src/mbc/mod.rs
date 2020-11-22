mod mbc0; // fix
mod mbc1;
use std::fs;

pub trait MBC {
    fn read_rom(&self, addr: u16) -> u8;
    fn write_rom(&mut self, addr: u16, val: u8);
    fn read_ram(&self, addr: u16) -> u8;
    fn write_ram(&mut self, addr: u16, val: u8);
}


pub fn create_mbc(cart_path: &str) -> Box<dyn MBC> {
    let cart = fs::read(cart_path).expect("File not found");
    let mbc_type = cart[0x147];
    match mbc_type {
        0 => Box::new(mbc0::MBC0::new(cart)),
        1 => Box::new(mbc1::MBC1::new(cart)),
        _ => panic!("Unsupported rom type")
    }
}
