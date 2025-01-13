use std::sync::OnceLock;

use mlua::{Lua, Result};

use super::lua_entity::LuaEntity;
use common::models::context::ServerContext;

pub fn run_lua_script(script: &str, context: OnceLock<ServerContext>) -> Result<LuaEntity> {
	let lua = Lua::new();
	let globals = lua.globals();

	globals.set(
		"query_entity",
		lua.create_function(move |_, entity_id: String| {
			let entity = context.get().unwrap().get_entity(&entity_id);
			match entity {
				Some(entity_ref) => Ok(LuaEntity::new(entity_ref.clone())),
				None => Err(mlua::Error::RuntimeError(format!(
					"Entity {} not found",
					entity_id
				))),
			}
		})?,
	)?;
	lua.load(script).eval()
}
