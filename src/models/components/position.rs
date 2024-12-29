use std::any::Any;

use crate::{
    lua::lua_component::LuaComponent,
    models::component::{Component, NamedComponent},
    utils::vector2::Vector2,
};

#[derive(Debug, Clone)]
pub struct Position {
    pub coords: Vector2,
    pub rotation: f32,
    pub map_id: String, // nano_id(8)
}

impl Position {
    pub fn new(coords: Vector2, rotation: f32, map_id: String) -> Self {
        Self {
            coords,
            map_id,
            rotation,
        }
    }

    pub fn set_map_id(&mut self, map_id: String) {
        self.map_id = map_id;
    }

    pub fn set_coords(&mut self, coords: Vector2) {
        self.coords = coords;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
}

impl Component for Position {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_lua_component(&self) -> LuaComponent {
        LuaComponent::new(self.clone())
    }
}

impl NamedComponent for Position {
    const NAME: &'static str = "position";
}
