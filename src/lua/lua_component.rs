use mlua::{FromLua, Lua, UserData, UserDataMethods, Value};

use crate::models::component::{Component, NamedComponent};

#[derive(Debug)]
pub struct LuaComponent {
    pub name: &'static str,
    pub component: Box<dyn Component>,
}

impl LuaComponent {
    pub fn new<T: Component + NamedComponent + 'static>(component: T) -> Self {
        Self {
            name: T::NAME,
            component: Box::new(component),
        }
    }
}

impl UserData for LuaComponent {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_name", |_, this, ()| Ok(this.name.to_string()));
    }
}

impl<'lua> FromLua<'lua> for LuaComponent {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let Value::Table(table) = value {
            let name: String = table.get("name")?;
            let component = table.get::<_, LuaComponent>("component")?;
            Ok(component)
        } else {
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaComponent",
                message: Some("Expected a table with 'name' and 'component'".to_string()),
            })
        }
    }
}
