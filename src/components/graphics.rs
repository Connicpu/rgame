use std::sync::Arc;
use std::default::Default;

use glium;
use glium::texture::Texture2d;
use glium::{VertexBuffer, IndexBuffer, Surface};
use glium::index::TriangleStrip;

use systems::graphics::SpriteSystem;
use DataHelper;
use ecs::EntityData;
use super::Components;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub texture: Arc<Texture2d>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpriteVertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
    color: [f32; 4],
}

implement_vertex!(SpriteVertex, position, tex_coord, color);

impl Sprite {
    pub fn draw<S>(&self, e: EntityData<Components>, data: &DataHelper,
                   sys: &SpriteSystem, target: &mut S)
                   where S: Surface {
        let white = [1.0, 1.0, 1.0, 1.0];
        let mut vertices = [
            SpriteVertex { position: [-0.5, 0.5],  tex_coord: [0.0, 1.0], color: white },
            SpriteVertex { position: [-0.5, -0.5], tex_coord: [0.0, 0.0], color: white },
            SpriteVertex { position: [0.5, 0.5],   tex_coord: [1.0, 1.0], color: white },
            SpriteVertex { position: [0.5, -0.5],  tex_coord: [1.0, 0.0], color: white },
        ];

        if let Some(scale) = data.scale.get(&e) {
            for vertex in vertices.iter_mut() {
                vertex.position[0] *= scale.size * scale.sx;
                vertex.position[1] *= scale.size * scale.sy;
            }
        }

        let position = data.position[e];
        for vertex in vertices.iter_mut() {
            vertex.position[0] += position.x;
            vertex.position[1] += position.y;
        }

        let display = sys.display.as_ref().unwrap();
        let program = sys.program.as_ref().unwrap();

        let vbuffer = VertexBuffer::new(display, vertices.to_vec());
        let ibuffer = IndexBuffer::new(display, TriangleStrip(vec![0, 1, 2, 3u8]));

        let uniforms = uniform! {
            camera: [ 
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            texture: &*self.texture
        };

        let mut draw_params: glium::DrawParameters = Default::default();
        draw_params.blending_function = Some(
            glium::BlendingFunction::Addition {
                source: glium::LinearBlendingFactor::SourceAlpha,
                destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
            }
        );

        target.draw(&vbuffer, &ibuffer, program, &uniforms, &draw_params).unwrap();
    }
}
