use std::any::Any;

use crate::models::component::{Component, NamedComponent};

use super::health::Health;

#[derive(Debug, Clone)]
pub struct Name {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub nickname: Option<String>,
}

impl Name {
    pub fn new(
        firstname: Option<String>,
        lastname: Option<String>,
        nickname: Option<String>,
    ) -> Self {
        Self {
            firstname,
            lastname,
            nickname,
        }
    }

    pub fn full_name(&self) -> String {
        let firstname = self.firstname.as_deref().unwrap_or("Unnamed");
        let lastname = self.lastname.as_deref().unwrap_or("");
        if lastname.is_empty() {
            firstname.to_string()
        } else {
            format!("{} {}", firstname, lastname)
        }
    }
}

impl Component for Name {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NamedComponent for Name {
    const NAME: &'static str = "name";
}
