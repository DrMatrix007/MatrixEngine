use std::{any::TypeId, collections::HashMap, sync::Arc};

use wgpu::{
    util::DeviceExt, Buffer, BufferAddress, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, ShaderStages,
};

use crate::engine::transform::{Transform, TransformMat, TransformRaw};

use super::{
    pipelines::{
        bind_groups::{
            bind::MatrixBindable,
            bind_group::{MatrixBindGroup, MatrixBindGroupLayout},
        },
        compute_pipeline::{MatrixComputePipeline, MatrixComputePipelineArgs},
        device_queue::DeviceQueue,
        models::Model,
        shaders::MatrixShaders,
        textures::MatrixTexture,
        vertecies::texture_vertex::TextureVertex,
    },
    render_object::RenderObject,
};

#[derive(Debug)]
pub struct InstanceVector<T> {
    buffer: Arc<Buffer>,
    staging_vec: Vec<T>,
    size: usize,
    is_recreated: bool,
    is_uniform: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> InstanceVector<T> {
    pub fn new(device_queue: &DeviceQueue, is_uniform: bool) -> Self {
        let size = 1;
        let buffer = device_queue
            .device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vector Buffer"),
                size: (size * std::mem::size_of::<T>()) as u64,
                usage: BufferUsages::VERTEX
                    | BufferUsages::COPY_DST
                    | BufferUsages::COPY_SRC
                    | BufferUsages::STORAGE,
                mapped_at_creation: false,
            });
        Self {
            buffer: Arc::new(buffer),
            staging_vec: Vec::new(),
            size,
            is_uniform,
            is_recreated: true,
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
        self.staging_vec.clear();
        self.is_recreated = false;
    }

    pub fn push(&mut self, element: T) {
        self.staging_vec.push(element);
        // If the current size equals the capacity, we need to grow the buffer
    }
    pub fn save_to_buffer(&mut self, device_queue: &DeviceQueue) {
        if self.staging_vec.capacity() > self.capacity() {
            // Double the capacity or set it to 1 if it's 0
            let new_size = self.staging_vec.capacity();

            // Create a new buffer with the new size
            let new_buffer = device_queue
                .device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Wgpu Vector Buffer"),
                    size: (new_size * std::mem::size_of::<T>()) as u64,
                    usage: BufferUsages::VERTEX
                        | BufferUsages::COPY_DST
                        | BufferUsages::COPY_SRC
                        | if self.is_uniform {
                            BufferUsages::UNIFORM
                        } else {
                            BufferUsages::STORAGE
                        },
                    mapped_at_creation: false,
                });
            self.is_recreated = true;
            self.buffer = Arc::new(new_buffer);
        }

        // After ensuring enough space, we can add the new element
        device_queue.queue().write_buffer(
            &self.buffer,
            (self.size() * core::mem::size_of::<T>()) as BufferAddress,
            bytemuck::cast_slice(self.staging_vec.as_slice()),
        );

        // Increment the size
        self.size = self.staging_vec.len();
    }

    pub fn shrink_buffer(&mut self, device_queue: &DeviceQueue) {
        let mut new_size = self.capacity() / 2;
        if new_size < self.size() {
            return;
        }
        // Shrink the buffer by half until it fits the current number of elements
        while new_size > self.size * 2 {
            new_size /= 2;
        }

        let new_buffer = device_queue
            .device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Shrinking Buffer"),
                size: (new_size * std::mem::size_of::<T>()) as u64,
                usage: BufferUsages::VERTEX
                    | BufferUsages::COPY_DST
                    | BufferUsages::COPY_SRC
                    | if self.is_uniform {
                        BufferUsages::UNIFORM
                    } else {
                        BufferUsages::STORAGE
                    },

                mapped_at_creation: false,
            });
        self.is_recreated = true;
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

    fn is_recreated(&self) -> bool {
        self.is_recreated
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstancedType {
    model: TypeId,
    texture_path: String,
}

impl InstancedType {
    pub fn new(texture_path: String, model: TypeId) -> Self {
        Self {
            texture_path,
            model,
        }
    }
    pub fn from_obj(obj: &RenderObject) -> Self {
        Self::new(obj.texture_path.clone(), obj.model_type_id)
    }
}

pub struct TransformsRaw {
    pub transforms_raw: InstanceVector<TransformRaw>,
}
impl MatrixBindable for TransformsRaw {
    fn bind_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn bind_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(
                self.transforms_raw.buffer().as_entire_buffer_binding(),
            ),
        }
    }
}

