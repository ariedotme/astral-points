use std::any::Any;

use crate::models::component::{Component, NamedComponent};

#[derive(Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

impl Component for Health {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NamedComponent for Health {
    const NAME: &'static str = "health";
}
