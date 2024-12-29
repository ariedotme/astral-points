use std::any::Any;

use mlua::UserData;

use crate::{
    lua::lua_component::LuaComponent,
    models::component::{Component, DependantComponent, NamedComponent},
    utils::vector2::Vector2,
};

#[derive(Debug, Clone)]
pub struct Collider {
    pub size: Vector2,
    pub offset: Vector2,
}

impl Collider {
    pub fn new(size: Vector2, offset: Vector2) -> Self {
        Self { size, offset }
    }
}

impl Component for Collider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dependant_component(&self) -> Option<&dyn DependantComponent> {
        Some(self)
    }

    fn to_lua_component(&self) -> LuaComponent {
        LuaComponent::new(self.clone())
    }
}

impl NamedComponent for Collider {
    const NAME: &'static str = "collider";
}

impl DependantComponent for Collider {
    fn dependencies(&self) -> Vec<String> {
        vec![crate::models::components::position::Position::NAME.to_string()]
    }
}
