use crate::arl::{
    matrix_renderer::{matrix_vertex::MatrixVertex, pentagon::MatrixModelID},
    models::Model,
};

pub struct MatrixRenderObject {
    model: Box<dyn Model<MatrixModelID, VGroup = (MatrixVertex,), I = u16> + Send + Sync>,
    model_id: MatrixModelID,
    added: bool,
    model_ptr: Option<usize>,
    instance_ptr: Option<usize>,
    bind_groups_ptr: Option<()>,
}

impl MatrixRenderObject {
    pub fn new(
        data: impl Model<MatrixModelID, VGroup = (MatrixVertex,), I = u16> + Send + Sync + 'static,
    ) -> Self {
        Self {
            model_id: data.id(),
            model: Box::new(data),
            added: false,
            model_ptr: None,
            bind_groups_ptr: None,
            instance_ptr: None
        }
    }

    pub fn object(&self) -> &dyn Model<MatrixModelID, I = u16, VGroup = (MatrixVertex,)> {
        self.model.as_ref()
    }

    pub fn bind_groups_id(&self) -> &() {
        &()
    }

    pub fn is_added(&self) -> bool {
        self.added
    }

    pub fn set_added(&mut self, added: bool) {
        self.added = added;
    }

    pub fn model_id(&self) -> &MatrixModelID {
        &self.model_id
    }

    pub fn model_ptr(&self) -> Option<usize> {
        self.model_ptr
    }

    pub fn set_model_ptr(&mut self, model_ptr: Option<usize>) {
        self.model_ptr = model_ptr;
    }

    pub fn bind_groups_ptr(&self) -> Option<()> {
        self.bind_groups_ptr
    }

    pub fn set_bind_groups_ptr(&mut self, bind_groups_ptr: Option<()>) {
        self.bind_groups_ptr = bind_groups_ptr;
    }
    
    pub fn instance_ptr(&self) -> Option<usize> {
        self.instance_ptr
    }
    
    pub fn set_instance_ptr(&mut self, instance_ptr: Option<usize>) {
        self.instance_ptr = instance_ptr;
    }
}
