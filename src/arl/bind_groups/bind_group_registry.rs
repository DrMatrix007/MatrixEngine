use std::{collections::HashMap, sync::Arc};

use paste::paste;

use crate::{
    arl::{
        bind_groups::{BindGroup, BindGroupLayout, BindGroupable},
        device_queue::DeviceQueue,
        id::IDWrapper,
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
            groups: Default::default(),
        }
    }

    pub fn get(&self, index: &Group::BindGroupID) -> Option<&Arc<BindGroup<Group>>> {
        self.groups.get(index)
    }

    pub fn get_or_create(&mut self, id: &Group::BindGroupID) -> &Arc<BindGroup<Group>> {
        self.groups.entry(*id).or_insert_with(|| {
            Arc::new(
                self.layout
                    .create(Group::new(id, &self.device_queue), &self.device_queue),
            )
        })
    }

    pub fn layout(&self) -> &BindGroupLayout<Group> {
        &self.layout
    }
}

pub trait BindGroupGroupRegistry: 'static {
    type Input;
    type Output;

    fn query_groups(&mut self, data: &Self::Input) -> Self::Output;

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]>;
}

macro_rules! impl_group_factory {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupRegistry for ($(BindGroupRegistry<$t>,)+) {
            type Input = IDWrapper<($($t::BindGroupID,)+)>;
            type Output = ($(Arc<BindGroup<$t>>,)+);

            fn query_groups(& mut self, data: &Self::Input) -> Self::Output {
                let ($($t,)+) = self;
                let ($(paste! { [<$t _id>] },)+) = data.0;
                ($(paste! { $t.get_or_create(&[<$t _id>]).clone() },)+)

            }

            fn layout_desc(& self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
                let ($($t,)+) = self;
                [$($t.layout().raw(),)+]
            }

        }
    }
}

impl_all!(mini impl_group_factory);

impl BindGroupGroupRegistry for () {
    type Input = ();

    type Output = ();

    fn query_groups(&mut self, _: &Self::Input) -> Self::Output {}

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
        []
    }
}
