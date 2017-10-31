extern crate piston_window;
extern crate rand;

mod counter;
mod apple;
mod snek;

use piston_window::*;
use rand::*;

use apple::*;
use snek::*;

const WIDTH: u32 = 48;
const HEIGHT: u32 = 32;
const SCALE: u32 = 16;
const WINDOW_WIDTH: u32 = (WIDTH + 2) * SCALE;
const WINDOW_HEIGHT: u32 = (HEIGHT + 2) * SCALE;

fn draw_border<G: Graphics>(c: &Context, g: &mut G) {
    let w = WINDOW_WIDTH as f64;
    let h = WINDOW_HEIGHT as f64;
    let scale = SCALE as f64;
    rectangle([0., 0., 0., 1.,], [       0.,        0., w, scale,], c.transform, g);
    rectangle([0., 0., 0., 1.,], [       0., h - scale, w, scale,], c.transform, g);
    rectangle([0., 0., 0., 1.,], [       0.,        0., scale, h,], c.transform, g);
    rectangle([0., 0., 0., 1.,], [w - scale,        0., scale, h,], c.transform, g);
}

fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Snek", [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut snek = Snek::new(WIDTH as u32 / 2, HEIGHT as u32 / 2);
    let mut rng = weak_rng();
    let mut apple = Apple::new(rng.gen_range(0, WIDTH), rng.gen_range(0, HEIGHT));

    while let Some(e) = window.next() {
        e.button(|a| {
            if let Button::Keyboard(key) = a.button {
                match key {
                    Key::W | Key::Up => snek.turn(Direction::Up),
                    Key::S | Key::Down => snek.turn(Direction::Down),
                    Key::A | Key::Left => snek.turn(Direction::Left),
                    Key::D | Key::Right => snek.turn(Direction::Right),
                    _ => (),
                }
            }
        });

        e.update(|_| {
            if snek.ate_apple(&apple) {
                snek.enlong();
                apple.set_pos(rng.gen_range(0, WIDTH), rng.gen_range(0, HEIGHT));
            }
            snek.update();
        });

        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.], g);
            snek.draw(&c, g);
            apple.draw(&c, g);
            draw_border(&c, g);
        });
    }
}