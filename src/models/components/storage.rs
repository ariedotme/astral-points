use std::any::Any;

use crate::{
    lua::lua_component::LuaComponent,
    models::component::{Component, NamedComponent},
};

#[derive(Debug, Clone)]
pub struct Storage {
    pub capacity: u32,
    pub max_weight: f32,
    pub stored_items: Vec<String>,
}

impl Component for Storage {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_lua_component(&self) -> LuaComponent {
        LuaComponent::new(self.clone())
    }
}

impl NamedComponent for Storage {
    const NAME: &'static str = "storage";
}
