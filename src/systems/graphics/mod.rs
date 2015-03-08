use glium::Display;
use ecs::{System, Process};
use components as comps;
use DataHelper;

pub use self::sprites::SpriteSystem;

pub mod sprites;

pub struct GraphicsSystem {
    pub display: Display,
}

impl GraphicsSystem {
    pub fn new() -> GraphicsSystem {
        use glutin;
        use glium::DisplayBuild;

        let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("rgame is bestgame"))
            .build_glium().unwrap();

        GraphicsSystem {
            display: display,
        }
    }
}

impl System for GraphicsSystem { type Components = comps::Components; }

impl Process for GraphicsSystem {
    fn process(&mut self, _: &mut DataHelper) {
        use glutin::Event;
        use libc;

        for event in self.display.poll_events() {
            match event {
                Event::Closed => unsafe {
                    println!("Window closed, exiting.");
                    libc::exit(0);
                },

                _ => ()
            }
        }
    }
}
