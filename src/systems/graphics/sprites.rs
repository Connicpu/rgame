use ecs::System;
use ecs::system::entity::EntityProcess;
use {DataHelper, EntityIter};
use components as comps;
use glium::{Display, Program};
use glium::texture as tex;

pub enum SpriteTexture {
    Texture2d(tex::Texture2d),
}

pub struct SpriteSystem {
    pub display: Option<Display>,
    pub program: Option<Program>,
}

impl SpriteSystem {
    pub fn new() -> SpriteSystem {
        SpriteSystem {
            display: None,
            program: None,
        }
    }

    pub fn init(&mut self, display: Display) {
        self.program = Some(Program::from_source(&display,
            include_str!("sprite_vertex.glsl"),
            include_str!("sprite_fragment.glsl"),
            None // No geometry shader
        ).unwrap());

        self.display = Some(display);
    }
}

impl System for SpriteSystem { type Components = comps::Components; }

impl EntityProcess for SpriteSystem {
    fn process(&mut self, iter: EntityIter, data: &mut DataHelper) {
        use glium::Surface;

        let mut target = self.display.as_ref().unwrap().draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);

        for e in iter {
            data.sprite[e].draw(e, data, self, &mut target);
        }

        target.finish();
    }
}
