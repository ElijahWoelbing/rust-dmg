use crate::utills::check_bit;
#[derive(Debug, Clone, Copy)]
pub enum Button {
    Down,
    Up,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}
pub struct Joypad {
    button_state: u8,
    data: u8,
    pub interrupt: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            button_state: 0xff,
            data: 0xff,
            interrupt: 0,
        }
    }

    pub fn read_byte(&self) -> u8 {
        self.data
    }

    pub fn write_byte(&mut self, value: u8) {
        self.data = (self.data & 0xcf) | (value & 0x30);
    }

    pub fn button_up(&mut self, button: Button) {
        match button {
            Button::Down => self.button_state |= 0x80,
            Button::Up => self.button_state |= 0x40,
            Button::Left => self.button_state |= 0x20,
            Button::Right => self.button_state |= 0x10,
            Button::Start => self.button_state |= 0x8,
            Button::Select => self.button_state |= 0x4,
            Button::B => self.button_state |= 0x2,
            Button::A => self.button_state |= 0x1,
        }
        self.update();

    }

    pub fn button_down(&mut self, button: Button) {
        match button {
            Button::Down => self.button_state &= 0x7f,
            Button::Up =>  self.button_state &= 0xbf,
            Button::Left => self.button_state &= 0xdf,
            Button::Right => self.button_state &= 0xef,
            Button::Start => self.button_state &= 0xf7,
            Button::Select => self.button_state &= 0xfb,
            Button::B => self.button_state &= 0xfd,
            Button::A => self.button_state &= 0xfe,
        }
        self.update();
    }

    fn update(&mut self) {
        let old_button_data = self.data & 0xf; // get old state
        let mut new_button_data = 0xf; // new state no buttons pressed
        println!("{:b}", self.data);
        if check_bit(self.data, 4) {
            new_button_data &= self.button_state >> 4;
        } else if check_bit(self.data, 5) {
            new_button_data &= self.button_state;
        }

        if new_button_data != 0xf && old_button_data == 0xf {
            self.interrupt = 0x10;
        }

        self.data = (self.data & 0xf0) | new_button_data;
    }
}
