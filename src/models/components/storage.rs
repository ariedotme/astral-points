use crate::models::component::NamedComponent;
use crate::models::entity::EntityId;

#[derive(Debug, Clone)]
pub struct Storage {
    pub capacity: f32,
    pub max_weight: f32,
    pub stored_items: Vec<EntityId>,
}

impl NamedComponent for Storage {
    const NAME: &'static str = "storage";
}
