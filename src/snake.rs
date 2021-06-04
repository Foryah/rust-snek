#[path = "consts.rs"] mod consts;

extern crate graphics;
extern crate piston;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs};

use std::collections::LinkedList;

use crate::consts::{SCREEN_START_X, SCREEN_END_X, SCREEN_START_Y, SCREEN_END_Y, SQUARE_SIZE};

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Direction {
    Right, Left, Up, Down
}

#[allow(dead_code)]
pub struct Snake {
    pub body: LinkedList<(u32, u32)>,
    pub dir: Direction,
}

impl Snake {
    #[allow(dead_code)]
    pub fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    (x * SQUARE_SIZE) as f64,
                    (y * SQUARE_SIZE) as f64,
                    SQUARE_SIZE as f64
                )
            })
            .collect();

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square| {
                    graphics::rectangle(red, square, transform, gl);
                })
        });
    }
    #[allow(dead_code)]
    pub fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snek has no body!")).clone();
        match self.dir {
            Direction::Left => {
                if new_head.0 == SCREEN_START_X {
                    new_head.0 = SCREEN_END_X;
                } else {
                    new_head.0 -= 1;
                }
            },
            Direction::Right => {
                if new_head.0 == SCREEN_END_X {
                    new_head.0 = SCREEN_START_X;
                } else {
                    new_head.0 += 1;
                }
            },
            Direction::Up => {
                if new_head.1 == SCREEN_START_Y {
                    new_head.1 = SCREEN_END_Y;
                } else {
                    new_head.1 -= 1;
                }
            },
            Direction::Down => {
                if new_head.1 == SCREEN_END_Y {
                    new_head.1 = SCREEN_START_Y;
                } else {
                    new_head.1 += 1;
                }
            },
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}
