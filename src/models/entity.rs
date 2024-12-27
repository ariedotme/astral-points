use crate::models::component::{Component, NamedComponent};
use std::collections::HashMap;

pub type EntityId = String;

#[derive(Debug)]
pub struct Entity {
    pub id: EntityId,
    pub components: HashMap<&'static str, Box<dyn Component>>,
    pub parts: Vec<Entity>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(16),
            components: HashMap::new(),
            parts: Vec::new(),
        }
    }

    pub fn add_component<T: Component + NamedComponent + 'static>(&mut self, component: T) -> bool {
        if self.components.contains_key(T::NAME) {
            false
        } else {
            self.components.insert(T::NAME, Box::new(component));
            true
        }
    }

    pub fn get_component<T: Component + NamedComponent + 'static>(&self) -> Option<&T> {
        self.components
            .get(T::NAME)
            .and_then(|comp| comp.as_any().downcast_ref::<T>())
    }

    pub fn remove_component<T: Component + NamedComponent + 'static>(
        &mut self,
    ) -> Option<Box<dyn Component>> {
        self.components.remove(T::NAME)
    }

    pub fn has_component<T: Component + NamedComponent + 'static>(&self) -> bool {
        self.components.contains_key(T::NAME)
    }

    pub fn add_part(&mut self, part: Entity) {
        self.parts.push(part);
    }

    pub fn get_part(&self, part_id: &EntityId) -> Option<&Entity> {
        self.parts.iter().find(|part| part.id == *part_id)
    }

    pub fn get_part_mut(&mut self, part_id: &EntityId) -> Option<&mut Entity> {
        self.parts.iter_mut().find(|part| part.id == *part_id)
    }

    pub fn remove_part(&mut self, part_id: &EntityId) {
        self.parts.retain(|part| part.id != *part_id);
    }
}
