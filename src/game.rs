#[path = "snake.rs"] mod snake;
#[path = "map.rs"] mod map;

extern crate graphics;
extern crate piston;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;

use piston::{Button, Key};
use piston::input::{RenderArgs};

use crate::snake::{Snake, Direction};
use crate::map::Map;

pub struct Game {
    pub gl: GlGraphics,
    pub snek: Snake,
    pub map: Map,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });

        self.map.render(&mut self.gl, arg);
        self.snek.render(&mut self.gl, arg);
    }

    pub fn update(&mut self) {
        self.snek.update();
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snek.dir.clone();

        self.snek.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}
