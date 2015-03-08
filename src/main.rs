#![feature(old_io, std_misc)]

#[macro_use]
extern crate ecs;
extern crate time;

use ecs::{World, BuildData};
use components::Components;
use components::core;
use systems::Systems;

pub mod components;
pub mod systems;

pub type DataHelper = ecs::world::DataHelper<Components>;
pub type EntityIter<'a> = ecs::entity::EntityIter<'a, Components>;

fn main() {
    let mut world = World::<Components, Systems>::new();

    world.create_entity(
        |entity: BuildData, data: &mut Components| {
            data.position.add(&entity, core::Position { x: 0.0, y: 0.0 });
            data.velocity.add(&entity, core::Velocity { dx: 0.0, dy: 0.0 });
            data.time_data.add(&entity, core::TimeData::new());
        }
    );

    while !world.systems.quit.should_quit() {
        world.update()
    }
}
