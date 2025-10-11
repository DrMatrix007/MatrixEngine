use crate::{
    arl::{
        buffered_vec::BufferedVec,
        buffers::Buffer,
        device_queue::DeviceQueue,
        vertex::{instantiable::Instantiable, vertexable::Vertexable},
    },
    impl_all,
};

use paste::paste;

pub trait VertexBufferGroup {
    type Raw<'a>;

    fn from<'a>(data: Self::Raw<'a>, device_queue: &DeviceQueue) -> Self;

    fn apply<'a>(&self, current_index: &mut u32, pass: &mut wgpu::RenderPass<'a>);
}

macro_rules! impl_tuple_vertex_buffer {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: Vertexable + 'static),+> VertexBufferGroup for ($(Buffer<$t>,)+) {
            type Raw<'a> = ($(&'a [$t],)+);

            fn from<'a>(data: Self::Raw<'a>, device_queue:&DeviceQueue) -> Self {
                let ($($t,)+) = data;
                ($(Buffer::new("tuple vertex buffer", $t, wgpu::BufferUsages::VERTEX, device_queue.clone()),)+)
            }

            fn apply<'a>(&self,current_index: &mut u32, pass: &mut wgpu::RenderPass<'a>) {
                let ($($t,)+) = self;
                $(
                    #[allow(unused_assignments)] // becasue the last index += 1 will not be read.
                    {
                        pass.set_vertex_buffer(*current_index,$t.raw().slice(..));

                        *current_index += 1;
                    }
                )+
            }
        }
    }
}

impl_all!(impl_tuple_vertex_buffer);

pub trait InstanceBufferGroup {
    type Raw;

    fn push(&mut self, data: Self::Raw);

    fn new(device_queue: &DeviceQueue) -> Self;

    fn apply<'a>(&self, current_index: &mut u32, pass: &mut wgpu::RenderPass<'a>);

    fn clear(&mut self);

    fn flush(&mut self);
}
macro_rules! impl_tuple_instance_buffer {
    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: Instantiable + 'static),+> InstanceBufferGroup for ($(BufferedVec<$t>,)+) {
            type Raw = ($($t,)+);

            fn new<'a>(device_queue: &DeviceQueue) -> Self {
                ($(BufferedVec::new(format!("tuple vertex buffer {}",core::any::type_name::<$t>()).as_str(), wgpu::BufferUsages::VERTEX, device_queue.clone()),)+)
            }

            fn push<'a>(&mut self, data: Self::Raw) {
                paste! {
                    let ($([<vec_ $t>],)+) = self;
                    let ($([<item_ $t>],)+) = data;
                    $(
                        [<vec_ $t>].push([<item_ $t>]);
                    )+
                }
            }


            fn apply<'a>(&self,current_index: &mut u32, pass: &mut wgpu::RenderPass<'a>) {
                let ($($t,)+) = self;
                $(
                    #[allow(unused_assignments)] // becasue the last index += 1 will not be read.
                    {
                        pass.set_vertex_buffer(*current_index, $t.buffer().raw().slice(..));

                        *current_index += 1;
                    }
                )+
            }

            fn clear(&mut self) {
                let ($($t,)+) = self;
                $(
                    {
                        $t.clear();
                    }
                )+

            }
            fn flush(&mut self) {
                let ($($t,)+) = self;
                $(
                    {
                        $t.flush();
                    }
                )+

            }
        }
    }
}

impl_all!(impl_tuple_instance_buffer);
