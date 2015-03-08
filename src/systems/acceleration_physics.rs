use ecs::System;
use ecs::system::entity::EntityProcess;
use components as comps;
use {DataHelper, EntityIter};

pub struct AccelerationPhysics;

impl System for AccelerationPhysics { type Components = comps::Components; }

impl EntityProcess for AccelerationPhysics {
    fn process(&mut self, iter: EntityIter, data: &mut DataHelper) {
        for e in iter {
            let time = data.time_data[e];
            let acceleration = data.acceleration[e];
            let velocity = &mut data.velocity[e];

            velocity.dx += acceleration.ax * time.dt;
            velocity.dy += acceleration.ay * time.dt;

            println!("{:?} now has {:?}", *e, velocity);
        }
    }
}
