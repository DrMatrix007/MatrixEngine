use crate::arl::{
    matrix_renderer::{matrix_vertex::MatrixVertex, pentagon::MatrixModelID},
    models::Model,
};

pub struct MatrixRenderObject {
    model: Box<dyn Model<MatrixModelID, VGroup = (MatrixVertex,), I = u16> + Send + Sync>,
    model_id: MatrixModelID,
    added: bool,
    render_archetype_index: Option<usize>,
}

impl MatrixRenderObject {
    pub fn new(
        data: impl Model<MatrixModelID, VGroup = (MatrixVertex,), I = u16> + Send + Sync + 'static,
    ) -> Self {
        Self {
            model_id: data.id(),
            model: Box::new(data),
            added: false,
            render_archetype_index: None,
        }
    }

    pub fn object(&self) -> &dyn Model<MatrixModelID, I = u16, VGroup = (MatrixVertex,)> {
        self.model.as_ref()
    }

    pub fn bind_groups_id(&self) {}

    pub fn is_added(&self) -> bool {
        self.added
    }

    pub fn set_added(&mut self, added: bool) {
        self.added = added;
    }

    pub fn model_id(&self) -> &MatrixModelID {
        &self.model_id
    }

    pub fn render_archetype_index(&self) -> Option<usize> {
        self.render_archetype_index
    }

    pub fn set_render_archetype_index(&mut self, render_archetype_index: usize) {
        self.render_archetype_index = Some(render_archetype_index);
    }
    pub fn clear_render_archetype_index(&mut self) {
        self.render_archetype_index = None;
    }
}
