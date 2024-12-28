use std::any::Any;

use crate::models::component::{Component, NamedComponent};

#[derive(Debug, Clone)]
pub struct Player {
    pub username: String,
}

impl Player {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}

impl Component for Player {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NamedComponent for Player {
    const NAME: &'static str = "player";
}
