use crate::renderer::pipelines::vertecies::texture_vertex::TextureVertex;

use super::Model;

pub struct Cube;

impl Model<TextureVertex> for Cube {
    fn vertices(&self) -> Vec<TextureVertex> {
        vec![
            // Front face
            TextureVertex {
                position: [-0.5, -0.5, 0.5], // Bottom-left-front
                tex_coords: [0., 1.],
            },
            TextureVertex {
                position: [0.5, -0.5, 0.5], // Bottom-right-front
                tex_coords: [1., 1.],
            },
            TextureVertex {
                position: [0.5, 0.5, 0.5], // Top-right-front
                tex_coords: [1., 0.],
            },
            TextureVertex {
                position: [-0.5, 0.5, 0.5], // Top-left-front
                tex_coords: [0., 0.],
            },
            // Back face
            TextureVertex {
                position: [-0.5, -0.5, -0.5], // Bottom-left-back
                tex_coords: [1., 1.],
            },
            TextureVertex {
                position: [0.5, -0.5, -0.5], // Bottom-right-back
                tex_coords: [0., 1.],
            },
            TextureVertex {
                position: [0.5, 0.5, -0.5], // Top-right-back
                tex_coords: [0., 0.],
            },
            TextureVertex {
                position: [-0.5, 0.5, -0.5], // Top-left-back
                tex_coords: [1., 0.],
            },
            // Left face
            TextureVertex {
                position: [-0.5, -0.5, -0.5], // Bottom-left-back
                tex_coords: [0., 1.],
            },
            TextureVertex {
                position: [-0.5, -0.5, 0.5], // Bottom-left-front
                tex_coords: [1., 1.],
            },
            TextureVertex {
                position: [-0.5, 0.5, 0.5], // Top-left-front
                tex_coords: [1., 0.],
            },
            TextureVertex {
                position: [-0.5, 0.5, -0.5], // Top-left-back
                tex_coords: [0., 0.],
            },
            // Right face
            TextureVertex {
                position: [0.5, -0.5, -0.5], // Bottom-right-back
                tex_coords: [1., 1.],
            },
            TextureVertex {
                position: [0.5, -0.5, 0.5], // Bottom-right-front
                tex_coords: [0., 1.],
            },
            TextureVertex {
                position: [0.5, 0.5, 0.5], // Top-right-front
                tex_coords: [0., 0.],
            },
            TextureVertex {
                position: [0.5, 0.5, -0.5], // Top-right-back
                tex_coords: [1., 0.],
            },
            // Top face
            TextureVertex {
                position: [-0.5, 0.5, -0.5], // Top-left-back
                tex_coords: [0., 1.],
            },
            TextureVertex {
                position: [0.5, 0.5, -0.5], // Top-right-back
                tex_coords: [1., 1.],
            },
            TextureVertex {
                position: [0.5, 0.5, 0.5], // Top-right-front
                tex_coords: [1., 0.],
            },
            TextureVertex {
                position: [-0.5, 0.5, 0.5], // Top-left-front
                tex_coords: [0., 0.],
            },
            // Bottom face
            TextureVertex {
                position: [-0.5, -0.5, -0.5], // Bottom-left-back
                tex_coords: [1., 0.],
            },
            TextureVertex {
                position: [0.5, -0.5, -0.5], // Bottom-right-back
                tex_coords: [0., 0.],
            },
            TextureVertex {
                position: [0.5, -0.5, 0.5], // Bottom-right-front
                tex_coords: [0., 1.],
            },
            TextureVertex {
                position: [-0.5, -0.5, 0.5], // Bottom-left-front
                tex_coords: [1., 1.],
            },
        ]
    }

    fn indexes(&self) -> Vec<u16> {
        vec![
            0, 2, 3, 2, 0, 1,
            4, 7, 6, 6, 5, 4, 
            8, 10, 11, 10, 8, 9,
            12, 15, 14, 14, 13, 12,
            16, 19, 18, 18, 17, 16,
            20, 22, 23, 22, 20, 21,
        ]
    }
}
