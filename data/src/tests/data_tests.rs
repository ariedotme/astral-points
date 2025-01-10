#[cfg(test)]
mod tests {
	use std::sync::OnceLock;

	use common::{
		models::{
			components::{health::Health, item::Item, position::Position},
			context::ServerContext,
			entity::Entity,
		},
		utils::vector2::Vector2,
	};
	use tokio::runtime::Runtime;

	use crate::{
		config::entity_template::{create_entity_from_template, load_entity_template},
		lua::lua::lua_get_entity,
	};

	#[test]
	fn test_lua_entity_query() {
		let binding: OnceLock<ServerContext> = OnceLock::new();
		binding.get_or_init(|| ServerContext::new());
		binding
			.get()
			.unwrap()
			.add_entity(Entity::new_with_id("test".to_string()));

		assert_eq!("test", lua_get_entity("test", binding).unwrap().id)
	}

	#[test]
	fn test_create_entity_from_template() {
		let rt = Runtime::new().unwrap();
		rt.block_on(async {
			let template = load_entity_template("skeleton").await.unwrap();
			let entity = create_entity_from_template(&template).await;

			assert_eq!(entity.get_component::<Health>().unwrap().current, 100.0);
			assert_eq!(entity.get_component::<Health>().unwrap().max, 100.0);

			let position = entity.get_component::<Position>().unwrap();
			assert_eq!(position.coords, Vector2::new(0.0, 0.0));
			assert_eq!(position.rotation, 0.0);
			assert_eq!(position.map_id, "map01");

			let item = entity.parts[0].get_component::<Item>().unwrap();
			assert_eq!(item.display_name, "Rusty Sword");
			assert_eq!(
				item.description,
				"A worn-out sword carried by the skeleton."
			);
			assert_eq!(item.weight, 2.5);
		});
	}
}
