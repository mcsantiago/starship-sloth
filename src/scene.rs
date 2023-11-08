use crate::{model, camera};
use glam::{Vec3, Vec4, Mat4, Affine3A, Quat};

pub struct RenderableModel {
    pub model: model::Model,
    pub transform: Transform,
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn get_model_matrix(&self) -> Affine3A {
        /*
        let identity = Mat4::identity();
        let translation_matrix = Mat4::translation(self.position.x, self.position.y, self.position.z);
        let rotation_matrix = Mat4::rotation(self.rotation.x, self.rotation.y, self.rotation.z);
        let scale_matrix = Mat4::scale(self.scale.x, self.scale.y, self.scale.z);
        translation_matrix * rotation_matrix * scale_matrix
        */
        
        Affine3A::from_scale_rotation_translation(self.scale, 
                                                  Quat::from_xyzw(self.rotation.x, self.rotation.y, self.rotation.z, 0.0), 
                                                  self.position)
    }
}

pub struct Scene {
    pub camera: camera::Camera,
    pub models: Vec<RenderableModel>,
    pub lights: Vec<Vec3>,
}
