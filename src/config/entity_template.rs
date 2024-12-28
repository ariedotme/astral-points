use futures::future::BoxFuture;
use std::{collections::HashMap, future::Future, pin::Pin};
use tokio::fs;

use serde::{Deserialize, Serialize};

use crate::{
    models::{
        components::{
            health::Health, item::Item, name::Name, player::Player, position::Position,
            storage::Storage,
        },
        entity::Entity,
    },
    utils::spatial_utils::Vector3,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Int(i32),
    UInt(u32),
    Float(f32),
    String(String),
    Bool(bool),
    List(Vec<PropertyValue>),
    Map(HashMap<String, PropertyValue>),
    Vec(Vector3),
}

#[derive(Debug, Deserialize)]
pub struct EntityTemplate {
    pub name: Option<String>,
    pub template: Option<String>,
    #[serde(default)]
    pub components: Vec<ComponentTemplate>,
    #[serde(default)]
    pub children: Vec<EntityTemplate>,
}

fn default_components() -> Vec<ComponentTemplate> {
    Vec::new()
}

fn default_children() -> Vec<EntityTemplate> {
    Vec::new()
}

#[derive(Debug, Deserialize)]
pub struct ComponentTemplate {
    pub name: String,
    pub properties: HashMap<String, PropertyValue>,
}

pub async fn load_entity_template(
    name: &str,
) -> Result<EntityTemplate, Box<dyn std::error::Error>> {
    let yaml_content =
        fs::read_to_string(format!("./src/data/entity_templates/{}.yaml", name)).await?;
    let template: EntityTemplate = serde_yaml::from_str(&yaml_content)?;
    Ok(template)
}

macro_rules! extract_property {
    ($map:expr, $key:expr, $variant:path) => {
        match $map.get($key) {
            Some($variant(value)) => value.clone(),
            _ => panic!("Property {} not found", $key),
        }
    };
}

macro_rules! extract_property_option {
    ($map:expr, $key:expr, $variant:path) => {
        match $map.get($key) {
            Some($variant(value)) => Some(value.clone()),
            _ => None,
        }
    };
}

pub async fn create_entity_from_template(template: &EntityTemplate) -> Entity {
    if let Some(template_name) = &template.template {
        let new_template = load_entity_template(&template_name).await.unwrap();
        return Box::pin(create_entity_from_template(&new_template)).await;
    }

    let mut entity = Entity::new();

    for child in &template.children {
        let child_entity = Box::pin(create_entity_from_template(&child)).await;
        entity.add_part(child_entity);
    }

    for component in &template.components {
        match component.name.as_str() {
            "health" => {
                let current =
                    extract_property!(component.properties, "current", PropertyValue::Float);
                let max = extract_property!(component.properties, "max", PropertyValue::Float);

                entity.add_component(Health {
                    current: current,
                    max: max,
                });
            }
            "position" => {
                let coords = extract_property!(component.properties, "coords", PropertyValue::Map);
                let x = extract_property!(coords, "x", PropertyValue::Float);
                let y = extract_property!(coords, "y", PropertyValue::Float);
                let z = extract_property!(coords, "z", PropertyValue::Float);

                let rotation =
                    extract_property!(component.properties, "rotation", PropertyValue::Map);
                let yaw = extract_property!(rotation, "yaw", PropertyValue::Float);
                let pitch = extract_property!(rotation, "pitch", PropertyValue::Float);
                let roll = extract_property!(rotation, "roll", PropertyValue::Float);

                let map_id =
                    extract_property!(component.properties, "map_id", PropertyValue::String);

                entity.add_component(Position {
                    coords: Vector3::new(x, y, z),
                    rotation: Vector3::new(yaw, pitch, roll),
                    map_id: map_id.clone(),
                });
            }
            "storage" => {
                let capacity =
                    extract_property!(component.properties, "capacity", PropertyValue::UInt);

                let max_weight =
                    extract_property!(component.properties, "max_weight", PropertyValue::Float);

                let stored_items = extract_property_option!(
                    component.properties,
                    "stored_items",
                    PropertyValue::List
                )
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|item| {
                    if let PropertyValue::String(s) = item {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();

                entity.add_component(Storage {
                    capacity,
                    stored_items,
                    max_weight,
                });
            }
            "player" => {
                let username =
                    extract_property!(component.properties, "username", PropertyValue::String);

                entity.add_component(Player { username });
            }
            "name" => {
                let firstname = extract_property_option!(
                    component.properties,
                    "firstname",
                    PropertyValue::String
                );

                let lastname = extract_property_option!(
                    component.properties,
                    "lastname",
                    PropertyValue::String
                );

                let nickname = extract_property_option!(
                    component.properties,
                    "nickname",
                    PropertyValue::String
                );

                entity.add_component(Name {
                    firstname,
                    lastname,
                    nickname,
                });
            }
            "item" => {
                let display_name =
                    extract_property!(component.properties, "display_name", PropertyValue::String);
                let weight =
                    extract_property!(component.properties, "weight", PropertyValue::Float);
                let description =
                    extract_property!(component.properties, "description", PropertyValue::String);

                entity.add_component(Item {
                    display_name,
                    weight,
                    description,
                });
            }
            // TODO: Handle other components
            _ => {}
        }
    }

    entity
}
