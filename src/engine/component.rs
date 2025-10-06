use std::iter::Peekable;

use anymap::AnyMap;

use crate::{
    engine::entity::Entity,
    impl_all,
    lockable::{Lockable, LockableError, LockableReadGuard, LockableWriteGuard},
};

pub trait Component: Send + Sync + 'static {}

// impl<T: Send + Sync + 'static> Component for T {}

pub struct ComponentCollection<T: Component> {
    components: Vec<Option<Box<T>>>,
}

impl<T: Component> Default for ComponentCollection<T> {
    fn default() -> Self {
        Self {
            components: Default::default(),
        }
    }
}

impl<T: Component> ComponentCollection<T> {
    pub fn insert(&mut self, entity: &Entity, comp: T) -> Option<T> {
        let new_index = entity.id();

        if new_index >= self.components.len() {
            self.components.resize_with(new_index + 1, || None);
        }

        self.components[new_index]
            .replace(Box::new(comp))
            .map(|x| *x)
    }

    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.components
            .get(entity.id())?
            .as_ref()
            .map(|x| x.as_ref())
    }
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.components
            .get_mut(entity.id())?
            .as_mut()
            .map(|x| x.as_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.components
            .iter()
            .enumerate()
            .filter_map(|(id, x)| x.as_ref().map(|x| (Entity::from_id(id), x.as_ref())))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.components
            .iter_mut()
            .enumerate()
            .filter_map(|(id, x)| x.as_mut().map(|x| (Entity::from_id(id), x.as_mut())))
    }
}

pub trait Join<'a> {
    type Item;
    fn join(self) -> Peekable<impl Iterator<Item = (Entity, Self::Item)> + 'a>;
}

impl<'a, T: Component> Join<'a> for &'a ComponentCollection<T> {
    type Item = &'a T;

    fn join(self) -> Peekable<impl Iterator<Item = (Entity, Self::Item)> + 'a> {
        self.iter().peekable()
    }
}
impl<'a, T: Component> Join<'a> for &'a mut ComponentCollection<T> {
    type Item = &'a mut T;

    fn join(self) -> Peekable<impl Iterator<Item = (Entity, Self::Item)> + 'a> {
        self.iter_mut().peekable()
    }
}

macro_rules! impl_join {
    ($($name:ident),+) => {
        impl<'a, $($name: Join<'a>),+> Join<'a> for ($($name,)+) {
            type Item = ($($name::Item),+,);

            #[allow(unused_parens, non_snake_case)]
            fn join(self) -> Peekable<impl Iterator<Item = (Entity, Self::Item)> + 'a> {
                let ($($name,)*) = self;
                // create mutable peekable iterators
                let ($(mut $name),*) = ($($name.join().peekable()),*);

                std::iter::from_fn(move || {
                    loop {
                        let max_entity = {
                            $(let $name = $name.peek();)*

                            $(let $name = $name?;)*

                            {
                                let mut max = None;
                                $(
                                    let e = $name.0;
                                    max = Some(match max { Some(m) if m > e => m, _ => e });
                                )*
                                max.unwrap()
                            }
                        };

                        $(
                            while $name.peek().map_or(false, |&(e, _)| e < max_entity) {
                                $name.next();
                            }
                        )*
                        $(
                            if $name.peek().map_or(true, |&(e, _)| e > max_entity) {
                                continue;
                            }
                        )*

                        // the unwrap is because it was peeked!
                        let tuple = ($($name.next().unwrap().1,)*);
                        return Some((max_entity, tuple));
                    }
                }).peekable()
            }
        }
    };
}

// expand for tuple arities you want
impl_all!(impl_join);

#[derive(Debug)]
pub struct ComponentRegistry {
    components: AnyMap,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self {
            components: AnyMap::new(),
        }
    }
}

impl ComponentRegistry {
    pub fn read<T: Component>(
        &mut self,
    ) -> Result<LockableReadGuard<ComponentCollection<T>>, LockableError> {
        self.components
            .entry::<Lockable<ComponentCollection<T>>>()
            .or_insert_with(Default::default)
            .read()
    }

    pub fn write<T: Component>(
        &mut self,
    ) -> Result<LockableWriteGuard<ComponentCollection<T>>, LockableError> {
        self.components
            .entry::<Lockable<ComponentCollection<T>>>()
            .or_insert_with(Default::default)
            .write()
    }

    pub fn read_consume<T: Component>(
        &mut self,
        data: LockableReadGuard<ComponentCollection<T>>,
    ) -> Result<(), LockableError> {
        self.components
            .entry::<Lockable<ComponentCollection<T>>>()
            .or_insert_with(Default::default)
            .consume_read(data)
    }

    pub fn write_consume<T: Component>(
        &mut self,
        data: LockableWriteGuard<ComponentCollection<T>>,
    ) -> Result<(), LockableError> {
        self.components
            .entry::<Lockable<ComponentCollection<T>>>()
            .or_insert_with(Default::default)
            .consume_write(data)
    }
}
