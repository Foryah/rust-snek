#[path = "consts.rs"] mod consts;

extern crate graphics;
extern crate piston;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs};

use std::collections::LinkedList;

use crate::consts::SQUARE_SIZE;

#[allow(dead_code)]
pub struct Map {
    pub walls: LinkedList<(u32, u32)>,
}

impl Map {
    #[allow(dead_code)]
    pub fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
        let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.walls
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
                    graphics::rectangle(black, square, transform, gl);
                })
        });
    }
}
