use std::sync::Arc;

use crate::context::ServerContext;
use mlua::{Lua, Result};

use super::lua_entity::LuaEntity;

pub fn execute_lua_script(script_path: &str, context: Arc<ServerContext>) -> Result<()> {
    let lua = Lua::new();

    let context_clone = context;
    let globals = lua.globals();

    globals.set(
        "query_entity",
        lua.create_function(move |_, entity_id: String| {
            let entity = context_clone.get_entity(&entity_id);
            match entity {
                Some(entity_ref) => Ok(LuaEntity::new(entity_ref.clone())),
                None => Err(mlua::Error::RuntimeError(format!(
                    "Entity {} not found",
                    entity_id
                ))),
            }
        })?,
    )?;

    let script = std::fs::read_to_string(script_path)?;
    lua.load(&script).exec()
}
