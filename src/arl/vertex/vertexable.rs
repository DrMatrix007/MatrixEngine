use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};

use crate::{
    arl::{buffers::Buffer, vertex::vertex_buffers::VertexBufferGroup},
    impl_all,
};

pub trait Vertexable: Zeroable + Pod {
    fn desc() -> (wgpu::VertexStepMode, Vec<wgpu::VertexFormat>);
}

pub trait VertexableGroup {
    type ATTRS;
    type BufferGroup: VertexBufferGroup;

    fn attrs() -> Self::ATTRS;

    fn desc<'a>(attrs: &'a Self::ATTRS) -> Vec<wgpu::VertexBufferLayout<'a>>;
}

pub struct VertexBufferLayoutDescriptorHelper<T = ()> {
    step: wgpu::VertexStepMode,
    attrs: Vec<wgpu::VertexAttribute>,
    size: u64,
    marker: PhantomData<T>,
}

macro_rules! impl_tuple_vertex_buffer {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: Vertexable + 'static),+> VertexableGroup for ($($t,)+) {
            type ATTRS = ($(VertexBufferLayoutDescriptorHelper<$t>,)+);
            type BufferGroup = ($(Buffer<$t>,)+);

            fn attrs() -> Self::ATTRS {
                let mut shader_location = 0;
                ($({
                let mut addr_offset = 0;
                let d = $t::desc();
                let attrs = d
                        .1
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

                VertexBufferLayoutDescriptorHelper {
                    step: d.0,
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

pub trait VertexIndexer: Pod + Zeroable {
    fn format() -> wgpu::IndexFormat;
}

impl VertexIndexer for u32 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint32
    }
}

impl VertexIndexer for u16 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
}
