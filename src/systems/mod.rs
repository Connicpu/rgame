use ecs::System;
use ecs::system::EntitySystem;
use components::Components;
use self::velocity_physics as vp;

pub mod quit;
pub mod time;
pub mod velocity_physics;

systems! {
    Systems<Components> {
        time: time::TimeSystem = time::TimeSystem::new(),

        velocity_physics: EntitySystem<vp::VelocityPhysics> = EntitySystem::new(
            vp::VelocityPhysics,
            aspect!(<Components> all: [position, velocity, time_data])
        ),

        quit: quit::QuitSystem = quit::QuitSystem::new(),
    }
}
