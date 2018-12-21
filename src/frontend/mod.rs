mod gfx;

use crate::gb::Gb;
use crate::gb::joypad::Joypad;

use crate::util::Array2D;
use piston_window::*;
use crate::gb::joypad::JoypadInterrupt;
use crate::gb::Interrupts;

pub trait Frontend {
    fn get_input(&self) -> Joypad;
    fn render(&mut self, gb: &mut Gb);
}


pub struct GlutinFrontend {
    window: PistonWindow,
}

impl GlutinFrontend {
    pub fn new() -> GlutinFrontend {
        let bg_map_dim = [32 * 8, 32 * 8];
        let screen_dim = [160, 144];

        let _window_dim = bg_map_dim;
        let _window_dim = [64 * 8, 64 * 8];
        let window_dim = screen_dim;

        let window: PistonWindow = WindowSettings::new("GB", window_dim)
            .exit_on_esc(true)
            .build()
            .unwrap();

        GlutinFrontend { window }
    }
}

impl Frontend for GlutinFrontend {
    fn get_input(&self) -> Joypad {
        unimplemented!()
    }

    fn render(&mut self, gb: &mut Gb) { // TODO remove mut
        let opt_event = self.window.next();
        if let Some(ref e) = opt_event {
            let joypad_interrupt: Option<JoypadInterrupt> = gb.cpu.mmu.joypad.on_event(&e);

            if joypad_interrupt.is_some() {
                gb.cpu.mmu._if |= Interrupts::JOYPAD;
            }

            if let Some(_) = e.render_args() {
                gfx::render_framebuffer1(&mut self.window, &e, &gb.cpu.mmu.ppu.framebuffer);
            }
        }
    }
}
