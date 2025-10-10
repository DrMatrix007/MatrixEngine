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
    type Registry: for<'a> BindGroupGroupRegistry<Input = Self::ID, Output<'a> = Self::BindGroups<'a>>;
    type ID: BindGroupIDable;
    type BindGroups<'a>: BindGroupGroupRef<'a>;

    fn create_registry(device_queue: &DeviceQueue) -> Self::Registry;
}
macro_rules! impl_bind_groupable_group_tuple {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupableGroup for ($($t,)+) {
            type Registry = ($(BindGroupRegistry::<$t>,)+);

            type ID = IDWrapper<($($t::BindGroupID,)+)>;

            type BindGroups<'a> = ($(&'a BindGroup<$t>,)+);
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

    type BindGroups<'a> = ();

    fn create_registry(_: &DeviceQueue) -> Self::Registry {}
}

impl_all!(impl_bind_groupable_group_tuple);

pub trait BindGroupGroupRef<'a> {
    fn apply<'b>(&self, pass: &mut RenderPass<'b>);
}

macro_rules! impl_bind_group_group {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<'a, $($t: BindGroupable + 'static),+> BindGroupGroupRef<'a> for ($(&'a BindGroup<$t>,)+) {
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

impl_all!(impl_bind_group_group);

impl BindGroupGroupRef<'_> for () {
    fn apply<'a>(&self, _: &mut RenderPass<'a>) {}
}
