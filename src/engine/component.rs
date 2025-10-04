use std::iter::Peekable;

use crate::{engine::{component, entity::Entity}, impl_all};

pub trait Component: Send + Sync + 'static {}

impl<T: Send + Sync + 'static> Component for T {}

#[derive(Default)]
struct ComponentCollection<T: Component> {
    components: Vec<Option<Box<T>>>,
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
    pub fn get_mut(&self, entity: &Entity) -> Option<&T> {
        self.components
            .get(entity.id())?
            .as_ref()
            .map(|x| x.as_ref())
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

                            $(if $name.is_none() { return None; })*
                            $(let $name = $name.unwrap();)*

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
                            if $name.peek().map_or(false, |&(e, _)| e > max_entity) {
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::entity::Entity;

    #[derive(Default, Debug, PartialEq)]
    struct TestComponent {
        value: i32,
    }

    impl TestComponent {
        fn new(value: i32) -> Self {
            Self { value }
        }
    }

    #[test]
    fn insert_and_retrieve_component() {
        let mut collection = ComponentCollection::<TestComponent>::default();
        let entity = Entity::from_id(0);

        assert!(collection.insert(&entity, TestComponent::new(10)).is_none());

        let comps: Vec<_> = collection.iter().collect();

        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].0.id(), 0);
        assert_eq!(comps[0].1.value, 10);
    }

    #[test]
    fn replace_existing_component_returns_old_value() {
        let mut collection = ComponentCollection::<TestComponent>::default();
        let entity = Entity::from_id(1);

        assert!(collection.insert(&entity, TestComponent::new(5)).is_none());
        let old = collection.insert(&entity, TestComponent::new(42));

        assert_eq!(old.unwrap().value, 5);

        let comps: Vec<_> = collection.iter().collect();
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].1.value, 42);
    }

    #[test]
    fn insert_out_of_order_entities_resizes_vec() {
        let mut collection = ComponentCollection::<TestComponent>::default();

        let entity = Entity::from_id(10);
        collection.insert(&entity, TestComponent::new(100));

        // Ensure intermediate indices are None
        assert_eq!(collection.components.len(), 11);
        assert!(collection.components[0..10].iter().all(|x| x.is_none()));

        let comps: Vec<_> = collection.iter().collect();
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].0.id(), 10);
        assert_eq!(comps[0].1.value, 100);
    }

    #[test]
    fn iter_mut_allows_modification() {
        let mut collection = ComponentCollection::<TestComponent>::default();

        for id in 0..3 {
            collection.insert(&Entity::from_id(id), TestComponent::new(id as i32));
        }

        for (_entity, comp) in collection.iter_mut() {
            comp.value += 10;
        }

        let values: Vec<_> = collection.iter().map(|(_, c)| c.value).collect();
        assert_eq!(values, vec![10, 11, 12]);
    }

    #[test]
    fn iter_skips_none_entries() {
        let mut collection = ComponentCollection::<TestComponent>::default();
        collection.insert(&Entity::from_id(0), TestComponent::new(1));
        collection.insert(&Entity::from_id(2), TestComponent::new(3));

        let ids: Vec<_> = collection.iter().map(|(e, _)| e.id()).collect();
        assert_eq!(ids, vec![0, 2]);
    }

    // -------------------------
    // Tests for the Join trait
    // -------------------------

    #[test]
    fn join_single_collection() {
        let mut collection = ComponentCollection::<TestComponent>::default();
        for i in 0..3 {
            collection.insert(&Entity::from_id(i), TestComponent::new(i as i32));
        }

        let mut joined = (&collection).join();

        let first = joined.next().unwrap();
        assert_eq!(first.0.id(), 0);
        assert_eq!(first.1.value, 0);

        let second = joined.next().unwrap();
        assert_eq!(second.0.id(), 1);
        assert_eq!(second.1.value, 1);

        let third = joined.next().unwrap();
        assert_eq!(third.0.id(), 2);
        assert_eq!(third.1.value, 2);

        assert!(joined.next().is_none());
    }

    #[test]
    fn join_two_collections_with_matching_entities() {
        let mut col_a = ComponentCollection::<TestComponent>::default();
        let mut col_b = ComponentCollection::<TestComponent>::default();

        col_a.insert(&Entity::from_id(0), TestComponent::new(1));
        col_a.insert(&Entity::from_id(2), TestComponent::new(2));

        col_b.insert(&Entity::from_id(0), TestComponent::new(10));
        col_b.insert(&Entity::from_id(2), TestComponent::new(20));

        let mut joined = (&col_a, &col_b).join();

        let first = joined.next().unwrap();
        assert_eq!(first.0.id(), 0);
        assert_eq!(first.1.0.value, 1);
        assert_eq!(first.1.1.value, 10);

        let second = joined.next().unwrap();
        assert_eq!(second.0.id(), 2);
        assert_eq!(second.1.0.value, 2);
        assert_eq!(second.1.1.value, 20);

        assert!(joined.next().is_none());
    }

    #[test]
    fn join_two_collections_with_non_matching_entities_skips_missing() {
        let mut col_a = ComponentCollection::<TestComponent>::default();
        let mut col_b = ComponentCollection::<TestComponent>::default();

        col_a.insert(&Entity::from_id(0), TestComponent::new(1));
        col_a.insert(&Entity::from_id(2), TestComponent::new(2));

        col_b.insert(&Entity::from_id(1), TestComponent::new(10));
        col_b.insert(&Entity::from_id(2), TestComponent::new(20));

        let mut joined = (&col_a, &col_b).join();

        let first = joined.next().unwrap();
        assert_eq!(first.0.id(), 2);
        assert_eq!(first.1.0.value, 2);
        assert_eq!(first.1.1.value, 20);

        assert!(joined.next().is_none());
    }

    #[test]
    fn join_three_collections() {
        let mut a = ComponentCollection::<TestComponent>::default();
        let mut b = ComponentCollection::<TestComponent>::default();
        let mut c = ComponentCollection::<TestComponent>::default();

        a.insert(&Entity::from_id(0), TestComponent::new(1));
        a.insert(&Entity::from_id(1), TestComponent::new(2));

        b.insert(&Entity::from_id(0), TestComponent::new(10));
        b.insert(&Entity::from_id(1), TestComponent::new(20));

        c.insert(&Entity::from_id(0), TestComponent::new(100));
        c.insert(&Entity::from_id(1), TestComponent::new(200));

        let mut joined = (&a, &b, &c).join();

        let first = joined.next().unwrap();
        assert_eq!(first.0.id(), 0);
        assert_eq!(first.1.0.value, 1);
        assert_eq!(first.1.1.value, 10);
        assert_eq!(first.1.2.value, 100);

        let second = joined.next().unwrap();
        assert_eq!(second.0.id(), 1);
        assert_eq!(second.1.0.value, 2);
        assert_eq!(second.1.1.value, 20);
        assert_eq!(second.1.2.value, 200);

        assert!(joined.next().is_none());
    }
}
