#[cfg(test)]
mod tests {
    use crate::config::entity_template::{create_entity_from_template, load_entity_template};
    use crate::models::components::{
        health::Health, item::Item, position::Position, storage::Storage,
    };
    use crate::models::entity::Entity;
    use crate::utils::spatial_utils::Vector3;
    use tokio::runtime::Runtime;

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
    fn test_create_entity_from_template() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let template = load_entity_template("skeleton").await.unwrap();
            let entity = create_entity_from_template(&template);

            assert_eq!(entity.get_component::<Health>().unwrap().current, 100.0);
            assert_eq!(entity.get_component::<Health>().unwrap().max, 100.0);

            let position = entity.get_component::<Position>().unwrap();
            assert_eq!(position.coords, Vector3::new(0.0, 0.0, 0.0));
            assert_eq!(position.rotation, Vector3::new(0.0, 0.0, 0.0));
            assert_eq!(position.map_id, "map01");

            let item = entity.get_component::<Item>().unwrap();
            assert_eq!(item.display_name, "Rusty Sword");
            assert_eq!(
                item.description,
                "A worn-out sword carried by the skeleton."
            );
            assert_eq!(item.weight, 2.5);
        });
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
}
