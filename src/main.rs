#![feature(old_io, std_misc, libc)]

#[macro_use]
extern crate glium;
#[macro_use]
extern crate ecs;

extern crate time;
extern crate glutin;
extern crate libc;
extern crate image;

use ecs::{World, BuildData};
use components::Components;
use components::core;
use components::graphics::Sprite;
use systems::Systems;

use glium::Display;
use glium::texture::Texture2d;
use std::sync::Arc;
use std::old_io::BufReader;

pub mod components;
pub mod systems;

pub type DataHelper = ecs::world::DataHelper<Components>;
pub type EntityIter<'a> = ecs::entity::EntityIter<'a, Components>;

fn make_wizard(world: &mut World<Components, Systems>, display: &Display,
               pos: core::Position, vel: core::Velocity, accel: core::Acceleration) {
    println!("Making wizard...");
    world.create_entity(
        |entity: BuildData, data: &mut Components| {
            data.position.add(&entity, pos);
            data.velocity.add(&entity, vel);
            data.acceleration.add(&entity, accel);

            data.time_data.add(&entity, core::TimeData::new());
            data.scale.add(&entity, core::Scale::uniform(0.1));

            println!("Making sprite...");
            let img = image::load(BufReader::new(
                include_bytes!("../test-data/wizard_yellow.png")),
                image::PNG).unwrap();
            let sprite = Texture2d::new(display, img);
            data.sprite.add(&entity, Sprite { texture: Arc::new(sprite) });
        }
    );
}

fn main() {
    let mut world = World::<Components, Systems>::new();

    println!("{:?}", world.systems.graphics.display.get_supported_glsl_version());

    world.systems.graphics.display.assert_no_error();
    println!("Initializing sprite system");

    let display = world.systems.graphics.display.clone();
    world.systems.sprites.init(display.clone());

    println!("Graphics initialized");

    make_wizard(
        &mut world, &display,
        core::Position { x: -1.0, y: 0.0 },
        core::Velocity { dx: 0.25, dy: 0.1 },
        core::Acceleration { ax: 0.0, ay: -0.1 }
    );

    make_wizard(
        &mut world, &display,
        core::Position { x: 0.0, y: -1.0 },
        core::Velocity { dx: 0.0, dy: 2.0 },
        core::Acceleration { ax: 0.0, ay: -1.0 }
    );

    while !world.systems.quit.should_quit() {
        world.update()
    }
}
