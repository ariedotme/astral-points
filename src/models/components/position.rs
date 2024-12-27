use crate::{models::component::NamedComponent, utils::spatial_utils::Vector3};

#[derive(Debug, Clone)]
pub struct Position {
    pub coords: Vector3,
    pub map_id: String, // nano_id(8)
    pub yaw: f32,
    pub pitch: f32,
}

impl Position {
    pub fn new(coords: Vector3, yaw: f32, pitch: f32, map_id: String) -> Self {
        Self {
            coords,
            map_id,
            yaw,
            pitch,
        }
    }

    pub fn set_map_id(&mut self, map_id: String) {
        self.map_id = map_id;
    }

    pub fn set_coords(&mut self, coords: Vector3) {
        self.coords = coords;
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch;
    }
}

impl NamedComponent for Position {
    const NAME: &'static str = "position";
}
