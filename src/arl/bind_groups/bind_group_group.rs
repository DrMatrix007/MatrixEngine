use std::sync::Arc;

use wgpu::RenderPass;

use crate::{
    arl::{
        bind_groups::{
            BindGroup, BindGroupIDable, BindGroupLayout, BindGroupable,
            bind_group_registry::{BindGroupGroupRegistry, BindGroupRegistry},
        },
        device_queue::DeviceQueue,
        id::IDWrapper,
    },
    impl_all,
};

pub trait BindGroupGroupLayouts {}

pub trait BindGroupableGroup {
    type Registry: BindGroupGroupRegistry<Input = Self::ID, Output = Self::BindGroups>;
    type ID: BindGroupIDable;
    type BindGroups: BindGroupGroupRef;

    fn create_registry(device_queue: &DeviceQueue) -> Self::Registry;
}

macro_rules! impl_bind_groupable_group_tuple {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupableGroup for ($($t,)+) {
            type Registry = ($(BindGroupRegistry::<$t>,)+);

            type ID = IDWrapper<($($t::BindGroupID,)+)>;

            type BindGroups = ($(Arc<BindGroup<$t>>,)+);
            fn create_registry(device_queue: &DeviceQueue) -> Self::Registry {
                ($(BindGroupRegistry::<$t>::new(device_queue.clone()),)+)
            }
        }

        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupLayouts for ($(BindGroupLayout<$t>,)+) {
        }
    }
}

impl BindGroupableGroup for () {
    type Registry = ();

    type ID = ();

    type BindGroups = ();

    fn create_registry(_: &DeviceQueue) -> Self::Registry {}
}

impl_all!(mini impl_bind_groupable_group_tuple);

pub trait BindGroupGroupRef {
    fn apply<'b>(&self, pass: &mut RenderPass<'b>);
}

macro_rules! impl_bind_group_group {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupRef for ($(Arc<BindGroup<$t>>,)+) {
            fn apply<'b>(&self, pass: &mut RenderPass<'b>) {
                let ($($t,)+) = self;
                let mut index = 0;

                $(
                    #[allow(unused_assignments)]
                    {
                        pass.set_bind_group(index, $t.raw(), &[]);
                        index += 1;
                    }
                )+

            }
        }
    }
}

impl_all!(mini impl_bind_group_group);

impl BindGroupGroupRef for () {
    fn apply<'a>(&self, _: &mut RenderPass<'a>) {}
}
