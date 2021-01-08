use crate::cpu::CPU;
use crate::minifb_display::Display;


pub struct Gameboy {
    cpu: CPU,
    display: Box<dyn Display>,
}

impl Gameboy {
    pub fn new(rom_path: &str, display: Box<dyn Display>) -> Self {
        Self {
            cpu: CPU::new(rom_path),
            display,
        }
    }

    pub fn ppu_updated(&mut self) -> bool {
        let updated = self.cpu.mmu.ppu.updated;
        self.cpu.mmu.ppu.updated = false;
        updated
    }

    pub fn emulate(&mut self) {
        let clocks_per_frame = 70224;
        let mut clocks_this_update;
        loop {
            clocks_this_update = 0;
            while clocks_this_update < clocks_per_frame {
                clocks_this_update += self.cpu.do_cycle();
                if self.ppu_updated() {
                    self.update_disply();
                }
            }
            self.handle_input();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    fn update_disply(&mut self){
        let buffer = &self.cpu.mmu.ppu.screen_data;
        self.display.update_with_buffer(buffer);
    }

    fn handle_input(&mut self){
        for button in self.display.get_buttons_down() {
            self.cpu.mmu.joypad.button_down(button);
        }

        for button in self.display.get_buttons_up() {
            self.cpu.mmu.joypad.button_up(button);
        }
    }
}
