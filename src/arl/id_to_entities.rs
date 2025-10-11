use std::collections::HashMap;

use crate::{arl::id::IDable, engine::entity::Entity, utils::fast_vec::FastVec};

pub struct IDToEntitiyWithIndex<'a, ID: IDable> {
    pub id: &'a ID,
    pub entity: &'a Entity,
    pub index: usize,
    pub updated: &'a mut bool,
}

impl<'a, ID: IDable> IDToEntitiyWithIndex<'a, ID> {
    pub fn to_op(&self, target: ID) -> IDToEntitiyWithIndexMoveOperation<ID> {
        IDToEntitiyWithIndexMoveOperation {
            id: *self.id,
            index: self.index,
            move_to_id: target,
        }
    }
}

pub struct IDToEntitiyWithIndexMoveOperation<ID: IDable> {
    pub id: ID,
    pub index: usize,
    pub move_to_id: ID,
}

pub struct EntityRef {
    pub entity: Entity,
    pub updated: bool,
}

pub struct IdToEntitiesRegistry<ID: IDable> {
    entities: HashMap<ID, FastVec<(), EntityRef>>,
}

impl<ID: IDable> IdToEntitiesRegistry<ID> {
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
        }
    }

    pub fn iter_all_entities<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = IDToEntitiyWithIndex<'a, ID>> {
        self.entities.iter_mut().flat_map(|(id, entities)| {
            entities.iter_mut().map(move |(i, e)| IDToEntitiyWithIndex {
                id,
                entity: &e.entity,
                index: i,
                updated: &mut e.updated,
            })
        })
    }

    pub fn fix_entities(
        &mut self,
        ops: impl Iterator<Item = IDToEntitiyWithIndexMoveOperation<ID>>,
    ) {
        for IDToEntitiyWithIndexMoveOperation {
            id,
            index,
            move_to_id,
        } in ops
        {
            if let Some(v) = self.entities.get_mut(&id)
                && let Some(e) = v.remove(index)
            {
                self.entities.entry(move_to_id).or_default().push((), e);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ID, impl Iterator<Item = &Entity>)> {
        self.entities
            .iter()
            .map(|(id, v)| (id, v.iter().map(|(_, x)| &x.entity)))
    }

    pub fn add_entity(&mut self, id: ID, entity: Entity) {
        self.entities.entry(id).or_default().push(
            (),
            EntityRef {
                entity,
                updated: false,
            },
        );
    }
    pub fn iter_ids(&self) -> impl Iterator<Item = &ID> {
        self.entities.keys()
    }
}

impl<ID: IDable> Default for IdToEntitiesRegistry<ID> {
    fn default() -> Self {
        Self::new()
    }
}
