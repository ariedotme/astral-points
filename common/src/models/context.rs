use crate::models::entity::Entity;
use crate::models::game_world::GameWorld;
use dashmap::DashMap;

pub struct ServerContext {
	pub entities_by_id: DashMap<String, Entity>,
	pub maps_by_id: DashMap<String, GameWorld>,
}

impl ServerContext {
	pub fn new() -> Self {
		Self {
			entities_by_id: DashMap::new(),
			maps_by_id: DashMap::new(),
		}
	}

	pub fn add_entity(&self, entity: Entity) {
		self.entities_by_id.insert(entity.id.clone(), entity);
	}

	pub fn get_entity(&self, entity_id: &String) -> Option<Entity> {
		self.entities_by_id
			.get(entity_id)
			.map(|entry| entry.value().clone())
	}

	pub fn add_map(&self, map: GameWorld) {
		self.maps_by_id.insert(map.map_id.clone(), map);
	}

	pub fn get_map(&self, map_id: &String) -> Option<GameWorld> {
		self.maps_by_id
			.get(map_id)
			.map(|entry| entry.value().clone())
	}
}
