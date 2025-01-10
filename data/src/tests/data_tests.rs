#[cfg(test)]
mod tests {
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