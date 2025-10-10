use std::any::TypeId;

use crate::arl::{matrix_renderer::matrix_vertex::MatrixVertex, models::Model};

#[derive(Debug, Hash, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct MatrixModelID(TypeId);

pub struct Pentagon;

impl Model<MatrixModelID> for Pentagon {
    type VGroup = (MatrixVertex,);

    type I = u16;

    fn id(&self) -> MatrixModelID {
        MatrixModelID(TypeId::of::<Self>())
    }

    fn vertecies(
        &self,
    ) -> <<Self::VGroup as crate::arl::vertex::vertexable::VertexableGroup>::BufferGroup as crate::arl::vertex::vertex_buffers::VertexBufferGroup>::Raw<'_>{
        (&[
            MatrixVertex {
                position: [-0.0868241, 0.49240386, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // A
            MatrixVertex {
                position: [-0.49513406, 0.06958647, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // B
            MatrixVertex {
                position: [-0.21918549, -0.44939706, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // C
            MatrixVertex {
                position: [0.35966998, -0.3473291, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // D
            MatrixVertex {
                position: [0.44147372, 0.2347359, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // E
        ],)
    }

    fn indecies(&self) -> &[Self::I] {
        &[0, 1, 4, 1, 2, 4, 2, 3, 4]
    }
}
