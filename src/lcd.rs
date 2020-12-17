const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
extern crate minifb;
use minifb::{Key, Window, WindowOptions};

pub struct LCD {
    pub window: Window,
}

impl LCD {
    
    pub fn new() -> Self {
        let mut window = Window::new(
            "rust-dmg",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Self {
            window
        }
    }
}
