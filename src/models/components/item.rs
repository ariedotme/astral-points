use crate::models::component::NamedComponent;

#[derive(Debug, Clone)]
pub struct Item {
    pub display_name: String,
    pub description: String,
    pub weight: f32,
}

impl NamedComponent for Item {
    const NAME: &'static str = "item";
}
