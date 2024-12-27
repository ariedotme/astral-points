use std::collections::HashMap;
use tokio::fs;

use serde::{Deserialize, Serialize};

use crate::{
    models::{
        components::{health::Health, position::Position},
        entity::Entity,
    },
    utils::spatial_utils::Vector3,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    List(Vec<PropertyValue>),
    Map(HashMap<String, PropertyValue>),
    Vec(Vector3),
}

#[derive(Debug, Deserialize)]
pub struct EntityTemplate {
    pub name: String,
    pub components: Vec<ComponentTemplate>,
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

pub fn create_entity_from_template(template: &EntityTemplate) -> Entity {
    let mut entity = Entity::new();

    for component in &template.components {
        match component.name.as_str() {
            "health" => {
                if let (Some(PropertyValue::Float(current)), Some(PropertyValue::Float(max))) = (
                    component.properties.get("current"),
                    component.properties.get("max"),
                ) {
                    entity.add_component(Health {
                        current: *current,
                        max: *max,
                    });
                }
            }
            "position" => {
                if let Some(PropertyValue::Map(coords)) = component.properties.get("coords") {
                    if let (
                        Some(PropertyValue::Float(x)),
                        Some(PropertyValue::Float(y)),
                        Some(PropertyValue::Float(z)),
                    ) = (coords.get("x"), coords.get("y"), coords.get("z"))
                    {
                        entity.add_component(Position {
                            coords: Vector3::new(*x, *y, *z),
                            yaw: component
                                .properties
                                .get("yaw")
                                .and_then(|p| match p {
                                    PropertyValue::Float(v) => Some(*v),
                                    _ => None,
                                })
                                .unwrap_or(0.0),
                            pitch: component
                                .properties
                                .get("pitch")
                                .and_then(|p| match p {
                                    PropertyValue::Float(v) => Some(*v),
                                    _ => None,
                                })
                                .unwrap_or(0.0),
                            map_id: component
                                .properties
                                .get("map_id")
                                .and_then(|p| match p {
                                    PropertyValue::String(s) => Some(s.clone()),
                                    _ => None,
                                })
                                .unwrap_or("unknown".to_string()),
                        });
                    }
                }
            }
            // TODO: Handle other components
            _ => {}
        }
    }

    entity
}
