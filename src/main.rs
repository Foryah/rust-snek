#[path = "game.rs"] mod game;
#[path = "snake.rs"] mod snake;
#[path = "map.rs"] mod map;
#[path = "consts.rs"] mod consts;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent, ButtonEvent};
use piston::{EventLoop, ButtonState};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

use crate::game::Game;
use crate::snake::{Snake, Direction};
use crate::map::Map;
use crate::consts::{SCREEN_END_X, SCREEN_END_Y, SQUARE_SIZE};

fn main() {
    let full_map: Vec<(u32, u32)> = vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10),
                                         (0, 11), (0, 12), (0, 13), (0, 14), (0, 15), (0, 16), (0, 17), (0, 18), (0, 19), (0, 20),
                                         (20, 0), (20, 1), (20, 2), (20, 3), (20, 4), (20, 5), (20, 6), (20, 7), (20, 8), (20, 9), (20, 10),
                                         (20, 11), (20, 12), (20, 13), (20, 14), (20, 15), (20, 16), (20, 17), (20, 18), (20, 19), (20, 20),
                                         (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0),
                                         (11, 0), (12, 0), (13, 0), (14, 0), (15, 0), (16, 0), (17, 0), (18, 0), (19, 0),
                                         (1, 20), (2, 20), (3, 20), (4, 20), (5, 20), (6, 20), (7, 20), (8, 20), (9, 20), (10, 20),
                                         (11, 20), (12, 20), (13, 20), (14, 20), (15, 20), (16, 20), (17, 20), (18, 20), (19, 20)];

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Snek Game",
        [(SCREEN_END_X + 1) * SQUARE_SIZE, (SCREEN_END_Y + 1) * SQUARE_SIZE]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snek: Snake {
            body: LinkedList::from_iter((vec![(10, 10), (10, 11), (10, 12)]).into_iter()),
            dir: Direction::Right
        },
        map: Map {
            walls: LinkedList::from_iter(full_map.into_iter()),
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
