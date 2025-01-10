use crate::models::entity::Entity;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GameWorld {
	pub map_id: String,
	pub entities: HashMap<String, Entity>,
}

impl GameWorld {
	pub fn new(map_id: String) -> Self {
		Self {
			map_id,
			entities: HashMap::new(),
		}
	}

	pub fn put_entity(&mut self, entity: Entity) -> String {
		let entity_id = entity.id.clone();
		self.entities.insert(entity_id.clone(), entity);
		entity_id
	}

	pub fn get_entity(&self, id: String) -> Option<&Entity> {
		self.entities.get(&id)
	}

	pub fn get_entity_mut(&mut self, id: String) -> Option<&mut Entity> {
		self.entities.get_mut(&id)
	}
}
