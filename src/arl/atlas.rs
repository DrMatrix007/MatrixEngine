use std::{collections::HashMap, sync::Arc};

use crate::arl::{
    bind_groups::{bind_group_registry::BindGroupGroupRegistry, bind_group_group::BindGroupGroup},
    device_queue::DeviceQueue,
    id::IDable,
    models::{Model, ModelBuffer, ModelIDable},
    passable::Passable,
    vertex::vertexable::{VertexIndexer, VertexableGroup},
};

pub struct Atlas<
    ModelID: ModelIDable,
    I: VertexIndexer,
    VGroup: VertexableGroup,
    BindGroups: BindGroupGroup,
> {
    models: HashMap<ModelID, Arc<ModelBuffer<ModelID, I, VGroup>>>,
    bind_groups: BindGroups::Registry,
}

impl<ModelID: IDable, I: VertexIndexer, VGroup: VertexableGroup, BindGroups: BindGroupGroup>
    Atlas<ModelID, I, VGroup, BindGroups>
{
    pub fn new(device_queue: &DeviceQueue) -> Self {
        Self {
            models: Default::default(),
            bind_groups: BindGroups::create_registry(device_queue),
        }
    }

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

    pub fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
        self.bind_groups.layout_desc()
    }
}
