use std::{collections::HashMap, sync::Arc};

use crate::{
    arl::{
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
    },
    utils::fast_vec::FastVec,
};

pub struct Atlas<
    ModelID: ModelIDable,
    I: VertexIndexer,
    VGroup: VertexableGroup,
    InstanceGroup: InstantiableGroup,
    BindGroups: BindGroupableGroup,
> {
    models: FastVec<ModelID, Arc<ModelBuffer<ModelID, I, VGroup>>>,
    bind_groups_registry: BindGroups::Registry,
    entities: IdToEntitiesRegistry<(ModelID, BindGroups::ID)>,
    instances: FastVec<(ModelID, BindGroups::ID), InstanceGroup::BufferGroup>,
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
            bind_groups_registry: BindGroups::create_registry(&device_queue),
            entities: IdToEntitiesRegistry::new(),
            instances: Default::default(),
            device_queue,
        }
    }

    pub fn try_insert_model(
        &mut self,
        m: &dyn Model<ModelID, VGroup = VGroup, I = I>,
        device_queue: &DeviceQueue,
    ) -> usize {
        let id = m.id();
        match self.models.get_index_by_id(&id) {
            None => {
                self.models
                    .push(
                        id,
                        Arc::new(ModelBuffer::new_from_raw(
                            m.vertecies(),
                            m.indecies(),
                            device_queue,
                        )),
                    )
                    .0
            }
            Some(index) => index.0,
        }
    }

    pub fn try_insert_bind_groups(
        &mut self,
        id: &BindGroups::ID,
        device_queue: &DeviceQueue,
    ) -> <BindGroups::Registry as BindGroupGroupRegistry>::Input {
        self.bind_groups_registry.try_create(id)
    }

    pub fn try_insert_instance(
        &mut self,
        model_id: &ModelID,
        bind_group_id: &BindGroups::ID,
    ) -> usize {
        let id = (*model_id, *bind_group_id);
        match self.instances.get_index_by_id(&id) {
            None => {
                self.instances
                    .push(id, InstanceGroup::BufferGroup::new(&self.device_queue))
                    .0
            }
            Some(index) => index.0,
        }
    }

    pub fn draw_all(&mut self, pass: &mut wgpu::RenderPass<'_>) {
        for id in self.entities.iter_ids() {
            let mut index = 0;
            let instances = self.instances.entry(*id).or_insert_with(|| {
                <InstanceGroup::BufferGroup as InstanceBufferGroup>::new(&self.device_queue)
            });
            let model = self.models.get(&id.0).unwrap();
            let binds = self.bind_groups_registry.query_groups(&id.1);

            let instance_count = instances.len();
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
        self.bind_groups_registry.layout_desc()
    }

    pub fn entities(&self) -> &IdToEntitiesRegistry<(ModelID, BindGroups::ID)> {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut IdToEntitiesRegistry<(ModelID, BindGroups::ID)> {
        &mut self.entities
    }

    pub fn iter_instances(&mut self) -> impl Iterator<Item = &mut InstanceGroup::BufferGroup> {
        self.instances.iter_mut().map(|(_, d)| d)
    }
    
    pub fn instances_mut(&mut self) -> &mut FastVec<(ModelID, BindGroups::ID), InstanceGroup::BufferGroup> {
        &mut self.instances
    }
}
