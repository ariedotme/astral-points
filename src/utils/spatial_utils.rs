use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Vector3) -> f32 {
        (((self.x - other.x).powi(2)) + ((self.y - other.y).powi(2)) + ((self.z - other.z).powi(2)))
            .sqrt()
    }
}
