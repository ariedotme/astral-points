use std::any::Any;

use crate::{
	models::component::{Component, NamedComponent},
	models::lua_component::LuaComponent,
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
