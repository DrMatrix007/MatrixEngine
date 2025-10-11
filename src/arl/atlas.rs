use std::{collections::HashMap, sync::Arc};

use crate::{
    arl::{
        bind_groups::{
            bind_group_group::{BindGroupGroupRef, BindGroupableGroup},
            bind_group_registry::BindGroupGroupRegistry,
        },
        device_queue::DeviceQueue,
        id::IDable,
        models::{Model, ModelBuffer, ModelIDable},
        vertex::{
            buffers::InstanceBufferGroup,
            instantiable::InstantiableGroup,
            vertexable::{VertexIndexer, VertexableGroup},
        },
    },
    utils::fast_vec::FastVec,
};

pub struct RenderArchetype<
    ModelID: ModelIDable,
    I: VertexIndexer,
    VGroup: VertexableGroup,
    InstanceGroup: InstantiableGroup,
    BindGroups: BindGroupableGroup,
> {
    pub model: Arc<ModelBuffer<ModelID, I, VGroup>>,
    pub instance_data: InstanceGroup::BufferGroup,
    pub bind_groups: BindGroups::BindGroups,
}

#[allow(clippy::type_complexity)]
pub struct Atlas<
    ModelID: ModelIDable,
    I: VertexIndexer,
    VGroup: VertexableGroup,
    InstanceGroup: InstantiableGroup,
    BindGroups: BindGroupableGroup,
> {
    models: HashMap<ModelID, Arc<ModelBuffer<ModelID, I, VGroup>>>,
    bind_groups_registry: BindGroups::Registry,
    instances: FastVec<
        (ModelID, BindGroups::ID),
        RenderArchetype<ModelID, I, VGroup, InstanceGroup, BindGroups>,
    >,
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
            instances: Default::default(),
            device_queue,
        }
    }

    pub fn try_register_model(&mut self, m: &dyn Model<ModelID, VGroup = VGroup, I = I>) {
        let id = m.id();
        if !self.models.contains_key(&id) {
            self.models.insert(
                id,
                Arc::new(ModelBuffer::new_from_raw(
                    m.vertecies(),
                    m.indecies(),
                    &self.device_queue,
                )),
            );
        }
    }

    pub fn try_register_bind_groups(&mut self, id: &BindGroups::ID) {
        self.bind_groups_registry.query_groups(id);
    }

    pub fn get_index(&mut self, id: &(ModelID, BindGroups::ID)) -> usize {
        match self.instances.get_index_by_id(id) {
            Some((index, _)) => index,
            None => {
                self.instances
                    .push(
                        *id,
                        RenderArchetype {
                            bind_groups: self.bind_groups_registry.query_groups(&id.1),
                            instance_data: InstanceGroup::BufferGroup::new(&self.device_queue),
                            model: self.models.get(&id.0).unwrap().clone(),
                        },
                    )
                    .0
            }
        }
    }

    // pub fn try_insert_instance(
    //     &mut self,
    //     model_id: &ModelID,
    //     bind_group_id: &BindGroups::ID,
    // ) -> usize {
    //     let id = (*model_id, *bind_group_id);
    //     match self.instances.get_index_by_id(&id) {
    //         None => {
    //             self.instances
    //                 .push(id, InstanceGroup::BufferGroup::new(&self.device_queue))
    //                 .0
    //         }
    //         Some(index) => index.0,
    //     }
    // }

    pub fn draw_all(&mut self, pass: &mut wgpu::RenderPass<'_>) {
        for (_, archetype) in self.instances.iter() {
            let mut index = 0;

            let model = &archetype.model;
            let binds = &archetype.bind_groups;
            let instances = &archetype.instance_data;

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

    pub fn iter_instances(
        &mut self,
    ) -> impl Iterator<Item = &mut RenderArchetype<ModelID, I, VGroup, InstanceGroup, BindGroups>>
    {
        self.instances.iter_mut().map(|(_, instance)| instance)
    }

    pub fn instance_at(
        &mut self,
        index: usize,
    ) -> Option<&mut RenderArchetype<ModelID, I, VGroup, InstanceGroup, BindGroups>> {
        self.instances.get_mut(index)
    }
}
