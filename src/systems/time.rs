use ecs::{System, Process};
use components as comps;
use DataHelper;
use time::precise_time_s;

pub struct TimeSystem {
    pub frames: u64,

    pub start_time: f64,
    pub last_time: f64,
    pub curr_time: f64,

    pub dt64: f64,
    pub dt: f32,
}

impl System for TimeSystem { type Components = comps::Components; }

impl Process for TimeSystem {
    fn process(&mut self, data: &mut DataHelper) {
        self.tick();

        for e 
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
    }
}
