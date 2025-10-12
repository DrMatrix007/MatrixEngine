use std::any::TypeId;

use crate::arl::{matrix_renderer::matrix_vertex::MatrixVertex, models::Model};

#[derive(Debug, Hash, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct MatrixModelID(TypeId);

pub struct Square;

static TYPE: TypeId = TypeId::of::<Square>();

impl Model<MatrixModelID> for Square {
    type VGroup = (MatrixVertex,);

    type I = u16;

    fn id(&self) -> MatrixModelID {
        MatrixModelID(TYPE)
    }

    fn vertecies(
        &self,
    ) -> <<Self::VGroup as crate::arl::vertex::vertexable::VertexableGroup>::BufferGroup as crate::arl::vertex::buffers::VertexBufferGroup>::Raw<'_>{
        (&[
            MatrixVertex {
                position: [0.5, 0.5, 0.0],
                color: [1.0, 0.0, 0.0], // Cool cyan
            }, // A
            MatrixVertex {
                position: [-0.5, 0.5, 0.0],
                color: [0.0,1.0,0.0], // Blue
            }, // B
            MatrixVertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0,0.0,1.0], // Electric purple
            }, // C
            MatrixVertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0,1.0,1.0], // Muted teal
            }, // D
        ],)
    }

    fn indecies(&self) -> &[Self::I] {
        &[0, 1, 2, 1, 3, 2]
        // &[0, 1, 2]
    }
}
