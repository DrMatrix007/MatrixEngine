use wgpu::{BindGroupLayout, Device};

use crate::impl_all;

use super::bind_groups::{BindData, BindGroupContainer, BindGroupLayoutContainer};

pub trait BindGroupCluster {
    type Args<'a>;
    type Groups: BindGroupLayoutContainerCluster;
    fn apply_to_pipeline<'a>(p: &mut wgpu::RenderPass<'a>, args: Self::Args<'a>);

    fn create_bind_group_layouts(label: &str, device: &Device) -> Self::Groups;
}

macro_rules! impl_cluster_group {
    ($($t:ident),+) => {
        #[allow(unused_parens)]
        impl<$($t:BindData+'static),+> BindGroupCluster for ($($t,)+) {
            type Args<'a> = ($(&'a BindGroupContainer<$t>),*);

            type Groups = ($(BindGroupLayoutContainer<$t>),*);

            #[allow(non_snake_case,unused_assignments)]
            fn apply_to_pipeline<'a>(p: &mut wgpu::RenderPass<'a>, ($($t),+): Self::Args<'a>) {
                let mut i = 0;
                {$(p.set_bind_group(i,$t.group(),&[]);i+=1;)*}
            }
            fn create_bind_group_layouts(label:&str,device:&Device) -> Self::Groups {
                Self::Groups::create_layouts(label,device)
            }

        }
    }
}

impl_all!(impl_cluster_group);

pub trait BindGroupLayoutContainerCluster {
    fn create_layouts(label: &str, device: &Device) -> Self
    where
        Self: Sized;
    fn iter_groups<'a: 'b, 'b>(&'a self) -> Box<dyn Iterator<Item = &'b BindGroupLayout> + 'b>;
}

macro_rules! impl_cluster_container_group {
    ($($t:ident),+) => {

#[allow(non_snake_case)]
#[allow(unused_parens)]
        impl<$($t:BindData+'static),+> BindGroupLayoutContainerCluster for ($(BindGroupLayoutContainer<$t>),+) {
            fn create_layouts(label:&str,device:&Device) -> Self where Self:Sized {
                ($($t::create_layout(label,device)),+)
            }
            fn iter_groups<'a:'b, 'b>(&'a self) -> Box<dyn Iterator<Item=&'b BindGroupLayout>+'b> {
                let ($($t),+) = self;
                Box::new([$($t.layout()),+].into_iter())
            }
        }
    }
}

impl<T: BindData + 'static> BindGroupLayoutContainerCluster for (BindGroupLayoutContainer<T>,) {
    fn create_layouts(label: &str, device: &Device) -> Self
    where
        Self: Sized,
    {
        (T::create_layout(label, device),)
    }

    fn iter_groups<'a: 'b, 'b>(&'a self) -> Box<dyn Iterator<Item = &'b BindGroupLayout> + 'b> {
        Box::new(std::iter::once(self.0.layout()))
    }
}

impl_all!(impl_cluster_container_group);
