mod mbc0; // fix
mod mbc1;
use std::fs;

pub trait MBC {
    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
}


pub fn create_mbc(cart_path: &str) -> Box<dyn MBC> {
    let cart = fs::read(cart_path).expect("File not found");
    let mbc_type = cart[0x147];
    println!("mbc type {}", mbc_type);
    match mbc_type {
        0 => Box::new(mbc0::MBC0::new(cart)),
        1 => Box::new(mbc1::MBC1::new(cart)),
        _ => panic!("Unsupported rom type")
    }
}
