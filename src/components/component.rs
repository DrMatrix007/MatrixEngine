use std::{
    any::{Any, TypeId},
    collections::{btree_map, BTreeMap, HashMap},
};

use crate::{
    impl_all,
    par::storage::{ReadStorageGuard, Storage, WriteStorageGuard},
    scenes::entity::Entity,
};

pub trait Component:'static {}

// type ComponentCollection<T> = BTreeMap<Entity, T>;
pub struct ComponentMap<T>(BTreeMap<Entity, T>);

impl<T> ComponentMap<T> {
    pub fn iter(&self) -> btree_map::Iter<'_, Entity, T> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> btree_map::IterMut<'_, Entity, T> {
        self.0.iter_mut()
    }
}

impl<T> Default for ComponentMap<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

type ParComponents<T> = Storage<ComponentMap<T>>;

#[derive(Default)]
pub struct ComponentRegistry {
    comps: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentRegistry {
    pub fn try_get_map<T: Component + 'static>(
        &mut self,
    ) -> Option<ReadStorageGuard<ComponentMap<T>>> {
        self.comps
            .get(&TypeId::of::<T>())
            .map(|x| {
                x.downcast_ref::<ParComponents<T>>()
                    .expect("the type of this value should be Components<T>")
            })
            .and_then(|x| x.try_read())
    }
    pub fn try_get_map_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<WriteStorageGuard<ComponentMap<T>>> {
        self.comps
            .get(&TypeId::of::<T>())
            .map(|x| {
                x.downcast_ref::<ParComponents<T>>()
                    .expect("the type of this value should be Components<T>")
            })
            .and_then(|x: &Storage<ComponentMap<T>>| x.try_write())
    }
}
trait ComponentRefIterable {
    type Item<'a>: 'a
    where
        Self: 'a;
    type IntoIter<'a>: Iterator<Item = (&'a Entity, Self::Item<'a>)>
    where
        Self: 'a;
    fn component_iter(&mut self) -> Self::IntoIter<'_>;
}

impl<T: Component> ComponentRefIterable for ReadStorageGuard<ComponentMap<T>> {
    type Item<'a> = &'a T where Self:'a;
    type IntoIter<'a> = btree_map::Iter<'a, Entity, T> where Self:'a;

    fn component_iter(&mut self) -> Self::IntoIter<'_> {
        self.iter()
    }
}

impl<T: Component> ComponentRefIterable for WriteStorageGuard<ComponentMap<T>> {
    type Item<'a> = &'a mut T where Self:'a;

    type IntoIter<'a> = btree_map::IterMut<'a,Entity,T> where Self:'a;

    fn component_iter(&mut self) -> Self::IntoIter<'_> {
        self.iter_mut()
    }
}

struct Iter<'a, Comps: CompsIntoIter>(Comps::IntoIter<'a>)
where
    Self: 'a;

trait CompsIntoIter {
    type IntoIter<'a>
    where
        Self: 'a;
}

macro_rules! impl_components {
    ($first:ident,$($t:ident),*) => {
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
impl_all!(impl_components);
// impl_components!(A, B, C);
// impl_components!(A);

#[test]
fn test_registry() {
    struct A;
    struct B;
    impl Component for A {}
    impl Component for B {}

    let mut _reg = ComponentRegistry::default();
}
