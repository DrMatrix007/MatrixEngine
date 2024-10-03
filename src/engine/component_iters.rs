use std::marker::PhantomData;

use paste::paste;

use crate::impl_all;

use super::{
    components::{self, Component, Iter, IterMut},
    entity::Entity,
    query::{ReadC, WriteC},
};

pub trait ComponentIterable<'a>: Iterator<Item = (&'a Entity, Self::C)> {
    type C: 'a;
}

impl<'a, C: Component> ComponentIterable<'a> for components::Iter<'a, C> {
    type C = &'a C;
}

impl<'a, C: Component> ComponentIterable<'a> for components::IterMut<'a, C> {
    type C = &'a mut C;
}

pub trait IntoWrapper<T> {
    fn into_wrapper(self) -> T;
}
// type A;
macro_rules! impl_group {
    ($($t:tt)*) => {
        paste!{

        pub struct [<ComponentIter $($t)*>]<'a,$($t:ComponentIterable<'a>,)*> {
            data: ($($t,)*),
            marker: PhantomData<&'a ()>,
        }

        #[allow(non_snake_case)]
        impl<'a, $($t:ComponentIterable<'a>,)*> Iterator for [<ComponentIter $($t)*>]<'a, $($t,)*> {
            type Item = (&'a Entity, ($($t::C,)*));

            fn next(&mut self) -> Option<Self::Item> {
                // Tuples of component data iterators
                let ($(ref mut $t,)*) = self.data;

                // Advance each iterator and check that all entities match
                let mut current_entity = None;

                $(
                    let $t = loop {
                        if let Some((entity, component)) = $t.next() {
                            if let Some(e) = current_entity {
                                if e < entity {
                                    continue;
                                }else if e==entity {
                                    break component;
                                }else {
                                    return None;
                                }
                            } else {
                                current_entity = Some(entity);
                                break component;
                            }
                        }else {
                            return None;
                        }
                    };
                )*

                // Construct the result tuple from all the components
                Some((
                    current_entity.unwrap(),
                    ($($t,)*)
                ))
            }
        }

        pub struct [<IteratorWrapper $($t)*>]<'a, $($t:ComponentIterable<'a>,)*> {
            data: ($($t,)*),
            marker: PhantomData<&'a ()>
        }
        #[allow(non_snake_case)]
        impl<'a,$($t:ComponentIterable<'a>,)*> IntoWrapper<[<ComponentIter $($t)*>]<'a,$($t,)*>> for ($($t,)*) {
            fn into_wrapper(self) -> [<ComponentIter $($t)*>]<'a,$($t,)*> {
                let ($($t,)*) = self;
                [<ComponentIter $($t)*>]::<'a,$($t,)*> {
                    data: ($($t,)*),
                    marker: PhantomData,
                }
            }
        }

        // impl<'a,$($t:ComponentIterable<'a>+'static,)*> ComponentIterableGroup<'a> for  [<IteratorWrapper $($t)*>]<'a, $($t,)*> {
        //     type C = ($($t::C,)*);

        //     #[allow(non_snake_case)]
        //     fn iter_all(self) -> impl Iterator<Item = (&'a Entity, Self::C)> {
        //         let ($($t,)*) = self.data;
        //         [<ComponentIter $($t)*>]::<'a,$($t,)*> {
        //             data: ($($t,)*)
        //         }
        //     }

        // }
        }
    };
}

impl_all!(impl_group);
