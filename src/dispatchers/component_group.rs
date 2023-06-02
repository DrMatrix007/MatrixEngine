use std::collections::btree_map;

use rayon::prelude::ParallelIterator;

use crate::{components::component::Component, entity::Entity, impl_all};

use super::dispatcher::{
    components::{ReadComponents, WriteComponents},
    DispatchedData,
};

pub trait GroupableComponents: DispatchedData + 'static {
    type Item<'a>;

    type Iter<'a>;

    fn iter(&mut self) -> Iter<'_, Self>
    where
        Self: Sized;

    // fn par_iter(&mut self)
    // where
    //     for<'a> Self::Item<'a>: Send;
}

pub trait SingleCollection: DispatchedData + 'static {
    type Item<'a>
    where
        Self: 'a;
    type OwnedItem;

    type Iter<'a>: Iterator<Item = (&'a Entity, Self::Item<'a>)>;

    fn iter(&mut self) -> Self::Iter<'_>;
}
pub trait ParSingleComponent: SingleCollection {
    type ParIter<'a>: ParallelIterator<Item = (&'a Entity, Self::Item<'a>)>;
}

impl<T: Component + 'static + Sync> SingleCollection for WriteComponents<T> {
    type Item<'a> = &'a mut T;
    type OwnedItem = T;

    fn iter(&mut self) -> Self::Iter<'_> {
        self.get().iter_mut()
    }

    type Iter<'a> = btree_map::IterMut<'a, Entity, Self::OwnedItem>;
}
impl<T: Component + Send + 'static> ParSingleComponent for WriteComponents<T>
where
    WriteComponents<T>: for<'a> SingleCollection<OwnedItem = T, Item<'a> = &'a mut T>,
{
    type ParIter<'a> = rayon::collections::btree_map::IterMut<'a, Entity, Self::OwnedItem>;
}

impl<T: Component + 'static + Sync> SingleCollection for ReadComponents<T> {
    type Item<'a> = &'a T;
    type OwnedItem = T;

    type Iter<'a> = btree_map::Iter<'a, Entity, Self::OwnedItem>;

    fn iter(&mut self) -> Self::Iter<'_> {
        self.get().iter()
    }
}

pub struct ComponentGroup<T: GroupableComponents> {
    data: T,
}

impl<T: GroupableComponents> DispatchedData for ComponentGroup<T> {
    type Data<'a> = T::Data<'a>;
    type Target = T::Target;

    fn dispatch(
        args: &mut super::dispatcher::DispatcherArgs<'_>,
    ) -> Result<Self::Target, super::dispatcher::DispatchError>
    where
        Self: Sized,
    {
        T::dispatch(args)
    }

    fn from_target_to_data(data: Self::Target) -> Self
    where
        Self: Sized,
    {
        ComponentGroup::<T>::new(T::from_target_to_data(data))
    }

    fn get(&mut self) -> Self::Data<'_> {
        self.data.get()
    }
}

impl<T: GroupableComponents> ComponentGroup<T> {
    pub fn iter(&mut self) -> Iter<'_, T> {
        self.data.iter()
    }
    fn new(data: T) -> ComponentGroup<T> {
        Self { data }
    }
}

pub struct Iter<'a, T: GroupableComponents>(T::Iter<'a>);

macro_rules! impl_group_tuple {
    ($first:ident $(,$t:ident)* $(,)?) => {
#[allow(non_snake_case)]

        impl<$first: SingleCollection+'static,$($t: SingleCollection+'static,)*> GroupableComponents for ($first,$($t,)*) {
            type Item<'a> = (&'a Entity, $first::Item<'a>,$($t::Item<'a>,)*);


            type Iter<'a> = ($first::Iter<'a>,$($t::Iter<'a>,)*);

            fn iter(&mut self) -> Iter<'_,Self> where Self:Sized {
                let ($first,$($t,)*) = self;
                Iter(($first.iter(),$($t.iter(),)*))
            }
        }
#[allow(non_snake_case)]
        impl<'a, $first: SingleCollection+'static,$($t: SingleCollection+'static,)*> Iterator for Iter<'a,($first,$($t,)*)> {
            type Item = <($first,$($t,)*) as GroupableComponents>::Item<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                let ($first,$($t,)*) = &mut self.0;
                let $first = $first.next()?;
                $(
                    let $t = $t.find(|(x,_)|{
                        *x==$first.0
                    }).map(|(_,x)|x);
                )*

                Some(($first.0,$first.1,$($t?,)*))

            }
        }
    };
}

impl_all!(impl_group_tuple);
