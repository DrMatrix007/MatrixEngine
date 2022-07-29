use super::entity::Entity;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait ComponentVec<T> {
    fn insert_component(&mut self, e: Entity, c: T);
    fn borrow_component(&self, e: Entity) -> Option<&T>;
    fn borrow_component_mut(&mut self, e: Entity) -> Option<&mut T>;
}
pub trait AnyToItem<Item> {
    fn as_ref(&self) -> Option<&Item>;
    fn as_mut(&mut self) -> Option<&mut Item>;
}
impl<T: 'static> AnyToItem<HashMap<Entity, T>> for dyn Any {
    fn as_ref(&self) -> Option<&HashMap<Entity, T>> {
        self.downcast_ref()
    }

    fn as_mut(&mut self) -> Option<&mut HashMap<Entity, T>> {
        self.downcast_mut()
    }
}
#[allow(dead_code)]
pub struct Registry {
    entity_count: u128,

    entities: Vec<Entity>,

    data: HashMap<TypeId, Box<dyn Any>>,
}
unsafe impl Send for Registry {
}

#[allow(dead_code)]
impl Registry {
    pub fn new() -> Self {
        Registry {
            entity_count: 0,
            entities: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let ans = Entity(self.entity_count);
        self.entity_count += 1;
        ans
    }

    pub fn borrow_component_mut<T: 'static>(&mut self, e: Entity) -> Option<&mut T> {
        let map = self.data.get_mut(&TypeId::of::<T>())?;
        let map = map.downcast_mut::<HashMap<Entity, T>>()?;
        map.borrow_component_mut(e)
    }
    pub fn borrow_component<T: 'static>(&self, e: Entity) -> Option<&T> {
        let map = self.data.get(&TypeId::of::<T>())?;
        let map = map.downcast_ref::<HashMap<Entity, T>>()?;
        map.borrow_component(e)


        
    }
    pub fn insert_component<T: 'static>(&mut self, e: Entity, c: T) {
        let ent = self.data.entry(TypeId::of::<T>());

        let any = ent.or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));
        if let Some(map) = any.downcast_mut::<HashMap<Entity, T>>() {
            map.insert_component(e, c);
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl<Comp> ComponentVec<Comp> for HashMap<Entity, Comp> {
    fn insert_component(&mut self, e: Entity, c: Comp) {
        self.insert(e, c);
    }

    fn borrow_component(&self, e: Entity) -> Option<&Comp> {
        self.get(&e)
    }

    fn borrow_component_mut(&mut self, e: Entity) -> Option<&mut Comp> {
        self.get_mut(&e)
    }
}
