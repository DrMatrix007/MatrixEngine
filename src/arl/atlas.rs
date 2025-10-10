use std::{collections::HashMap, sync::Arc};

use crate::arl::{
    bind_groups::{
        bind_group_group::{BindGroupGroupRef, BindGroupableGroup},
        bind_group_registry::BindGroupGroupRegistry,
    },
    device_queue::DeviceQueue,
    id::IDable,
    id_to_entities::IdToEntitiesRegistry,
    models::{Model, ModelBuffer, ModelIDable},
    vertex::vertexable::{VertexIndexer, VertexableGroup},
};

pub struct Atlas<
    ModelID: ModelIDable,
    I: VertexIndexer,
    VGroup: VertexableGroup,
    BindGroups: BindGroupableGroup,
> {
    models: HashMap<ModelID, Arc<ModelBuffer<ModelID, I, VGroup>>>,
    bind_groups: BindGroups::Registry,
    entities: IdToEntitiesRegistry<(ModelID, BindGroups::ID)>,
}

impl<ModelID: IDable, I: VertexIndexer, VGroup: VertexableGroup, BindGroups: BindGroupableGroup>
    Atlas<ModelID, I, VGroup, BindGroups>
{
    pub fn new(device_queue: &DeviceQueue) -> Self {
        Self {
            models: Default::default(),
            bind_groups: BindGroups::create_registry(device_queue),
            entities: IdToEntitiesRegistry::new(),
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

    pub fn draw_all(&mut self, pass: &mut wgpu::RenderPass<'_>) {
        for (model_id, binds_id) in self.entities.iter_ids() {
            let model = self.models.get(model_id).unwrap();
            let binds = self.bind_groups.query_groups(binds_id);
            model.apply(pass);
            binds.apply(pass);

            pass.draw_indexed(0..model.index_size(), 0, 0..1);
        }
    }

    pub fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
        self.bind_groups.layout_desc()
    }

    pub fn entities(&self) -> &IdToEntitiesRegistry<(ModelID, BindGroups::ID)> {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut IdToEntitiesRegistry<(ModelID, BindGroups::ID)> {
        &mut self.entities
    }
}
