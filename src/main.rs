#![feature(old_io, std_misc)]

#[macro_use]
extern crate ecs;

use ecs::World;
use components::Components;
use systems::Systems;

pub mod components;
pub mod systems;

pub type DataHelper = ecs::world::DataHelper<Components>;

fn main() {
    let mut world = World::<Components, Systems>::new();

    while !world.systems.quit.should_quit() {
        world.update()
    }
}
