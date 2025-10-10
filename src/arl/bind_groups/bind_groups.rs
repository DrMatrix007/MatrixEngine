use std::{marker::PhantomData, num::NonZero};

use crate::{
    arl::{bind_groups::bind_group_registry::{BindGroupGroupRegistry, BindGroupRegistry}, device_queue::DeviceQueue, id::ID},
    impl_all,
};

pub struct BindGroupLayoutEntry {
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub count: Option<NonZero<u32>>,
}

pub trait BindGroupable {
    type BindGroupID: ID;

    fn new(id: &Self::BindGroupID) -> Self;

    fn label(&self) -> String;
    fn layout_label() -> String;

    fn get_layout_entries() -> &'static [BindGroupLayoutEntry];
    fn get_group_entries(&self) -> Vec<wgpu::BindingResource<'_>>;

    fn id(&self) -> Self::BindGroupID;
}

pub struct BindGroupLayout<Group: BindGroupable> {
    layout: wgpu::BindGroupLayout,
    marker: PhantomData<Group>,
}

impl<Group: BindGroupable> BindGroupLayout<Group> {
    pub fn new(device_queue: &DeviceQueue) -> Self {
        let layout =
            device_queue
                .device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(Group::layout_label().as_str()),
                    entries: Group::get_layout_entries()
                        .iter()
                        .enumerate()
                        .map(|(index, entry)| wgpu::BindGroupLayoutEntry {
                            visibility: entry.visibility,
                            count: entry.count,
                            ty: entry.ty,
                            binding: index as _,
                        })
                        .collect::<Vec<_>>()
                        .as_slice(),
                });

        Self {
            layout,
            marker: PhantomData,
        }
    }

    pub fn create(&self, data: Group, device_queue: &DeviceQueue) -> BindGroup<Group> {
        let bind = device_queue
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(data.label().as_str()),
                layout: &self.layout,
                entries: data
                    .get_group_entries()
                    .into_iter()
                    .enumerate()
                    .map(|(index, entry)| wgpu::BindGroupEntry {
                        binding: index as _,
                        resource: entry,
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            });

        BindGroup::new(data, bind)
    }
    
    pub fn raw(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}

pub struct BindGroup<Group: BindGroupable> {
    bind_group: wgpu::BindGroup,
    group: Group,
}

impl<Group: BindGroupable> BindGroup<Group> {
    fn new(group: Group, bind_group: wgpu::BindGroup) -> Self {
        Self { bind_group, group }
    }

    pub fn group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

pub trait BindGroupGroupLayouts {}

pub trait BindGroupGroup {
    type Registry: BindGroupGroupRegistry;

    fn create_registry(device_queue: &DeviceQueue) -> Self::Registry;
}

macro_rules! impl_bind_group_group_tuple {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroup for ($($t,)+) {
            type Registry = ($(BindGroupRegistry::<$t>,)+);


            fn create_registry(device_queue: &DeviceQueue) -> Self::Registry {
                ($(BindGroupRegistry::<$t>::new(device_queue.clone()),)+)
            }
        }

        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupLayouts for ($(BindGroupLayout<$t>,)+) {
        }
    }
}

impl_all!(mini impl_bind_group_group_tuple); // TODO: remove mini
