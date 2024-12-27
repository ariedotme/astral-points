use crate::models::component::NamedComponent;

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

impl NamedComponent for Player {
    const NAME: &'static str = "player";
}
