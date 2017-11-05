use piston_window::*;
use a3d;

use {WIDTH, HEIGHT, SCALE};

pub struct Apple {
    pos: (u32, u32),
}

impl Apple {
    pub fn new(x: u32, y: u32) -> Self {
        Self { pos: (x, y), }
    }

    pub fn pos(&self) -> (u32, u32) {
        self.pos
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.pos = (x, y);
    }

    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        let (x, y) = (self.pos.0 as f64, self.pos.1 as f64);
        let scale = SCALE as f64;
        a3d::rectangle(
            [1., 0., 0., 1.],
            [(x + 1.) * scale, (y + 1.) * scale, scale, scale],
            c.transform,
            g
        );
    }
}
