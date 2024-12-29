use std::any::Any;

use crate::{
    lua::lua_component::LuaComponent,
    models::component::{Component, NamedComponent},
};

#[derive(Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

impl Component for Health {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_lua_component(&self) -> LuaComponent {
        LuaComponent::new(self.clone())
    }
}

impl NamedComponent for Health {
    const NAME: &'static str = "health";
}
