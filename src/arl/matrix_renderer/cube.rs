use std::any::TypeId;

use crate::arl::{
    matrix_renderer::{matrix_vertex::MatrixVertex, square::MatrixModelID},
    models::Model,
};

#[derive(Debug, Hash, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Cube;

static TYPE: TypeId = TypeId::of::<Cube>();

impl Model<MatrixModelID> for Cube {
    type VGroup = (MatrixVertex,);

    type I = u16;

    fn id(&self) -> MatrixModelID {
        MatrixModelID(TYPE)
    }

    fn vertecies(
        &self,
    ) -> <<Self::VGroup as crate::arl::vertex::vertexable::VertexableGroup>::BufferGroup as crate::arl::vertex::buffers::VertexBufferGroup>::Raw<'_>{
        (&[
            // Front face
            MatrixVertex {
                pos: [0.5, 0.5, 0.5], // A
                tex_pos: [1.0, 0.0],
            },
            MatrixVertex {
                pos: [-0.5, 0.5, 0.5], // B
                tex_pos: [0.0, 0.0],
            },
            MatrixVertex {
                pos: [0.5, -0.5, 0.5], // C
                tex_pos: [1.0, 1.0],
            },
            MatrixVertex {
                pos: [-0.5, -0.5, 0.5], // D
                tex_pos: [0.0, 1.0],
            },
            // Back face
            MatrixVertex {
                pos: [0.5, 0.5, -0.5], // E
                tex_pos: [0.0, 0.0],
            },
            MatrixVertex {
                pos: [-0.5, 0.5, -0.5], // F
                tex_pos: [1.0, 0.0],
            },
            MatrixVertex {
                pos: [0.5, -0.5, -0.5], // G
                tex_pos: [0.0, 1.0],
            },
            MatrixVertex {
                pos: [-0.5, -0.5, -0.5], // H
                tex_pos: [1.0, 1.0],
            },
        ],)
    }

    fn indecies(&self) -> &[Self::I] {
        &[
            // Front face
            0, 1, 2, 1, 3, 2, // Back face
            4, 6, 5, 5, 6, 7, // Right face
            0, 2, 4, 4, 2, 6, // Left face
            1, 5, 3, 5, 7, 3, // Top face
            0, 4, 1, 1, 4, 5, // Bottom face
            2, 3, 6, 3, 7, 6,
        ]
    }
}
