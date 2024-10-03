use crate::renderer::pipelines::vertecies::texture_vertex::TextureVertex;

use super::Model;

pub struct Square;

impl Model<TextureVertex> for Square {
    fn vertices(&self) -> Vec<TextureVertex> {
        vec![
            TextureVertex {
                position: [-0.5, 0.5, 0.], // Top-left
                tex_coords: [0., 0.],
            },
            TextureVertex {
                position: [0.5, 0.5, 0.], // Top-right
                tex_coords: [1., 0.],
            },
            TextureVertex {
                position: [0.5, -0.5, 0.], // Bottom-right
                tex_coords: [1., 1.],
            },
            TextureVertex {
                position: [-0.5, -0.5, 0.], // Bottom-left
                tex_coords: [0., 1.],
            },
        ]
    }

    fn indexes(&self) -> Vec<u16> {
        vec![0, 2, 1, 2, 3, 0, 0, 1, 2, 2, 0, 3] // Two triangles forming the square
    }
}
