use std::{marker::PhantomData, sync::Arc};

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Device, ShaderStages,
};

use crate::impl_all;

use super::texture::MatrixTexture;

pub trait BindDataEntry {
    type Args<'a>;

    fn layout_entries() -> Box<dyn Iterator<Item = BindGroupLayoutEntry>>;

    fn entries<'a>(args: Self::Args<'a>) -> Box<dyn Iterator<Item = BindGroupEntry<'a>> + 'a>;
}

impl BindDataEntry for MatrixTexture {
    type Args<'a> = &'a Self;

    fn layout_entries() -> Box<dyn Iterator<Item = BindGroupLayoutEntry>> {
        Box::new(
            std::iter::once(BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            })
            .chain(std::iter::once(BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            })),
        )
    }

    fn entries<'a>(args: Self::Args<'a>) -> Box<dyn Iterator<Item = BindGroupEntry<'a>> + 'a> {
        Box::new(
            std::iter::once(BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(args.view()),
            })
            .chain(std::iter::once(BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(args.sampler()),
            })),
        )
    }
}
pub trait BindData {
    type Args<'a>;

    fn create_layout(label: &str, device: &Device) -> BindGroupLayoutContainer<Self>
    where
        Self: Sized;

    fn create_group(
        device: &Device,
        layout: &BindGroupLayoutContainer<Self>,
        args: Self::Args<'_>,
    ) -> BindGroupContainer<Self>
    where
        Self: Sized;
}

macro_rules! impl_bind_data {
    ($($t:ident),+) => {
    #[allow(non_snake_case)]
        impl<$($t:BindDataEntry,)+> BindData for ($($t,)+) {
            type Args<'a> = ($($t::Args<'a>,)+);

            fn create_layout(label:&str,device: &Device) -> BindGroupLayoutContainer<Self>
            where
                Self: Sized {
                    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
                        label:Some(label),
                        entries: &([$($t::layout_entries(),)+].into_iter().flatten().collect::<Vec<_>>())

                    });
                    BindGroupLayoutContainer {
                        marker: PhantomData,
                        layout: Arc::new(layout),
                    }
                }

            #[allow(non_snake_case)]
            fn create_group(device: &Device, layout: &BindGroupLayoutContainer<Self>,args: Self::Args<'_>) -> BindGroupContainer<Self>
            where
                Self: Sized{
                let ($($t,)+) = args;
                BindGroupContainer {
                    marker: PhantomData,
                    group:
                    device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: layout.layout(),
                        entries: &([$($t::entries($t),)+].into_iter().flatten().collect::<Vec<_>>()),
                        label: Some("tuple group"),
                    })
                }
            }
        }
    }
}

impl_all!(impl_bind_data);

pub struct BindGroupContainer<T: BindData> {
    pub(self) marker: PhantomData<T>,
    pub(self) group: BindGroup,
}

impl<T: BindData> BindGroupContainer<T> {
    pub fn group(&self) -> &BindGroup {
        &self.group
    }
}
pub struct BindGroupLayoutContainer<T: BindData> {
    pub(self) marker: PhantomData<T>,
    pub(self) layout: Arc<wgpu::BindGroupLayout>,
}

impl<T: BindData> BindGroupLayoutContainer<T> {
    pub fn create_bind_group(&self,device:&Device,args:T::Args<'_>) ->BindGroupContainer<T> {
        T::create_group(device, self,args )
    }
}

impl<T: BindData> From<BindGroupLayoutContainer<T>> for Arc<BindGroupLayout> {
    fn from(val: BindGroupLayoutContainer<T>) -> Self {
        val.layout
    }
}

impl<T: BindData> From<Arc<BindGroupLayout>> for BindGroupLayoutContainer<T> {
    fn from(value: Arc<BindGroupLayout>) -> Self {
        Self {
            layout: value,
            marker: PhantomData,
        }
    }
}

impl<T: BindData> BindGroupLayoutContainer<T> {
    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }
}
