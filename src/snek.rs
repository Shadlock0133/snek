use std::collections::VecDeque;

use piston_window::*;

use {WIDTH, HEIGHT, SCALE};
use apple::*;
use counter::*;

#[derive(Clone, Copy)]
enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snek {
    state: State,
    body: VecDeque<(u32, u32)>,
    dir: Direction,
    update_counter: Counter,
}

impl Snek {
    pub fn new(x: u32, y: u32) -> Self {
        let mut body = VecDeque::new();
        body.push_front((x as u32, y as u32));
        Snek {
            state: State::Alive,
            body,
            dir: Direction::Up,
            update_counter: Counter::new(20),
        }
    }

    fn move_body(&mut self) {
        if let Some(&(x, y)) = self.body.front() {
            let new_head = match self.dir {
                Direction::Up    => (x    , y - 1),
                Direction::Down  => (x    , y + 1),
                Direction::Left  => (x - 1, y    ),
                Direction::Right => (x + 1, y    ),
            };
            self.body.push_front(new_head);
            self.body.pop_back().unwrap();
        }
    }

    pub fn enlong(&mut self) {
        let pos = self.body.back().unwrap().clone();
        self.body.push_back(pos);
    }

    pub fn turn(&mut self, dir: Direction) {
        use Direction::*;
        match (&self.dir, &dir) {
            (&Up, &Down) |
            (&Down, &Up) |
            (&Left, &Right) |
            (&Right, &Left) => return,
            _ => (),
        }
        self.dir = dir;
    }

    fn hit_self(&self) -> bool {
        let head = self.body.front().unwrap();
        self.body.iter().skip(1).rev().skip(1).any(|piece| head == piece)
    }

    fn is_outside(&self) -> bool {
        self.body.iter().any(|&piece| {
            (piece.0 >= WIDTH) | (piece.1 >= HEIGHT)
        })
    }

    fn update_speed(&mut self) {
        let speed = 20 - ((self.body.len() as u32) / 3);
        let speed = if speed == 0 {
            1
        } else if speed > 20 {
            1
        } else {
            speed
        };
        self.update_counter.set_trigger(speed);
    }

    pub fn update(&mut self) {
        match self.state {
            State::Alive => {
                if self.update_counter.checked_inc() {
                    self.update_speed();
                    self.move_body();
                }        
                if self.hit_self() | self.is_outside() {
                    self.state = State::Dead;
                }
            }
            State::Dead => {
                *self = Self::new(WIDTH / 2, HEIGHT / 2);
            }
        }
    }

    pub fn ate_apple(&self, apple: &Apple) -> bool {
        let head = self.body.front().unwrap();
        *head == apple.pos()
    }

    fn get_colour(&self, index: usize) -> [f32; 4] {
        fn ramp(min: [f32; 4], max: [f32; 4], value: f32) -> [f32; 4] {
            let mut out = [0.; 4];
            for i in 0..4 {
                out[i] = ((min[i] * (1. - value)) + (max[i] * value)) / 2.;
            }
            out
        }

        if index == 0 {
            [0.3, 0.7, 0.15, 1.]
        } else {
            ramp(
                [0.4, 0.6, 0.15, 1.],
                [0.3, 0.8, 0.2, 1.],
                ((index - 1) as f32) / (self.body.len() - 2) as f32
            )
        }
    }

    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        for (index, &(x, y)) in self.body.iter().enumerate() {
            let (x, y) = (x as f64, y as f64);
            let scale = SCALE as f64;
            rectangle(
                self.get_colour(index),
                [(x + 1.) * scale, (y + 1.) * scale, scale, scale],
                c.transform,
                g
            );
        }
    }
}