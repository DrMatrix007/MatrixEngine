use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use wgpu::BufferUsages;

use crate::renderer::matrix_renderer::{render_object::RenderObject, renderer_system::DeviceQueue};

use super::{
    bind_groups::BindGroupContainer,
    buffers::{BufferContainer, Bufferable, Vertex, VertexBuffer},
    group_layout_manager::BindGroupLayoutAtlas,
    texture::MatrixTexture,
    transform::{InstanceTransform, Transform},
};

pub trait VertexStructure<Vertex: Bufferable>: Any {
    fn create_buffer(&self, device: &DeviceQueue) -> VertexBuffer<Vertex>;
}

pub struct InstancedData {
    texture: MatrixTexture,
    texture_group: BindGroupContainer<(MatrixTexture,)>,
    transform_buffer: BufferContainer<InstanceTransform>,
    transform_vec: Vec<InstanceTransform>,
    buffer: Arc<VertexBuffer<Vertex>>,
}

impl InstancedData {
    pub fn new(
        texture_name: &str,
        device: &DeviceQueue,
        buffer: Arc<VertexBuffer<Vertex>>,
        manager: &mut BindGroupLayoutAtlas,
    ) -> Self {
        let t = MatrixTexture::from_name(texture_name, device, "instanced generated texture")
            .expect("this shouldnt be implemnted now");
        let group = manager.create_group::<(MatrixTexture,)>((&t,));
        Self {
            texture: t,
            buffer,
            transform_vec: Vec::new(),
            texture_group: group,
            transform_buffer: BufferContainer::create_buffer(
                &InstanceTransform::default(),
                device,
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
                false,
            ),
        }
    }

    fn prepare_capacity(&mut self, device: &DeviceQueue) -> bool {
        // if self.transform_buffer.size() < count || (self.transform_buffer.size() / 2) > count {
        //     let new_size = (2_u32).pow((count as f32).log2().ceil() as u32);
        //     self.transform_buffer = BufferContainer::create_with_size(
        //         count,
        //         device,
        //         self.transform_buffer.usage(),
        //         true,
        //     );
        //     true
        // } else {
        //     false
        // }

        if self.transform_buffer.size() as usize != self.transform_vec.capacity() {
            self.transform_buffer = BufferContainer::create_with_size(
                self.transform_vec.capacity() as u64,
                device,
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
                false,
            );
            device.queue().write_buffer(
                self.transform_buffer.buffer(),
                0,
                bytemuck::cast_slice(&self.transform_vec),
            );

            println!("allocated! {}", self.transform_vec.capacity());
            return true;
        }
        device.queue().write_buffer(
            self.transform_buffer.buffer(),
            0,
            bytemuck::cast_slice(&self.transform_vec),
        );
        false
    }

    pub fn texture_group(&self) -> &BindGroupContainer<(MatrixTexture,)> {
        &self.texture_group
    }

    pub fn transform_buffer(&self) -> &BufferContainer<InstanceTransform> {
        &self.transform_buffer
    }

    pub fn structure_buffer(&self) -> &VertexBuffer<Vertex> {
        self.buffer.as_ref()
    }

    pub fn push(&mut self, raw: InstanceTransform, device: &DeviceQueue) {
        self.transform_vec.push(raw);

        if self.transform_vec.capacity() != self.transform_buffer.size() as usize {}
    }

    pub fn clear(&mut self) {
        self.transform_vec.clear();
    }

    pub fn instace_count(&self) -> u32 {
        self.transform_buffer.size() as u32
    }
}

pub struct InstanceAtlas {
    device: DeviceQueue,
    data: HashMap<(TypeId, String), InstancedData>,
    buffer: HashMap<TypeId, (u64, Arc<VertexBuffer<Vertex>>)>,
}

impl InstanceAtlas {
    pub fn new(device: DeviceQueue) -> Self {
        Self {
            device,
            buffer: Default::default(),
            data: Default::default(),
        }
    }

    pub fn register_object(
        &mut self,
        obj: &RenderObject,
        transform: &Transform,
        group_manager: &mut BindGroupLayoutAtlas,
    ) {
        self.data
            .entry((obj.structure_type_id(), obj.texture_name().into()))
            .or_insert_with(|| {
                InstancedData::new(
                    obj.texture_name(),
                    &self.device,
                    self.buffer
                        .entry(obj.structure_type_id())
                        .or_insert_with(|| (1, Arc::new(obj.create_buffer(&self.device))))
                        .1
                        .clone(),
                    group_manager,
                )
            })
            .push(InstanceTransform::from(transform), &self.device);
        self.buffer
            .entry(obj.structure_type_id())
            .and_modify(|(x, _)| *x += 1)
            .or_insert_with(|| (1, Arc::new(obj.create_buffer(&self.device))));
    }
    pub fn prepare(&mut self) -> bool {
        self.data.retain(|_, data| data.instace_count() > 0);
        self.data
            .iter_mut()
            .map(|((_structure, _texture_name), data)| data.prepare_capacity(&self.device))
            .any(|x| x)
    }
    pub fn iter_data(&self) -> impl Iterator<Item = &'_ InstancedData> {
        self.data.values()
    }
    pub fn clear(&mut self) {
        for data in self.data.values_mut() {
            data.clear();
        }
    }
}
