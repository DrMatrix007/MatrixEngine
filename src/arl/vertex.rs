use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};

use crate::{arl::{buffers::Buffer, vertex_buffers::VertexBufferGroup}, impl_all};

pub trait Vertex: Zeroable + Pod {
    fn desc() -> (wgpu::VertexStepMode, Vec<wgpu::VertexFormat>);
}

pub trait VertexGroup {
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

// impl<T: Vertex + 'static> VertexGroup for T {
//     type ATTRS = VertexBufferLayoutDescriptorHelper;
//     type Raw<'a> = &'a [T];

//     fn attrs() -> Self::ATTRS {
//         let d = T::desc();
//         let mut addr_offest = 0;
//         let mut shader_location = 0;
//         VertexBufferLayoutDescriptorHelper {
//             step: d.0,
//             attrs: d
//                 .1
//                 .iter()
//                 .map(move |format| {
//                     let attr = wgpu::VertexAttribute {
//                         format: *format,
//                         offset: addr_offest,
//                         shader_location,
//                     };
//                     shader_location += 1;
//                     addr_offest += format.size();

//                     attr
//                 })
//                 .collect(),
//             size: addr_offest,
//             marker: PhantomData,
//         }
//     }

//     fn desc<'a>(attrs: &'a Self::ATTRS) -> Vec<wgpu::VertexBufferLayout<'a>> {
//         vec![wgpu::VertexBufferLayout {
//             array_stride: attrs.size,
//             attributes: &attrs.attrs,
//             step_mode: attrs.step,
//         }]
//     }
// }

macro_rules! impl_tuple_vertex_buffer {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: Vertex + 'static),+> VertexGroup for ($($t,)+) {
            type ATTRS = ($(VertexBufferLayoutDescriptorHelper<$t>,)+);
            type BufferGroup = ($(Buffer<$t>,)+);

            fn attrs() -> Self::ATTRS {
                let mut shader_location = 0;
                ($({
                let mut addr_offest = 0;
                let d = $t::desc();
                VertexBufferLayoutDescriptorHelper {
                    step: d.0,
                    attrs: d
                        .1
                        .iter()
                        .map(move |format| {
                            let attr = wgpu::VertexAttribute {
                                format: *format,
                                offset: addr_offest,
                                shader_location,
                            };
                            shader_location += 1;
                            addr_offest += format.size();

                            attr
                        })
                        .collect(),
                    size: addr_offest,
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

// impl_tuple_vertex_buffer!(A, B, C);
impl_all!(impl_tuple_vertex_buffer);

pub trait Index : Pod + Zeroable {
    fn format() -> wgpu::IndexFormat;
}

impl Index for u32 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint32
    }
}

impl Index for u16 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
}
