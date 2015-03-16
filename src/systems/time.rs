use ecs::System;
use ecs::system::entity::EntityProcess;
use {DataHelper, EntityIter};
use components as comps;
use time::precise_time_s;

pub struct TimeSystem {
    pub frames: u64,

    pub start_time: f64,
    pub last_time: f64,
    pub curr_time: f64,

    pub dt64: f64,
    pub dt: f32,
}

impl System for TimeSystem { system_boilerplate!(); }

impl EntityProcess for TimeSystem {
    fn process(&mut self, iter: EntityIter, data: &mut DataHelper) {
        self.tick();

        for e in iter {
            data.time_data[e].dt = self.dt;
        }
    }
}

impl TimeSystem {
    pub fn new() -> TimeSystem {
        let ctime = precise_time_s();

        TimeSystem {
            frames: 0u64,

            start_time: ctime,
            last_time: ctime,
            curr_time: ctime,

            dt: 0.0f32,
            dt64: 0.0f64,
        }
    }

    pub fn tick(&mut self) {
        self.frames += 1;

        self.last_time = self.curr_time;
        self.curr_time = precise_time_s();

        self.dt64 = self.curr_time - self.last_time;
        self.dt = self.dt64 as f32;
    }
}
