use std::any::Any;

use crate::{
    lua::lua_component::LuaComponent,
    models::component::{Component, NamedComponent},
};

#[derive(Debug, Clone)]
pub struct Item {
    pub display_name: String,
    pub description: String,
    pub weight: f32,
}

impl Component for Item {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_lua_component(&self) -> LuaComponent {
        LuaComponent::new(self.clone())
    }
}

impl NamedComponent for Item {
    const NAME: &'static str = "item";
}
