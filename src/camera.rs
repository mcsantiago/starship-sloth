use std::collections::HashMap;

use glam::{Vec3, Mat4};

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,

    pub speed: f32,
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

    pub fn rotate_active_camera(&mut self, angle: f32, axis: Vec3) {
        if let Some(camera) = self.get_active_camera_mut() {
            let rotation = Mat4::from_axis_angle(axis, angle);
            let direction = (camera.target - camera.position).normalize();
            camera.target = camera.position + rotation.transform_vector3(direction);
        }
    }
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32, speed: f32) -> Self {
        Self {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            z_near,
            z_far,
            speed
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let camera_direction = Vec3::normalize(self.position - self.target);
        let camera_right = Vec3::normalize(Vec3::cross(world_up, self.target));
        let camera_up = Vec3::normalize(Vec3::cross(camera_direction, camera_right));
        Mat4::look_at_rh(self.position, camera_direction, camera_up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.z_near, self.z_far)
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    pub fn get_front(&self) -> Vec3 {
        Vec3::normalize(self.target - self.position)
    }

    pub fn get_right(&self) -> Vec3 {
        Vec3::normalize(Vec3::cross(self.get_front(), self.up))
    }

    pub fn get_up(&self) -> Vec3 {
        Vec3::normalize(Vec3::cross(self.get_right(), self.get_front()))
    }
}
