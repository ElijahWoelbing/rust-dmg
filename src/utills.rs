pub fn check_bit(byte: u8, bit_position: u8) -> bool {
    let bit_position_value = 1 << bit_position;
    byte & bit_position_value == bit_position_value
}

pub fn get_bit_value(byte: u8, bit_position: u8) -> u8 {
    match check_bit(byte, bit_position) {
        true => 1,
        false => 0,
    }
}

