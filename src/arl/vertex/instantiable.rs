use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};

use crate::{
    arl::{buffered_vec::BufferedVec, vertex::buffers::InstanceBufferGroup},
    impl_all,
};

pub trait Instantiable: Zeroable + Pod {
    fn desc() -> impl AsRef<[wgpu::VertexFormat]>;
}

pub trait InstantiableGroup {
    type ATTRS;
    type BufferGroup: InstanceBufferGroup;

    fn attrs() -> Self::ATTRS;

    fn desc<'a>(attrs: &'a Self::ATTRS) -> Vec<wgpu::VertexBufferLayout<'a>>;
}

pub struct InstanceBufferLayoutDescriptorHelper<T = ()> {
    step: wgpu::VertexStepMode,
    attrs: Vec<wgpu::VertexAttribute>,
    size: u64,
    marker: PhantomData<T>,
}
macro_rules! impl_tuple_vertex_buffer {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: Instantiable + 'static),+> InstantiableGroup for ($($t,)+) {
            type ATTRS = ($(InstanceBufferLayoutDescriptorHelper<$t>,)+);
            type BufferGroup = ($(BufferedVec<$t>,)+);

            fn attrs() -> Self::ATTRS {
                let mut shader_location = 0;
                ($({
                let mut addr_offset = 0;
                let d = $t::desc();
                let attrs = d.as_ref()
                        .iter()
                        .map(|format| {
                            let attr = wgpu::VertexAttribute {
                                format: *format,
                                offset: addr_offset,
                                shader_location,
                            };
                            shader_location += 1;
                            addr_offset += format.size();

                            attr
                        })
                        .collect();

                InstanceBufferLayoutDescriptorHelper {
                    step: wgpu::VertexStepMode::Instance,
                    attrs,
                    size: addr_offset,
                    marker: PhantomData
                }},)+)


            }

            fn desc<'a>(attrs: &'a Self::ATTRS) -> Vec<wgpu::VertexBufferLayout<'a>> {
                let ($($t,)+) = attrs;
                vec![$(wgpu::VertexBufferLayout {
                    array_stride: $t.size,
                    attributes: &$t.attrs,
                    step_mode: $t.step,
                }),+]
            }
        }
    };
}

impl_all!(impl_tuple_vertex_buffer);
