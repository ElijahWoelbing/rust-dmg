extern crate minifb;
use crate::joypad::Button;

pub trait Display {
    fn update_with_buffer(&mut self, buffer: &[u32]);
    fn get_buttons_down(&self) -> Vec<Button>;
    fn get_buttons_up(&self) -> Vec<Button>;
}

pub struct MinifbDisplay {
    pub window: minifb::Window,
    width: usize,
    height: usize,
}

impl MinifbDisplay {
    pub fn new(width: usize, height: usize) -> Box<Self> {
        let window = match 
            minifb::Window::new("Rust DMG", width, height, minifb::WindowOptions::default()) {
                Ok(window) => window,
                Err(e) => panic!("{}", e)
            };
        Box::new(Self {
            window,
            width,
            height,
        })
    }
}

impl Display for MinifbDisplay {
    fn update_with_buffer(&mut self, buffer: &[u32]) {
       match self.window
            .update_with_buffer(buffer, self.width, self.height) {
                Ok(_) => (),
                Err(e) => panic!("{}", e)
            }
    }

    fn get_buttons_down(&self) -> Vec<Button> {
        let mut buttons: Vec<Button> = vec![];
            for key in self.window.get_keys_pressed(minifb::KeyRepeat::No).unwrap() {
                match key {
                    minifb::Key::W => {buttons.push(Button::Up);},
                    minifb::Key::A => {buttons.push(Button::Left);},
                    minifb::Key::S => {buttons.push(Button::Down);},
                    minifb::Key::D => {buttons.push(Button::Right);},
                    minifb::Key::V => {buttons.push(Button::Start);},
                    minifb::Key::B => {buttons.push(Button::Select);},
                    minifb::Key::J => {buttons.push(Button::A);},
                    minifb::Key::K => {buttons.push(Button::B);},
                    _=> ()
                }
            }
       buttons
    }

    fn get_buttons_up(&self) -> Vec<Button> {
        let mut buttons: Vec<Button> = vec![];
            for key in self.window.get_keys_released().unwrap() {
                match key {
                    minifb::Key::W => {buttons.push(Button::Up);},
                    minifb::Key::A => {buttons.push(Button::Left);},
                    minifb::Key::S => {buttons.push(Button::Down);},
                    minifb::Key::D => {buttons.push(Button::Right);},
                    minifb::Key::V => {buttons.push(Button::Start);},
                    minifb::Key::B => {buttons.push(Button::Select);},
                    minifb::Key::J => {buttons.push(Button::A);},
                    minifb::Key::K => {buttons.push(Button::B);},
                    _=> ()
                }
            }
       buttons
    }
}
