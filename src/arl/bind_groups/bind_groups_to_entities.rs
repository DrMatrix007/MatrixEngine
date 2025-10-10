use std::collections::HashMap;

use crate::{
    arl::bind_groups::bind_group_group::BindGroupGroup, engine::entity::Entity,
    utils::fast_vec::FastVec,
};

pub struct IDToEntitiyWithIndex<'a, Groups: BindGroupGroup> {
    pub id: &'a Groups::ID,
    pub entity: &'a Entity,
    pub index: usize,
}

impl<'a, Groups: BindGroupGroup> IDToEntitiyWithIndex<'a, Groups> {
    pub fn to_op(&self, target: Groups::ID) -> IDToEntitiyWithIndexMoveOperation<Groups> {
        IDToEntitiyWithIndexMoveOperation {
            id: *self.id,
            index: self.index,
            move_to_id: target,
        }
    }
}

pub struct IDToEntitiyWithIndexMoveOperation<Groups: BindGroupGroup> {
    pub id: Groups::ID,
    pub index: usize,
    pub move_to_id: Groups::ID,
}

pub struct BindGroupEntitiesRegistry<Groups: BindGroupGroup> {
    entities: HashMap<Groups::ID, FastVec<Entity>>,
}

impl<Groups: BindGroupGroup> BindGroupEntitiesRegistry<Groups> {
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
        }
    }

    pub fn iter_all_entities<'a>(
        &'a self,
    ) -> impl Iterator<Item = IDToEntitiyWithIndex<'a, Groups>> {
        self.entities.iter().flat_map(|(id, entities)| {
            entities.iter().map(move |(i, e)| IDToEntitiyWithIndex {
                id,
                entity: e,
                index: i,
            })
        })
    }

    pub fn fix_entities(
        &mut self,
        ops: impl Iterator<Item = IDToEntitiyWithIndexMoveOperation<Groups>>,
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
                self.entities.entry(move_to_id).or_default().push(e);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Groups::ID, impl Iterator<Item = &Entity>)> {
        self.entities
            .iter()
            .map(|(id, v)| (id, v.iter().map(|(_, x)| x)))
    }
}

impl<Groups: BindGroupGroup> Default for BindGroupEntitiesRegistry<Groups> {
    fn default() -> Self {
        Self::new()
    }
}
