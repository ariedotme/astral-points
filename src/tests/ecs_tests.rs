#[cfg(test)]
mod tests {
    use crate::config::entity_template::{create_entity_from_template, load_entity_template};
    use crate::models::components::collider::Collider;
    use crate::models::components::{
        health::Health, item::Item, position::Position, storage::Storage,
    };
    use crate::models::entity::Entity;
    use crate::utils::collision::check_collision;
    use crate::utils::vector2::Vector2;
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
}
