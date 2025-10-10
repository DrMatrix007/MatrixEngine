use crate::{
    arl::{
        bind_groups::{
            BindGroupLayout, BindGroupable,
            bind_group_registry::{BindGroupGroupRegistry, BindGroupRegistry},
        },
        device_queue::DeviceQueue,
        id::IDable,
    },
    impl_all,
};

pub trait BindGroupGroupLayouts {}

pub trait BindGroupGroup {
    type Registry: BindGroupGroupRegistry;
    type ID: IDable;

    fn create_registry(device_queue: &DeviceQueue) -> Self::Registry;
}

macro_rules! impl_bind_group_group_tuple {

    ($($t:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroup for ($($t,)+) {
            type Registry = ($(BindGroupRegistry::<$t>,)+);

            type ID = ($($t::BindGroupID,)+);

            fn create_registry(device_queue: &DeviceQueue) -> Self::Registry {
                ($(BindGroupRegistry::<$t>::new(device_queue.clone()),)+)
            }
        }

        #[allow(non_snake_case)]
        impl<$($t: BindGroupable + 'static),+> BindGroupGroupLayouts for ($(BindGroupLayout<$t>,)+) {
        }
    }
}

impl BindGroupGroup for () {
    type Registry = ();

    type ID = ();

    fn create_registry(_: &DeviceQueue) -> Self::Registry {}
}

impl_all!(mini impl_bind_group_group_tuple); // TODO: remove mini
