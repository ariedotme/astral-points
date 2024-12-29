use std::any::Any;
use std::fmt::Debug;

use mlua::UserData;

use crate::lua::lua_component::LuaComponent;

pub trait Component: Any + Send + Sync + Debug + CloneComponent {
    fn as_any(&self) -> &dyn Any;
    fn to_lua_component(&self) -> LuaComponent;

    fn as_dependant_component(&self) -> Option<&dyn DependantComponent> {
        None
    }
}

// impl<T: Any + Send + Sync + Debug> Component for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

pub trait NamedComponent {
    const NAME: &'static str;
}

pub trait DependantComponent {
    fn dependencies(&self) -> Vec<String>;
}

pub trait CloneComponent {
    fn clone_component(&self) -> Box<dyn Component>;
}

impl<T> CloneComponent for T
where
    T: 'static + Component + Clone,
{
    fn clone_component(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_component()
    }
}
