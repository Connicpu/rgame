use std::old_io::timer::sleep;
use std::time::duration::Duration;
use ecs::{System, Process};
use components as comps;
use DataHelper;

pub struct QuitSystem {
    quit: bool,
}

impl System for QuitSystem { system_boilerplate!(); }

impl Process for QuitSystem {
    fn process(&mut self, _: &mut DataHelper) {
        if !self.quit {
            // Arbitrary wait
            sleep(Duration::milliseconds(10));
        }
    }
}

impl QuitSystem {
    #[inline]
    pub fn new() -> QuitSystem {
        QuitSystem {
            quit: false,
        }
    }

    #[inline]
    pub fn should_quit(&self) -> bool {
        self.quit
    }

    #[inline]
    pub fn quit(&mut self) {
        self.quit = true;
    }
}


