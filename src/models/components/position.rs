use std::any::Any;

use crate::{
    models::component::{Component, NamedComponent},
    utils::spatial_utils::Vector3,
};

#[derive(Debug, Clone)]
pub struct Position {
    pub coords: Vector3,
    pub rotation: Vector3,
    pub map_id: String, // nano_id(8)
}

impl Position {
    pub fn new(coords: Vector3, rotation: Vector3, map_id: String) -> Self {
        Self {
            coords,
            map_id,
            rotation,
        }
    }

    pub fn set_map_id(&mut self, map_id: String) {
        self.map_id = map_id;
    }

    pub fn set_coords(&mut self, coords: Vector3) {
        self.coords = coords;
    }

    pub fn set_rotation(&mut self, rotation: Vector3) {
        self.rotation = rotation;
    }
}

impl Component for Position {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NamedComponent for Position {
    const NAME: &'static str = "position";
}
