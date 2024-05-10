use std::{
    any::{Any, TypeId}, collections::HashMap, hash::Hash, sync::{Arc, PoisonError, RwLock, RwLockWriteGuard}
};

use crate::impl_all;

use super::{component::Component, entity::Entity};

pub trait Archtype: 'static {}

#[derive(Debug)]
pub struct ArchtypeCollection<A: Archtype> {
    data: Vec<(Entity, A)>,
}

impl<A: Archtype> ArchtypeCollection<A> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ (Entity, A)> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut (Entity, A)> {
        self.data.iter_mut()
    }
    pub fn push(&mut self, a: A) {
        self.data.push((Entity::new(), a));
    }
}

impl<A: Archtype> Default for ArchtypeCollection<A> {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_archtype{
    ($($t:ident),+) => {
        impl<$($t:Component,)+> Archtype for ($($t,)+) {
        }
    };
}

impl_all!(impl_archtype);

pub struct Archtypes {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Archtypes {

    pub fn new() -> Self {
        Archtypes {
            map :HashMap::new()
        }
    }
    pub fn get_or_insert<A: Archtype>(&mut self) -> &Arc<RwLock<ArchtypeCollection<A>>> {
        unsafe {
            self.map
                .entry(TypeId::of::<A>())
                .or_insert_with(|| Box::new(Arc::new(RwLock::new(ArchtypeCollection::<A>::new()))))
                .downcast_ref_unchecked::<Arc<RwLock<ArchtypeCollection<A>>>>()
        }
    }
    pub fn get<A: Archtype>(&self) -> Option<&Arc<RwLock<ArchtypeCollection<A>>>> {
        self.map
            .get(&TypeId::of::<A>())
            .map(|x| unsafe { x.downcast_ref_unchecked() })
    }

    pub fn insert<A: Archtype>(
        &mut self,
        a: A,
    ) -> Result<(), PoisonError<RwLockWriteGuard<ArchtypeCollection<A>>>> {
        self.get_or_insert::<A>().write()?.push(a);
        Ok(())
    }
}

impl Default for Archtypes {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::Archtypes;

    #[derive(Debug)]
    struct A;
    #[derive(Debug)]
    struct B;

    #[test]
    fn test_archtypes() {
        let mut reg = Archtypes::new();

        reg.insert((A,B)).unwrap();

        println!("{:?}",reg.get::<(A,B)>().iter().next().unwrap());

    }

}
