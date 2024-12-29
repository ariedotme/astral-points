use mlua::{Error, FromLua, Lua, UserData, UserDataFields, UserDataMethods, Value};

use crate::models::{component::Component, entity::Entity};

use super::lua_component::LuaComponent;

#[derive(Debug)]
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
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
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

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id.clone()));
    }
}

impl<'lua> FromLua<'lua> for LuaEntity {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let Value::Table(table) = value {
            let id: String = table.get("id")?;
            let component = table.get::<_, LuaEntity>("entity")?;
            Ok(component)
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaEntity",
                message: Some("Expected a table with 'id' and 'entity'".to_string()),
            })
        }
    }
}
