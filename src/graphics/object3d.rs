use crate::graphics::mesh::Mesh;
use crate::utils::spatial_utils::Vector3;

#[derive(Debug, Clone)]
pub struct Object3D {
    pub mesh: Mesh,
    pub position: Vector3,
    pub rotation: Vector3, // Euler angles (yaw, pitch, roll)
    pub scale: Vector3,
}

impl Object3D {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn translate(&mut self, translation: Vector3) {
        self.position = Vector3::new(
            self.position.x + translation.x,
            self.position.y + translation.y,
            self.position.z + translation.z,
        );
    }

    pub fn rotate(&mut self, rotation: Vector3) {
        self.rotation = Vector3::new(
            self.rotation.x + rotation.x,
            self.rotation.y + rotation.y,
            self.rotation.z + rotation.z,
        );
    }

    pub fn scale(&mut self, scaling: Vector3) {
        self.scale = Vector3::new(
            self.scale.x * scaling.x,
            self.scale.y * scaling.y,
            self.scale.z * scaling.z,
        );
    }
}
