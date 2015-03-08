use ecs::System;
use ecs::system::entity::EntityProcess;
use components as comps;
use {DataHelper, EntityIter};

pub struct VelocityPhysics;

impl System for VelocityPhysics { type Components = comps::Components; }

impl EntityProcess for VelocityPhysics {
    fn process(&mut self, iter: EntityIter, data: &mut DataHelper) {
        for e in iter {
            let time = data.time_data[e];
            let velocity = data.velocity[e];
            let position = &mut data.position[e];

            position.x += velocity.dx * time.dt;
            position.y += velocity.dy * time.dt;

            println!("Entity({:?}) position is now {:?}", e, position);
        }
    }
}
