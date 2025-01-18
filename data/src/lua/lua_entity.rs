use mlua::{Error, FromLua, Lua, UserData, UserDataFields, UserDataMethods, Value};

use common::models::entity::Entity;


#[derive(Debug, Clone)]
pub struct LuaEntity {
	pub id: String,
	pub entity: Box<Entity>,
}

impl LuaEntity {
	pub fn new(entity: Entity) -> Self {
		Self {
			id: entity.id.clone(),
			entity: Box::new(entity),
		}
	}
}

impl UserData for LuaEntity {
	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
		methods.add_method("get_id", |_, this, ()| Ok(this.id.clone()));

		methods.add_method("get_components", |_, this, ()| {
			let component_names: Vec<&str> = this.entity.components.keys().cloned().collect();
			Ok(component_names)
		});

		methods.add_method("get_component", |_, this, name: String| {
			if let Some(component) = this.entity.get_component_by_name(&name) {
				Ok(component.to_lua_component())
			} else {
				Err(Error::RuntimeError(format!("Component {} not found", name)))
			}
		});

		// methods.add_method_mut("add_component", |_, this, lua_component: LuaComponent| {
		//     let success = this.entity.add_component(lua_component.component.into());
		//     Ok(success)
		// });

		methods.add_method_mut("remove_component", |_, this, name: String| {
			let removed = this.entity.remove_component_by_name(name);
			Ok(removed.is_some())
		});
	}

	fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
		fields.add_field_method_get("id", |_, this| Ok(this.id.clone()));
	}
}

impl FromLua for LuaEntity {
	fn from_lua(value: Value, _: &Lua) -> Result<Self, Error> {
		match value {
			Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
			_ => unreachable!(),
		}
	}
}