pub struct Transforms {
    pub transforms: InstanceVector<TransformMat>,
}

impl MatrixBindable for Transforms {
    fn bind_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn bind_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: self.transforms.buffer().as_entire_binding(),
        }
    }
}
pub struct InstanceData {
    transforms: Transforms,
    transforms_group: MatrixBindGroup<Transforms>,
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
        transforms_layout: &MatrixBindGroupLayout<Transforms>,
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
        let transforms = Transforms {
            transforms: InstanceVector::new(device_queue, false),
        };
        Self {
            transforms_group: transforms_layout.create_group(device_queue, &transforms),
            transforms,
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

    pub(crate) fn instance_buffer(&self) -> &Buffer {
        self.transforms.transforms.buffer()
    }

    pub(crate) fn instaces(&self) -> u32 {
        self.transforms.transforms.size() as _
    }
}

pub(crate) struct Atlas {
    data: HashMap<InstancedType, InstanceData>,
    compute_pipeline: MatrixComputePipeline<(Transforms,)>,
}

impl Atlas {
    pub(crate) fn new(device_queue: &DeviceQueue) -> Self {
        Self {
            data: HashMap::new(),
            compute_pipeline: MatrixComputePipeline::new(MatrixComputePipelineArgs {
                label: "atlas compute pipeline",
                device_queue: device_queue.clone(),
                shaders: MatrixShaders::new(device_queue, include_str!("transform_shaders.wgsl")),
            }),
        }
    }

    pub(crate) fn reset(&mut self) {
        for k in self.data.values_mut() {
            k.transforms.transforms.clear();
        }
    }

    pub(crate) fn try_shrink(&mut self, device_queue: &DeviceQueue) {
        for k in self.data.values_mut() {
            k.transforms.transforms.shrink_buffer(device_queue);
        }
    }

    pub(crate) fn write(
        &mut self,
        device_queue: &DeviceQueue,
        obj: &RenderObject,
        tranform: &Transform,
        texture_layout: &MatrixBindGroupLayout<MatrixTexture>,
    ) {
        let t = InstancedType::from_obj(obj);
        let state = self.data.entry(t).or_insert_with(|| {
            InstanceData::new(
                device_queue,
                MatrixTexture::from_path(device_queue, &obj.texture_path).unwrap(),
                texture_layout,
                &self.compute_pipeline.layouts().0,
                &*obj.model,
            )
        });
        state.transforms.transforms.push(TransformMat {
            mat: tranform.raw().mat,
        });
    }

    pub(crate) fn instances(&self) -> impl Iterator<Item = &'_ InstanceData> {
        self.data.values()
    }

    pub(crate) fn update_buffers(&mut self, device_queue: &DeviceQueue) {
        for i in self.data.values_mut() {
            i.transforms.transforms.save_to_buffer(device_queue);
        }

        let mut command_encoder =
            device_queue
                .device()
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("atlas command encoder"),
                });
        {
            let mut compute_pass = command_encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("atlas compute pass"),
                timestamp_writes: None,
            });

            for i in self.data.values_mut() {
                self.compute_pipeline.setup_pass(&mut compute_pass);

                if i.transforms.transforms.is_recreated() {
                    i.transforms_group = self
                        .compute_pipeline
                        .layouts()
                        .0
                        .create_group(device_queue, &i.transforms);
                }

                self.compute_pipeline
                    .setup_groups(&mut compute_pass, (&i.transforms_group,));
                compute_pass.dispatch_workgroups((i.instaces() + 256 - 1) / 256, 1, 1);
            }
        }
        device_queue
            .queue()
            .submit(core::iter::once(command_encoder.finish()));
    }
}
