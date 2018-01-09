
use super::gpu::Color;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Gfx {
    pub gl: GlGraphics
}

fn from_hex(rgba: u32) -> [f32; 4] {
    use std::mem::transmute;
    let bytes: [u8; 4] = unsafe { transmute(rgba.to_be()) };
    let rgba_fractional = [
        (bytes[0] as f32) / 256.0,
        (bytes[1] as f32) / 256.0,
        (bytes[2] as f32) / 256.0,
        (bytes[3] as f32) / 256.0,
    ];
    return rgba_fractional;
}

impl Gfx {
    pub fn render(&mut self, args: &RenderArgs, framebuffer: [Color; 160 * 144]) {
        use graphics::*;

        use std::collections::HashMap;
        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.3, 0.3, 0.3, 1.0], gl);
            for y in 0..144 {
                for x in 0..160 {
                    let square = rectangle::square(1.0, 1.0, 1.0);
                    let transform = c.transform.trans(x as f64, y as f64);
                    let color = framebuffer[y * 144 + x];
                    let rgba = to_rgba(color);
                    rectangle(rgba, square, transform, gl);
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {}
}

fn to_rgba(c: Color) -> [f32; 4] {
    match c {
        Color::DARKEST => from_hex(0x0f380fff),
        Color::DARK => from_hex(0x306230ff),
        Color::LIGHT => from_hex(0x8bac0fff),
        Color::LIGHTEST => from_hex(0x9bbc0fff),
    }
}
