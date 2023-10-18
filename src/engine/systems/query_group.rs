use crate::{engine::scenes::{entities::Entity, components::Component}, impl_all};
use std::collections::btree_map;
use super::query::{Query, QueryCleanup, QueryError, components::{ReadC, WriteC}};

macro_rules! impl_query_components {
    ($t1:tt $(,$t:tt)*) => {
        #[allow(non_snake_case)]
        impl<Args, $t1:Query<Args>,$($t:Query<Args>),*> Query<Args> for ($t1,$($t),*) {

            fn get(args:&mut Args) -> Result<Self,QueryError>{
                Ok(($t1::get(args)?,$($t::get(args)?),*))
            }

            fn available(args:&mut Args) -> bool {
                $t1::available(args) $(&& $t::available(args))*
            }
        }
        #[allow(non_snake_case)]
        impl<Args, $t1:Query<Args>,$($t:Query<Args>),*> QueryCleanup<Args> for ($t1,$($t),*) {

            fn cleanup(&mut self,args:&mut Args){
                let (ref mut $t1,$(ref mut $t),*)= self;
                $t1::cleanup($t1, args);$($t::cleanup($t,args));*
            }
        }
    };
}

impl_all!(impl_query_components);

pub trait ComponentRefIterable {
    type Item<'a>: 'a
    where
        Self: 'a;
    type IntoIter<'a>: Iterator<Item = (&'a Entity, Self::Item<'a>)>
    where
        Self: 'a;
    fn component_iter(&mut self) -> Self::IntoIter<'_>;
}

impl<C: Component> ComponentRefIterable for ReadC<C> {
    type Item<'a> = &'a C where Self:'a;
    type IntoIter<'a> = btree_map::Iter<'a, Entity, C> where Self:'a;

    fn component_iter(&mut self) -> Self::IntoIter<'_> {
        self.iter()
    }
}

impl<C: Component> ComponentRefIterable for WriteC<C> {
    type Item<'a> = &'a mut C where Self:'a;

    type IntoIter<'a> = btree_map::IterMut<'a,Entity,C> where Self:'a;

    fn component_iter(&mut self) -> Self::IntoIter<'_> {
        self.iter_mut()
    }
}

pub struct Iter<'a, Comps: CompsIntoIter>(Comps::IntoIter<'a>)
where
    Self: 'a;

pub trait CompsIntoIter {
    type IntoIter<'a>
    where
        Self: 'a;
}

macro_rules! impl_components {
    ($first:ident $(,$t:ident)*) => {
        #[allow(unused_parens)]
        impl<$first:ComponentRefIterable,$($t:ComponentRefIterable),*> ComponentRefIterable for  ($first,$($t),*) {
            type Item<'a> = ($first::Item<'a>,$($t::Item<'a>),*) where Self:'a;

            type IntoIter<'a> = Iter<'a,($first,$($t),*)>  where Self:'a;

            #[allow(non_snake_case)]
            fn component_iter(&mut self) -> <Self as ComponentRefIterable>::IntoIter<'_> {
                let ($first,$($t),*) = self;
                Iter((($first.component_iter(),$($t.component_iter()),*)))
            }
        }
        impl<$first:ComponentRefIterable,$($t:ComponentRefIterable),*> CompsIntoIter for ($first,$($t),*) {
            type IntoIter<'a> = ($first::IntoIter<'a>,$($t::IntoIter<'a>),*) where $first:'a,$($t:'a),*;
        }
        impl<'a, $first:ComponentRefIterable,$($t:ComponentRefIterable),*> Iterator for Iter<'a,($first,$($t),*)> where $first:'a,$($t:'a),* {
            type Item =(&'a Entity,($first::Item<'a>,$($t::Item<'a>),*))  ;

            #[allow(non_snake_case)]
            fn next(&mut self) -> Option<Self::Item> {
                let (first,$($t),*) = &mut self.0;
                let val = first.next()?;

                $(
                    let $t = find_by_entity::<$t>(val.0,$t)?;
                )*
                Some((val.0,(val.1,$($t),*)))
            }

        }
    };
}

impl_all!(impl_components);

fn find_by_entity<'a, T: ComponentRefIterable>(
    e: &Entity,
    iter: &mut T::IntoIter<'a>,
) -> Option<T::Item<'a>> {
    let mut current = iter.next()?;
    loop {
        match current.0.cmp(e) {
            std::cmp::Ordering::Less => {
                current = iter.next()?;
            }
            std::cmp::Ordering::Greater => {
                return None;
            }
            std::cmp::Ordering::Equal => {
                return Some(current.1);
            }
        }
    }
}
