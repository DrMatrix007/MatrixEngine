use std::{any::TypeId, collections::HashMap, sync::Arc};

use wgpu::{
    util::DeviceExt, Buffer, BufferAddress, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor,
};

use super::{
    pipelines::{
        bind_groups::bind_group::{MatrixBindGroup, MatrixBindGroupLayout},
        device_queue::DeviceQueue,
        models::Model,
        textures::MatrixTexture,
        vertecies::{texture_vertex::TextureVertex, Vertexable},
    },
    render_object::RenderObject,
};

#[derive(Debug)]
struct InstanceVector<T> {
    buffer: Arc<Buffer>,
    staging_buffer: Arc<Buffer>,
    size: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> InstanceVector<T> {
    pub fn new(device_queue: &DeviceQueue) -> Self {
        let size = 1;
        let buffer = device_queue
            .device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vector Buffer"),
                size: (size * std::mem::size_of::<T>()) as u64,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
        let staging_buffer = device_queue.device().create_buffer(&BufferDescriptor {
            label: Some("Stage Buffer"),
            size: std::mem::size_of::<T>() as BufferAddress,
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        Self {
            buffer: Arc::new(buffer),
            staging_buffer: Arc::new(staging_buffer),
            size,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn capacity(&self) -> usize {
        (self.buffer.size() as usize) / std::mem::size_of::<T>()
    }

    fn clear(&mut self) {
        self.size = 0;
    }

    pub fn push(&mut self, device_queue: &DeviceQueue, element: T) {
        // If the current size equals the capacity, we need to grow the buffer
        if self.size == self.capacity() {
            // Double the capacity or set it to 1 if it's 0
            let new_size = if self.size == 0 { 1 } else { self.size * 2 };

            // Create a new buffer with the new size
            let new_buffer = device_queue
                .device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Wgpu Vector Buffer"),
                    size: (new_size * std::mem::size_of::<T>()) as u64,
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });

            // Copy the existing buffer contents to the new buffer
            let mut encoder =
                device_queue
                    .device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Buffer Copy Encoder"),
                    });

            encoder.copy_buffer_to_buffer(
                &self.buffer,
                0,
                &new_buffer,
                0,
                (self.size * std::mem::size_of::<T>()) as u64,
            );

            device_queue.queue().submit(Some(encoder.finish()));

            // Replace the old buffer with the new buffer
            self.buffer = Arc::new(new_buffer);
        }

        // After ensuring enough space, we can add the new element
        device_queue.queue().write_buffer(
            &self.buffer,
            (self.size() * core::mem::size_of::<T>()) as BufferAddress,
            bytemuck::bytes_of(&element),
        );

        // Increment the size
        self.size += 1;
    }

    pub fn shrink_buffer(&mut self, device_queue: &DeviceQueue) {
        let mut new_size = self.capacity() / 2;
        if new_size < self.size() {
            return;
        }
        // Shrink the buffer by half until it fits the current number of elements
        while new_size > self.size {
            new_size /= 2;
        }

        let new_buffer = device_queue
            .device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shrinking Buffer"),
                size: (new_size * std::mem::size_of::<T>()) as u64,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
        let mut encoder = device_queue
            .device()
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("shrink command encoder"),
            });

        encoder.copy_buffer_to_buffer(
            &self.buffer,
            0,
            &new_buffer,
            0,
            (self.size() * core::mem::size_of::<T>()) as BufferAddress,
        );

        device_queue.queue().submit(Some(encoder.finish()));

        self.buffer = Arc::new(new_buffer);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstancedType {
    model: TypeId,
    texture_path: String,
}

impl InstancedType {
    pub fn new<V: Vertexable>(texture_path: String, model: &dyn Model<V>) -> Self {
        Self {
            texture_path,
            model: model.type_id(),
        }
    }
    pub fn from_obj(obj: &RenderObject) -> Self {
        Self::new(obj.texture_path.clone(), &*obj.model)
    }
}

pub struct InstanceData {
    transforms: InstanceVector<u8>,
    texture: MatrixTexture,
    texture_group: MatrixBindGroup<MatrixTexture>,
    vertex_buffer: Arc<Buffer>,
    index_buffer: Arc<Buffer>,
    num_indices: u32,
}

impl InstanceData {
    pub fn new(
        device_queue: &DeviceQueue,
        texture: MatrixTexture,
        texture_layout: &MatrixBindGroupLayout<MatrixTexture>,
        model: &dyn Model<TextureVertex>,
    ) -> Self {
        let vertex_buffer = Arc::new(device_queue.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("altas vertex buffer"),
                contents: bytemuck::cast_slice(model.vertices().as_slice()),
                usage: BufferUsages::VERTEX,
            },
        ));
        let indexes = model.indexes();
        let index_buffer = Arc::new(device_queue.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("altas index buffer"),
                contents: bytemuck::cast_slice(indexes.as_slice()),
                usage: BufferUsages::INDEX,
            },
        ));

        Self {
            transforms: InstanceVector::new(device_queue),
            texture_group: texture_layout.create_group(device_queue, &texture),
            texture,
            vertex_buffer,
            index_buffer,
            num_indices: indexes.len() as _,
        }
    }

    pub fn texture_group(&self) -> &MatrixBindGroup<MatrixTexture> {
        &self.texture_group
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}

pub(crate) struct Atlas {
    data: HashMap<InstancedType, InstanceData>,
}

impl Atlas {
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub(crate) fn reset(&mut self) {
        for k in self.data.values_mut() {
            k.transforms.clear();
        }
    }

    pub(crate) fn try_shrink(&mut self, device_queue: &DeviceQueue) {
        for k in self.data.values_mut() {
            k.transforms.shrink_buffer(device_queue);
        }
    }

    pub(crate) fn write(
        &mut self,
        device_queue: &DeviceQueue,
        obj: &RenderObject,
        texture_layout: &MatrixBindGroupLayout<MatrixTexture>,
    ) {
        let t = InstancedType::from_obj(obj);
        self.data.entry(t).or_insert_with(|| {
            InstanceData::new(
                device_queue,
                MatrixTexture::from_path(device_queue, &obj.texture_path).unwrap(),
                texture_layout,
                &*obj.model,
            )
        });
    }

    pub(crate) fn instances(&self) -> impl Iterator<Item = &'_ InstanceData> {
        self.data.values()
    }
}