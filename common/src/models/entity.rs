use mlua::{UserData, UserDataMethods};
use tracing::info;

use crate::models::component::{Component, NamedComponent};
use std::collections::HashMap;

use super::lua_component::LuaComponent;

#[derive(Debug, Clone)]
pub struct Entity {
	pub id: String,
	pub components: HashMap<&'static str, Box<dyn Component>>,
	pub parts: Vec<Entity>,
}

impl Entity {
	pub fn new() -> Self {
		Self {
			id: nanoid::nanoid!(16),
			components: Default::default(),
			parts: Default::default(),
		}
	}

	pub fn add_component<T: Component + NamedComponent + Clone + 'static>(
		&mut self,
		component: T,
	) -> bool {
		if self.components.contains_key(T::NAME) {
			false
		} else {
			if let Some(dependant) = component.as_dependant_component() {
				info!("Component {} is a DependantComponent", T::NAME);
				for dependency in dependant.dependencies() {
					if !self.components.contains_key(dependency.as_str()) {
						return false;
					}
				}
			}
			self.components.insert(T::NAME, Box::new(component));
			true
		}
	}

	pub fn add_component_from_lua(&mut self, lua_component: LuaComponent) -> bool {
		if self.components.contains_key(lua_component.name) {
			false
		} else {
			self.components
				.insert(lua_component.name, lua_component.component);
			true
		}
	}

	pub fn get_component<T: Component + NamedComponent + 'static>(&self) -> Option<&T> {
		println!("Getting component: {}", T::NAME);
		self.components.get(T::NAME)?.as_any().downcast_ref::<T>()
	}

	pub fn get_component_by_name(&self, name: &str) -> Option<&Box<dyn Component>> {
		self.components.get(name)
	}

	pub fn has_component<T: NamedComponent>(&self) -> bool {
		self.components.contains_key(T::NAME)
	}

	pub fn remove_component_by_name(&mut self, name: String) -> Option<Box<dyn Component>> {
		self.components.remove(name.as_str())
	}

	pub fn remove_component<T: Component + NamedComponent + 'static>(
		&mut self,
	) -> Option<Box<dyn Component>> {
		self.components.remove(T::NAME)
	}

	pub fn add_part(&mut self, part: Entity) {
		self.parts.push(part);
	}

	pub fn get_part(&self, part_id: &String) -> Option<&Entity> {
		self.parts.iter().find(|part| part.id == *part_id)
	}

	pub fn get_part_mut(&mut self, part_id: &String) -> Option<&mut Entity> {
		self.parts.iter_mut().find(|part| part.id == *part_id)
	}

	pub fn remove_part(&mut self, part_id: &String) {
		self.parts.retain(|part| part.id != *part_id);
	}
}

impl Default for Entity {
	fn default() -> Self {
		Self::new()
	}
}

impl UserData for Entity {
	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_method("get_id", |_, this, ()| Ok(this.id.clone()));
		methods.add_method_mut("add_component", |_, this, lua_component: LuaComponent| {
			this.add_component_from_lua(lua_component);
			Ok(())
		});
	}
}
