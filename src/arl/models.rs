use core::marker::PhantomData;

use crate::arl::{
    buffers::Buffer,
    device_queue::DeviceQueue,
    id::IDable,
    vertex::{
        vertex_buffers::VertexBufferGroup,
        vertexable::{VertexIndexer, VertexableGroup},
    },
};

pub trait ModelIDable: IDable {}

impl<T: IDable> ModelIDable for T {}

pub trait Model<ModelID: ModelIDable> {
    type VGroup: VertexableGroup;
    type I: VertexIndexer;
    fn id(&self) -> ModelID;

    fn vertecies(
        &self,
    ) -> <<Self::VGroup as VertexableGroup>::BufferGroup as VertexBufferGroup>::Raw<'_>;
    fn indecies(&self) -> &[Self::I];
}

pub struct ModelBuffer<ModelID: ModelIDable, I: VertexIndexer, VGroup: VertexableGroup> {
    index_buffer: Buffer<I>,
    vertex_buffers: VGroup::BufferGroup,
    marker: PhantomData<(ModelID, I, VGroup)>,
}

impl<ID: ModelIDable, I: VertexIndexer, VGroup: VertexableGroup> ModelBuffer<ID, I, VGroup> {
    pub fn new_from_raw(
        vertex_raw: <VGroup::BufferGroup as VertexBufferGroup>::Raw<'_>,
        indexes: &[I],
        device_queue: &DeviceQueue,
    ) -> Self {
        Self {
            index_buffer: Buffer::new(
                "index buffer",
                indexes,
                wgpu::BufferUsages::INDEX,
                device_queue.clone(),
            ),
            vertex_buffers: <VGroup::BufferGroup as VertexBufferGroup>::from(
                vertex_raw,
                device_queue,
            ),
            marker: PhantomData,
        }
    }
    pub fn index_size(&self) -> u32 {
        self.index_buffer.len() as u32
    }

    pub fn index_buffer(&self) -> &Buffer<I> {
        &self.index_buffer
    }

    pub fn vertex_buffers(&self) -> &VGroup::BufferGroup {
        &self.vertex_buffers
    }

    pub fn apply<'a>(&self, pass: &mut wgpu::RenderPass<'a>) {
        self.vertex_buffers.apply(pass);
        pass.set_index_buffer(self.index_buffer.raw().slice(..), I::format());
    }
}
