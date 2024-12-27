use crate::models::entity::{Entity, EntityId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameWorld {
    pub entities: HashMap<EntityId, Entity>,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn put_entity(&mut self, entity: Entity) -> EntityId {
        let entity_id = entity.id.clone();
        self.entities.insert(entity_id.clone(), entity);
        entity_id
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }
}
