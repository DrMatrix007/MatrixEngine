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
    vertex::{
        buffers::InstanceBufferGroup,
        instantiable::InstantiableGroup,
        vertexable::{VertexIndexer, VertexableGroup},
    },
};

pub struct Atlas<
    ModelID: ModelIDable,
    I: VertexIndexer,
    VGroup: VertexableGroup,
    InstanceGroup: InstantiableGroup,
    BindGroups: BindGroupableGroup,
> {
    models: HashMap<ModelID, Arc<ModelBuffer<ModelID, I, VGroup>>>,
    bind_groups: BindGroups::Registry,
    entities: IdToEntitiesRegistry<(ModelID, BindGroups::ID)>,
    instances: HashMap<(ModelID, BindGroups::ID), InstanceGroup::BufferGroup>,
    device_queue: DeviceQueue,
}

impl<
    ModelID: IDable,
    I: VertexIndexer,
    VGroup: VertexableGroup,
    InstanceGroup: InstantiableGroup,
    BindGroups: BindGroupableGroup,
> Atlas<ModelID, I, VGroup, InstanceGroup, BindGroups>
{
    pub fn new(device_queue: DeviceQueue) -> Self {
        Self {
            models: Default::default(),
            bind_groups: BindGroups::create_registry(&device_queue),
            entities: IdToEntitiesRegistry::new(),
            instances: Default::default(),
            device_queue,
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
        for id in self.entities.iter_ids() {
            let mut index = 0;
            let instances = self.instances.entry(*id).or_insert_with(|| {
                <InstanceGroup::BufferGroup as InstanceBufferGroup>::new(&self.device_queue)
            });
            let model = self.models.get(&id.0).unwrap();
            let binds = self.bind_groups.query_groups(&id.1);

            let instance_count= instances.len();
            if instance_count == 0 {
                continue;
            }

            model.apply(&mut index, pass);
            instances.apply(&mut index, pass);
            binds.apply(pass);

            pass.draw_indexed(0..model.index_size(), 0, 0..instance_count);
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

    pub fn instances(&self) -> &HashMap<(ModelID, BindGroups::ID), InstanceGroup::BufferGroup> {
        &self.instances
    }

    pub fn instances_mut(
        &mut self,
    ) -> &mut HashMap<(ModelID, BindGroups::ID), InstanceGroup::BufferGroup> {
        &mut self.instances
    }
}
