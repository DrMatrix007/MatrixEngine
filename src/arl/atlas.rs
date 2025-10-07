use std::collections::HashMap;

use crate::arl::{
    device_queue::DeviceQueue,
    models::{Model, ModelBuffer, ModelID},
    vertex::{Index, VertexGroup},
};

pub struct Atlas<ID: ModelID, I: Index, VGroup: VertexGroup> {
    map: HashMap<ID, ModelBuffer<ID, I, VGroup>>,
}

impl<ID: ModelID, I: Index, VGroup: VertexGroup> Default for Atlas<ID, I, VGroup> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<ID: ModelID, I: Index, VGroup: VertexGroup> Atlas<ID, I, VGroup> {
    pub fn try_insert_model(
        &mut self,
        m: &dyn Model<ID, VGroup = VGroup, I = I>,
        device_queue: &DeviceQueue,
    ) {
        self.map.entry(m.id()).or_insert_with(|| {
            ModelBuffer::new_from_raw(m.vertecies(), m.indecies(), device_queue)
        });
    }
}
