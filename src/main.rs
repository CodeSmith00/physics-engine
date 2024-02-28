extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f64::consts::PI;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

pub mod models;
use models::Vector;
use models::World;
use models::Object;

fn main() {
    const RESOLUTION: Vector = Vector::new(960.0, 540.0);
    const WORLD_SIZE: Vector = Vector::new(96.0, 54.0);

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("physics-engine", [RESOLUTION.x, RESOLUTION.y])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let objects = vec![
        Object::new(
            [1.0, 0.0, 0.0, 1.0],
            Vector::new(0.0, WORLD_SIZE.y / 2.0),
            0.0,
            Vector::new(0.0, 0.0),
            vec![],
            1.0,
        ),
        Object::new_passive(
            [1.0, 1.0, 1.0, 1.0],
            Vector::new(0.0, 0.0),
            PI / 4.0,
            Vector::new(0.0, 0.0),
            vec![],
            0.0,
        ),
    ];

    let mut world = World::new(GlGraphics::new(opengl), objects, WORLD_SIZE);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            world.render(&args);
        }

        if let Some(args) = e.update_args() {
            world.update(&args);
        }
    }
}