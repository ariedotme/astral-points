#[cfg(test)]
mod tests {
	use std::sync::OnceLock;

	use crate::models::components::collider::Collider;
	use crate::models::components::{health::Health, position::Position, storage::Storage};
	use crate::models::context::ServerContext;
	use crate::models::entity::Entity;
	use crate::utils::collision::check_collision;
	use crate::utils::vector2::Vector2;

	#[test]
	fn test_entity_creation() {
		let entity = Entity::new();
		assert!(entity.id.len() > 0);
		assert!(entity.components.is_empty());
		assert!(entity.parts.is_empty());
	}

	#[test]
	fn test_add_component() {
		let mut entity = Entity::new();
		let health = Health::new(100.0);
		assert!(entity.add_component(health.clone()));
		assert!(entity.has_component::<Health>());

		match entity.get_component::<Health>() {
			Some(component) => assert_eq!(component.current, 100.0),
			None => panic!("Health component not found in entity"),
		}
	}

	#[test]
	fn test_storage_component() {
		let mut entity = Entity::new();
		let storage = Storage {
			capacity: 10,
			max_weight: 50.0,
			stored_items: vec![],
		};
		assert!(entity.add_component(storage.clone()));
		assert!(entity.has_component::<Storage>());

		match entity.get_component::<Storage>() {
			Some(component) => {
				assert_eq!(component.capacity, 10);
				assert_eq!(component.max_weight, 50.0);
			}
			None => panic!("Storage component not found in entity"),
		}
	}

	#[test]
	fn test_dependant_component() {
		let mut entity = Entity::new();
		let collider = Collider::new(Vector2::new(0.0, 0.0), Vector2::zero());
		assert!(!entity.add_component(collider.clone()));
		let position = Position::new(Vector2::new(0.0, 0.0), 0.0, "map01".to_string());
		assert!(entity.add_component(position.clone()));
		assert!(entity.add_component(collider.clone()));
	}

	#[test]
	fn test_collision() {
		let mut entity1 = Entity::new();
		let mut entity2 = Entity::new();
		let position1 = Position::new(Vector2::new(0.0, 0.0), 0.0, "map01".to_string());
		let collider1 = Collider::new(Vector2::new(2.0, 2.0), Vector2::zero());
		let position2 = Position::new(Vector2::new(1.0, 1.0), 0.0, "map01".to_string());
		let collider2 = Collider::new(Vector2::new(2.0, 2.0), Vector2::zero());
		assert!(entity1.add_component(position1));
		assert!(entity1.add_component(collider1));
		assert!(entity2.add_component(position2));
		assert!(entity2.add_component(collider2));
		assert!(check_collision(&entity1, &entity2));
	}

	#[test]
	fn test_entity_query() {
		let mut entity1 = Entity::new();
		let binding = OnceLock::new();
		let context = binding.get_or_init(|| ServerContext::new());
		entity1.id = "test".to_string();
		context.add_entity(entity1);
		let entity = context.get_entity(&"test".to_string());
		assert!(entity.is_some());
		assert_eq!(entity.unwrap().id, "test");
	}
}
