use std::{hash::Hash, marker::PhantomData};

use crate::arl::{
    buffers::Buffer,
    device_queue::DeviceQueue,
    passable::Passable,
    vertex::{Index, Vertex, VertexGroup},
    vertex_buffers::VertexBufferGroup,
};

pub trait ModelID: Hash + Eq {}
impl<T: Hash + Eq> ModelID for T {}

pub trait Model<ID: ModelID> {
    type VGroup: VertexGroup;
    type I: Index;
    fn id(&self) -> ID;

    fn vertecies(
        &self,
    ) -> <<Self::VGroup as VertexGroup>::BufferGroup as VertexBufferGroup>::Raw<'_>;
    fn indecies(&self) -> &[Self::I];
}

pub struct ModelBuffer<ID: ModelID, I: Index, VGroup: VertexGroup> {
    index_buffer: Buffer<I>,
    vertex_buffers: VGroup::BufferGroup,
    marker: PhantomData<(ID, I, VGroup)>,
}

impl<ID: ModelID, I: Index, VGroup: VertexGroup> Passable for ModelBuffer<ID, I, VGroup> {
    fn apply<'a>(&self, pass: &mut wgpu::RenderPass<'a>) {
        self.vertex_buffers.apply(pass);
        pass.set_index_buffer(self.index_buffer.raw().slice(..), I::format());
    }
}

impl<ID: ModelID, I: Index, VGroup: VertexGroup> ModelBuffer<ID, I, VGroup> {
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
                device_queue,
            ),
            vertex_buffers: <VGroup::BufferGroup as VertexBufferGroup>::from(
                vertex_raw,
                device_queue,
            ),
            marker: PhantomData,
        }
    }
}
