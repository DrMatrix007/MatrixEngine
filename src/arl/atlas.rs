use std::{collections::HashMap, sync::Arc};

use crate::arl::{
    device_queue::DeviceQueue,
    id::ID,
    models::{Model, ModelBuffer},
    passable::Passable,
    vertex::vertexable::{VertexIndexer, VertexableGroup},
};

pub struct Atlas<ModelID: ID, I: VertexIndexer, VGroup: VertexableGroup> {
    models: HashMap<ModelID, Arc<ModelBuffer<ModelID, I, VGroup>>>,
}

impl<ModelID: ID, I: VertexIndexer, VGroup: VertexableGroup> Default for Atlas<ModelID, I, VGroup> {
    fn default() -> Self {
        Self {
            models: Default::default(),
        }
    }
}

impl<ModelID: ID, I: VertexIndexer, VGroup: VertexableGroup> Atlas<ModelID, I, VGroup> {
    pub fn try_insert_model(
        &mut self,
        m: &dyn Model<ModelID, VGroup = VGroup, I = I>,
        device_queue: &DeviceQueue,
    ) {
        self.models.entry(m.id()).or_insert_with(|| {
            Arc::new(ModelBuffer::new_from_raw(
                m.vertecies(),
                m.indecies(),
                device_queue,
            ))
        });
    }

    pub fn draw_all(&self, pass: &mut wgpu::RenderPass<'_>) {
        for buffer in self.models.values() {
            buffer.apply(pass);
            pass.draw_indexed(0..buffer.index_size(), 0, 0..1);
        }
    }
}
