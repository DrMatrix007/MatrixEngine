use std::any::TypeId;

use crate::arl::{matrix_renderer::matrix_vertex::MatrixVertex, models::Model};

pub struct MatrixRenderObject {
    data: Box<dyn Model<TypeId, VGroup = (MatrixVertex,), I = u16> + Send + Sync>,
}

impl MatrixRenderObject {
    pub fn new(
        data: impl Model<TypeId, VGroup = (MatrixVertex,), I = u16> + Send + Sync + 'static,
    ) -> Self {
        Self {
            data: Box::new(data),
        }
    }

    pub fn object(&self) -> &dyn Model<TypeId, I = u16, VGroup = (MatrixVertex,)> {
        self.data.as_ref()
    }
}
