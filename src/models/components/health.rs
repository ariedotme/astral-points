use crate::models::component::NamedComponent;

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

impl NamedComponent for Health {
    const NAME: &'static str = "health";
}
