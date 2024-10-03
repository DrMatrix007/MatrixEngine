use std::any::TypeId;

use super::pipelines::{models::Model, vertecies::texture_vertex::TextureVertex};

pub struct RenderObject {
    pub(crate) model: Box<dyn Model<TextureVertex>>,
    pub(crate) model_type_id: TypeId,
    pub(crate) texture_path: String,
}

impl RenderObject {
    pub fn new(model: impl Model<TextureVertex>, texture_path: String) -> Self {
        Self {
            model_type_id: model.type_id(),
            model: Box::new(model),
            texture_path,
        }
    }
}
