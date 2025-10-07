use crate::{
    arl::{buffers::Buffer, device_queue::DeviceQueue, vertex::Vertex},
    impl_all,
};

pub trait VertexBufferGroup {
    type Raw<'a>;

    fn from<'a>(data: Self::Raw<'a>, device_queue: &DeviceQueue) -> Self;

    fn apply<'a>(&self, pass: &mut wgpu::RenderPass<'a>);
}

macro_rules! impl_tuple_vertex_buffer {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: Vertex + 'static),+> VertexBufferGroup for ($(Buffer<$t>,)+) {
            type Raw<'a> = ($(&'a [$t],)+);

            fn from<'a>(data: Self::Raw<'a>, device_queue:&DeviceQueue) -> Self {
                let ($($t,)+) = data;
                ($(Buffer::new("tuple vertex buffer", $t, wgpu::BufferUsages::VERTEX, device_queue),)+)
            }

            fn apply<'a>(&self, pass: &mut wgpu::RenderPass<'a>) {
                let ($($t,)+) = self;
                let mut index = 0;
                $(
                    #[allow(unused_assignments)] // becasue the last index += 1 will not be read.
                    {
                        pass.set_vertex_buffer(index,$t.raw().slice(..));

                        index += 1;
                    }
                )+
            }


        }
    }
}

// impl_tuple_vertex_buffer!(A, B, C);
impl_all!(impl_tuple_vertex_buffer);
