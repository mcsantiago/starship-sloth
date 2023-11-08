use std::collections::HashMap;

use glam::{Vec3, Mat4};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CameraId(usize);

pub struct CameraManager {
    pub cameras: HashMap<CameraId, Camera>,
    pub active_camera: Option<CameraId>,
}

impl CameraManager {
    pub fn new() -> Self {
        Self {
            cameras: HashMap::new(),
            active_camera: None
        }
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraId {
        let id = CameraId(self.cameras.len());
        self.cameras.insert(id, camera);
        id
    }

    pub fn get_active_camera(&self) -> Option<&Camera> {
        match self.active_camera {
            Some(id) => self.cameras.get(&id),
            None => None,
        }
    }

    pub fn get_active_camera_mut(&mut self) -> Option<&mut Camera> {
        match self.active_camera {
            Some(id) => self.cameras.get_mut(&id),
            None => None,
        }
    }

    pub fn set_active_camera(&mut self, id: CameraId) {
        self.active_camera = Some(id);
    }
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            z_near,
            z_far,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.z_near, self.z_far)
    }
}
