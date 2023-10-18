use std::{any::TypeId, collections::HashMap, sync::Arc};

use wgpu::{BindGroupLayout};

use crate::renderer::matrix_renderer::renderer_system::DeviceQueue;

use super::bind_groups::{BindData, BindGroupContainer, BindGroupLayoutContainer};

pub struct BindGroupLayoutManager {
    bind_groups: HashMap<TypeId, Arc<BindGroupLayout>>,
    device: DeviceQueue,
}
impl BindGroupLayoutManager {
    pub fn new(device: DeviceQueue) -> Self {
        Self {
            bind_groups: Default::default(),
            device,
        }
    }

    pub fn get_bind_group_layout<T: BindData + 'static>(&mut self) -> BindGroupLayoutContainer<T> {
        BindGroupLayoutContainer::from(
            self.bind_groups
                .entry(TypeId::of::<T>())
                .or_insert_with(|| {
                    T::create_layout("auto generated bind group layout", &self.device.device())
                        .into()
                })
                .clone(),
        )
    }
    pub fn create_group<T: BindData + 'static>(
        &mut self,
        args: T::Args<'_>,
    ) -> BindGroupContainer<T> {
        self.get_bind_group_layout::<T>()
            .create_bind_group(&self.device.device(), args)
    }
}
