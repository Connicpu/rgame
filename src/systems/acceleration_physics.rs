use ecs::System;
use ecs::system::entity::EntityProcess;
use components as comps;
use {DataHelper, EntityIter};

pub struct AccelerationPhysics;

impl System for AccelerationPhysics { system_boilerplate!(); }

impl EntityProcess for AccelerationPhysics {
    fn process(&mut self, iter: EntityIter, data: &mut DataHelper) {
        for e in iter {
            let time = data.time_data[e];
            let acceleration = data.acceleration[e];
            let mut velocity = data.velocity[e];

            velocity.dx += acceleration.ax * time.dt;
            velocity.dy += acceleration.ay * time.dt;

            data.velocity[e] = velocity;

            if let Some(_) = data.debug_print.get(&e) {
                println!("{:?} now has {:?}", *e, velocity);
            }
        }
    }
}
