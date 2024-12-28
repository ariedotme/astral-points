use std::any::Any;

use crate::models::component::{Component, NamedComponent};
use crate::models::entity::EntityId;

#[derive(Debug, Clone)]
pub struct Storage {
    pub capacity: u32,
    pub max_weight: f32,
    pub stored_items: Vec<EntityId>,
}

impl Component for Storage {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NamedComponent for Storage {
    const NAME: &'static str = "storage";
}
