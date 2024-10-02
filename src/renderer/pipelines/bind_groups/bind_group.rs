use std::marker::PhantomData;

use wgpu::{BindGroup, BindGroupLayout, BindGroupLayoutDescriptor};

use crate::{impl_all, renderer::pipelines::device_queue::DeviceQueue};

use super::bind::MatrixBindable;

pub struct MatrixBindGroupLayout<G: MatrixBindGroupable> {
    layout: BindGroupLayout,
    marker: PhantomData<G>,
}

impl<G: MatrixBindGroupable> MatrixBindGroupLayout<G> {
    pub fn new(device_queue: &DeviceQueue) -> Self {
        Self {
            layout: G::create_group_layout(device_queue),
            marker: PhantomData,
        }
    }
    pub fn create_group(&self, device_queue: &DeviceQueue, data: &G) -> MatrixBindGroup<G> {
        MatrixBindGroup::new(data.create_group(device_queue, self))
    }

    pub fn layout(&self) -> &BindGroupLayout {
        &self.layout
    }
}

pub trait MatrixBindGroupable {
    fn layout_name() -> &'static str {
        "matrix bind group default name"
    }
    fn group_name() -> &'static str {
        "matrix group default name"
    }

    fn create_group_layout(device_queue: &DeviceQueue) -> BindGroupLayout;

    fn create_group(
        &self,
        device_queue: &DeviceQueue,
        layout: &MatrixBindGroupLayout<Self>,
    ) -> BindGroup
    where
        Self: Sized;
}

macro_rules! impl_group {
    ($($t:tt)*) => {
        impl<$($t:MatrixBindable,)*> MatrixBindGroupable for ($($t,)*) {
            #[allow(non_snake_case)]
            fn create_group_layout(device_queue:&DeviceQueue) -> BindGroupLayout {
                let mut i = 0;
                $(let $t =$t::bind_layout_entry({i+=1;i-1});)*
                device_queue
                    .device()
                    .create_bind_group_layout(&BindGroupLayoutDescriptor {
                        label: Some(Self::layout_name()),
                        entries: &[$($t,)*],
                    })
             }
            fn create_group(&self,device_queue:&DeviceQueue, layout: &MatrixBindGroupLayout<Self>) -> BindGroup where Self:Sized {
                #[allow(non_snake_case)]
                let ($($t,)*) = self;
                device_queue
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some(&Self::group_name()),
                        layout: layout.layout(),
                        entries: &[$($t.bind_entry(),)*],
                    })
            }
        }
    };
}

impl_all!(impl_group);

pub struct MatrixBindGroup<G: MatrixBindGroupable> {
    group: BindGroup,
    marker: PhantomData<G>,
}

impl<G: MatrixBindGroupable> MatrixBindGroup<G> {
    pub fn new(group: BindGroup) -> Self {
        Self {
            group,
            marker: PhantomData,
        }
    }

    pub fn group(&self) -> &BindGroup {
        &self.group
    }
}
