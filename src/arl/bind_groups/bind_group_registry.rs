use std::collections::HashMap;

use paste::paste;

use crate::{
    arl::{
        bind_groups::{BindGroup, BindGroupLayout, BindGroupable},
        device_queue::DeviceQueue,
        id::{IDWrapper, IDable},
    },
    impl_all,
    utils::fast_vec::FastVec,
};

pub struct BindGroupRegistry<Group: BindGroupable> {
    groups: FastVec<Group::BindGroupID, BindGroup<Group>>,
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

    pub fn get(&self, index: usize) -> Option<&BindGroup<Group>> {
        self.groups.get(index)
    }

    pub fn get_or_create(&mut self, id: &Group::BindGroupID) -> usize {
        match self.groups.get_index_by_id(id) {
            Some((index, _)) => index,
            None => {
                self.groups
                    .push(*id, self.layout.create(Group::new(id), &self.device_queue))
                    .0
            }
        }
    }

    pub fn layout(&self) -> &BindGroupLayout<Group> {
        &self.layout
    }
}

pub trait BindGroupGroupRegistry: 'static {
    type CreationParams;
    type Input;
    type Output<'a>;

    fn try_create(&mut self, data: &Self::CreationParams) -> Self::Input;

    fn query_groups(&mut self, data: &Self::Input) -> Option<Self::Output<'_>>;

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]>;
}
trait UsizeType {
    type Usize: Into<usize>;
}
impl<T> UsizeType for T {
    type Usize = usize;
}

macro_rules! impl_group_factory {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupRegistry for ($(BindGroupRegistry<$t>,)+) {
            type CreationParams = IDWrapper<($($t::BindGroupID,)+)>;
            type Input = ($(<$t as UsizeType>::Usize,)+);
            type Output<'a> = ($(&'a BindGroup<$t>,)+);

            fn try_create(&mut self, data: &Self::CreationParams) -> Self::Input {
                let ($($t,)+) = self;
                let ($(paste! { [<$t _id>] },)+) = data.0;
                ($(paste! { $t.get_or_create(&[<$t _id>]) },)+)
            }

            fn query_groups<'a>(&'a mut self, data: &Self::Input) -> Option<Self::Output<'a>> {
                let ($($t,)+) = self;
                let ($(paste! { [<$t _id>] },)+) = *data;
                Some(($(paste! { $t.get([<$t _id>].into())? },)+))

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

    fn query_groups(&mut self, _: &Self::Input) -> Option<Self::Output<'_>> {
        Some(())
    }

    fn layout_desc(&self) -> impl AsRef<[&wgpu::BindGroupLayout]> {
        []
    }

    type CreationParams = ();

    fn try_create(&mut self, _: &Self::CreationParams) -> Self::Input {}
}
