use std::{collections::HashMap, sync::Arc};

use paste::paste;

use crate::{
    arl::{
        bind_groups::{BindGroup, BindGroupLayout, BindGroupable},
        device_queue::DeviceQueue,
    },
    impl_all,
};

pub struct BindGroupRegistry<Group: BindGroupable> {
    groups: HashMap<Group::BindGroupID, Arc<BindGroup<Group>>>,
    layout: BindGroupLayout<Group>,
    device_queue: DeviceQueue,
}

impl<Group: BindGroupable> BindGroupRegistry<Group> {
    pub fn new(device_queue: DeviceQueue) -> Self {
        let layout = BindGroupLayout::new(&device_queue);

        Self {
            device_queue,
            layout,
            groups: HashMap::new(),
        }
    }

    pub fn get(&self, id: &Group::BindGroupID) -> Option<&Arc<BindGroup<Group>>> {
        self.groups.get(id)
    }

    pub fn get_or_create(&mut self, id: Group::BindGroupID) -> &Arc<BindGroup<Group>> {
        self.groups.entry(id).or_insert_with(|| {
            let group = self.layout.create(Group::new(&id), &self.device_queue);
            Arc::new(group)
        })
    }

    pub fn layout(&self) -> &BindGroupLayout<Group> {
        &self.layout
    }
}

pub trait BindGroupGroupRegistry {
    type Input<'a>;
    type Output;

    fn query_groups<'a>(&mut self, data: Self::Input<'a>) -> Self::Output;

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]>;
}

macro_rules! impl_group_factory {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupRegistry for ($(BindGroupRegistry<$t>,)+) {
            type Input<'a> = ($(&'a $t::BindGroupID,)+);
            type Output = ($(Arc<BindGroup<$t>>,)+);

            fn query_groups<'a>(&mut self, data: Self::Input<'a>) -> Self::Output {
                let ($($t,)+) = self;
                let ($(paste! { [<$t _id>] },)+) = data;
                ($(paste! { $t.get_or_create(*[<$t _id>]).clone() },)+)
            }

            fn layout_desc(& self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
                let ($($t,)+) = self;
                [$($t.layout().raw(),)+]
            }

        }
    }
}

impl_all!(impl_group_factory);

impl BindGroupGroupRegistry for () {
    type Input<'a> = ();

    type Output = ();

    fn query_groups<'a>(&mut self, _: Self::Input<'a>) -> Self::Output {}

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
        []
    }
}
