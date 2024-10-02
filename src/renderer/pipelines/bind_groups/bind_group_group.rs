use wgpu::{BindGroupLayout, RenderPass};

use crate::{impl_all, renderer::pipelines::device_queue::DeviceQueue};

use super::bind_group::{MatrixBindGroup, MatrixBindGroupLayout, MatrixBindGroupable};
pub trait MatrixBindGroupableGroupable {
    type Layouts;

    fn create_layouts(device_queue: &DeviceQueue) -> Self::Layouts;

    fn as_slice(data: &Self::Layouts) -> Vec<&BindGroupLayout>;

    type Groups<'a>;

    fn setup_pass(pass: &mut RenderPass, groups: Self::Groups<'_>);
}

macro_rules! impl_group_group {
    ($($t:tt)*) => {
        impl<$($t:MatrixBindGroupable+'static,)*> MatrixBindGroupableGroupable for ($($t,)*) {
            type Layouts = ($(MatrixBindGroupLayout<$t>,)*);

            fn create_layouts(device_queue: &DeviceQueue) -> Self::Layouts {
                ($(MatrixBindGroupLayout::<$t>::new(device_queue),)*)
            }

            fn as_slice(data:&Self::Layouts) -> Vec<&BindGroupLayout> {
                #[allow(non_snake_case)]
                let ($($t,)*) = data;
                vec![$($t.layout(),)*]
            }

            type Groups<'a> = ($(&'a MatrixBindGroup<$t>,)*);

            #[allow(non_snake_case)]
            fn setup_pass(pass: &mut RenderPass, groups: Self::Groups<'_>) {
                let mut i = 0;
                let ($($t,)*) = groups;
                $(pass.set_bind_group({i+=1;i-1},$t.group(),&[]);)*
            }

        }
    }
}
impl MatrixBindGroupableGroupable for () {
    type Layouts = ();

    fn create_layouts(_device_queue: &DeviceQueue) -> Self::Layouts {}

    fn as_slice(_data: &Self::Layouts) -> Vec<&BindGroupLayout> {
        vec![]
    }

    type Groups<'a> = ();

    fn setup_pass<'a>(pass: &mut RenderPass, groups: Self::Groups<'a>) {}
}

impl_all!(impl_group_group);
