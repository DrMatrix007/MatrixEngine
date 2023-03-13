use std::{
    any::{Any, TypeId},
    collections::{HashMap, hash_map},
    sync::Arc, fmt::Debug, vec,
};

use crate::entity::{Entity, self};

pub trait Component: Send + Sync {
    fn to_ref(&self) -> &dyn Component where Self:Sized {
        self
    }    
    fn to_ref_mut(&mut self) -> &mut dyn Component where Self:Sized {
        self
    }
}

pub trait IComponentCollection: Send+Sync+Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn iter<'a>(&'a self) ->  vec::IntoIter<(&'a Entity,&'a dyn Component)>;
    fn iter_mut<'a>(&'a mut self) -> vec::IntoIter<(&'a Entity,&'a mut dyn Component)>;

    fn get(&self,e:&Entity) -> Option<&dyn Component>; 
    fn get_mut(&mut self,e:&Entity) -> Option<&mut dyn Component>; 

}

#[derive(Debug)]
pub struct InsertError;

#[derive(Debug)]
pub struct ComponentCollection<T> {
    components: HashMap<Entity, Box<T>>,
}

impl<T> Default for ComponentCollection<T> {
    fn default() -> Self {
        Self { components: Default::default() }
    }
}

impl<T> ComponentCollection<T> {
    pub fn insert(&mut self, e: Entity, t: T) -> Option<Box<T>> {
        self.components.insert(e, Box::new(t))
    }
    pub fn remove(&mut self, e: Entity) -> Option<Box<T>> {
        self.components.remove(&e)
    }

    pub fn get(&self, e: &Entity) -> Option<&Box<T>> {
        self.components.get(e)
    }
    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut Box<T>> {
        self.components.get_mut(e)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &Box<T>)> {
        self.components.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Entity, &mut Box<T>)> {
        self.components.iter_mut()
    }
}


impl<T:Component+Debug+'static>  IComponentCollection for ComponentCollection<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn iter<'a>(&'a self) ->  vec::IntoIter<(&'a Entity,&'a dyn Component)>
    {
        self.components.iter().map(|(x,y)|(x, Box::as_ref(y).to_ref())).collect::<Vec<(&Entity,&dyn Component)>>().into_iter()
    }

    fn iter_mut<'a>(&'a mut self) -> vec::IntoIter<(&'a Entity,&'a mut dyn Component)> {
        self.components.iter_mut().map(|(x,y)|(x, Box::as_mut(y).to_ref_mut())).collect::<Vec<(&Entity,&mut dyn Component)>>().into_iter()

    }

    fn get(&self,e:&Entity) -> Option<&dyn Component> {
        Some(Component::to_ref(Box::as_ref(self.components.get(e)?)))
    }

    fn get_mut(&mut self,e:&Entity) -> Option<&mut dyn Component> {
        Some(Component::to_ref_mut(Box::as_mut(self.components.get_mut(e)?)))
        
    }

}
#[derive(Default)]
pub struct ComponentRegistryBuilder {
    components: HashMap<TypeId, Box<dyn IComponentCollection>>,
}

impl ComponentRegistryBuilder {
    pub fn build(self) -> ComponentRegistry {
        ComponentRegistry {
            components: self
                .components
                .into_iter()
                .map(|(x, y)| (x, ComponentCollectionState::Available(y)))
                .collect(),
        }
    }
    pub fn insert<T: Component +Debug+ 'static>(&mut self, e: Entity, t: T) -> Result<(),InsertError> {
        let b = self.components.get_mut(&TypeId::of::<T>());
        let Some(b) = b else {
            self.components.insert(TypeId::of::<T>(), Box::<ComponentCollection<T>>::default());    

            return self.insert::<T>(e, t);
        };
        let Some(v) = b.as_any_mut().downcast_mut::<ComponentCollection<T>>() else {
            return Err(InsertError);
        };
        v.insert(e, t);

        Ok(())
    }
}

pub enum ComponentCollectionState {
    Available(Box<dyn IComponentCollection>),
    ReadOnly(Arc<Box<dyn IComponentCollection>>,i32),
    Taken,
}

pub struct ComponentRegistry {
    components: HashMap<TypeId, ComponentCollectionState>,
}

impl ComponentRegistry {
    pub fn read_vec(&self,id:&TypeId) -> Option<&ComponentCollectionState> {
        self.components.get(id)
    } 
    pub fn read_vec_mut(&mut self,id:&TypeId) -> Option<&mut ComponentCollectionState> {
        self.components.get_mut(id)
    } 
    pub fn pop_vec(&mut self,id:&TypeId) -> Option<ComponentCollectionState> {
        self.components.remove(id)
    }

    pub(crate) fn insert_vec(&mut self, id: TypeId, vec: ComponentCollectionState)  {
        self.components.insert(id, vec);
    } 
}
