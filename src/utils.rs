pub fn bit_is_set(byte: u8, bit_pos: u8) -> bool{
    let bit_value = 1 << bit_pos;
    byte & bit_value == bit_value 
}

pub fn reset_bit(byte: u8, bit_pos: u8) -> u8{
    let bit_value = 1 << bit_pos;
    byte & !bit_value
}





    #[test]
    fn test_bit_it_set() {
       let set = bit_is_set(0b00000100, 2);
       let not_set = bit_is_set(0b00000001, 1);
       assert!(set);
       assert!(!not_set);
    }   

    #[test]
    fn test_reset_bit(){
        let reset = reset_bit(0b11011111, 1);
        assert_eq!(reset, 0b11011101)
    }