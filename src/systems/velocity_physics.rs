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
            let mut position = data.position[e];

            position.x += velocity.dx * time.dt;
            position.y += velocity.dy * time.dt;

            data.position[e] = position;

            if let Some(_) = data.debug_print.get(&e) {
                println!("{:?} is now at {:?}", *e, position);
            }
        }
    }
}
