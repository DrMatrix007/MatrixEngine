use std::{marker::PhantomData, num::NonZero};

use crate::arl::{device_queue::DeviceQueue, id::IDable};

pub mod bind_group_group;
pub mod bind_group_registry;

pub struct BindGroupLayoutEntry {
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub count: Option<NonZero<u32>>,
}

pub trait BindGroupIDable: IDable {}

impl<T: IDable> BindGroupIDable for T {}

pub trait BindGroupable {
    type BindGroupID: BindGroupIDable;
    fn new(id: &Self::BindGroupID, device_queue: &DeviceQueue) -> Self;

    fn label(&self) -> String;
    fn layout_label() -> String;

    fn get_layout_entries() -> &'static [BindGroupLayoutEntry];
    fn get_group_entries(&self) -> impl AsRef<[wgpu::BindingResource<'_>]>;

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
                    .as_ref()
                    .iter()
                    .enumerate()
                    .map(|(index, entry)| wgpu::BindGroupEntry {
                        binding: index as _,
                        resource: entry.clone(),
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

    pub fn raw(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn group(&self) -> &Group {
        &self.group
    }
}
