use ecs::System;
use ecs::system::EntitySystem;
use components::{Components, core};
use self::velocity_physics as vp;
use self::acceleration_physics as ap;

pub mod acceleration_physics;
pub mod graphics;
pub mod quit;
pub mod time;
pub mod velocity_physics;

services! {
    Services {
        time: core::TimeData = core::TimeData::new(),
    }
}

systems! {
    Systems<Components, Services> {
        time: EntitySystem<time::TimeSystem> = EntitySystem::new(
            time::TimeSystem::new(),
            aspect!(<Components> all: [time_data])
        ),

        velocity_physics: EntitySystem<vp::VelocityPhysics> = EntitySystem::new(
            vp::VelocityPhysics,
            aspect!(<Components> all: [position, velocity, time_data])
        ),
        acceleration_physics: EntitySystem<ap::AccelerationPhysics> = EntitySystem::new(
            ap::AccelerationPhysics,
            aspect!(<Components> all: [velocity, acceleration, time_data])
        ),

        graphics: graphics::GraphicsSystem = graphics::GraphicsSystem::new(),
        sprites: EntitySystem<graphics::SpriteSystem> = EntitySystem::new(
            graphics::SpriteSystem::new(),
            // All entities with a position and a sprite get drawn
            aspect!(<Components> all: [position, sprite])
        ),

        quit: quit::QuitSystem = quit::QuitSystem::new(),
    }
}
