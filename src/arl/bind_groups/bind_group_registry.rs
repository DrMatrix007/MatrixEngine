use std::collections::HashMap;

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
    groups: HashMap<Group::BindGroupID, BindGroup<Group>>,
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

    pub fn get(&self, id: &Group::BindGroupID) -> Option<&BindGroup<Group>> {
        self.groups.get(id)
    }

    pub fn get_or_create(&mut self, id: Group::BindGroupID) -> &BindGroup<Group> {
        self.groups
            .entry(id)
            .or_insert_with(|| self.layout.create(Group::new(&id), &self.device_queue))
    }

    pub fn layout(&self) -> &BindGroupLayout<Group> {
        &self.layout
    }
}

pub trait BindGroupGroupRegistry: 'static {
    type Input;
    type Output<'a>;

    fn query_groups(&mut self, data: &Self::Input) -> Self::Output<'_>;

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]>;
}

macro_rules! impl_group_factory {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupRegistry for ($(BindGroupRegistry<$t>,)+) {
            type Input = IDWrapper<($($t::BindGroupID,)+)>;
            type Output<'a> = ($(&'a BindGroup<$t>,)+);

            fn query_groups<'a>(&'a mut self, data: &Self::Input) -> Self::Output<'a> {
                let ($($t,)+) = self;
                let ($(paste! { [<$t _id>] },)+) = data.0;
                ($(paste! { $t.get_or_create([<$t _id>]) },)+)
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
    type Input = ();

    type Output<'a> = ();

    fn query_groups(&mut self, _: &Self::Input) -> Self::Output<'_> {}

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
        []
    }
}
