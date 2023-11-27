use std::{sync::Arc, fmt};

use glam::{Vec3, Mat4, Quat};

use crate::{texture::TextureId, model::ModelId, camera::CameraId, command::InputManager};

type UpdateFn = Box<dyn Fn(&Node, &InputManager, f32)>;

#[derive(Debug)]
pub enum NodeType {
    Mesh(Arc<ModelData>),
    Light(Arc<Vec3>),
    Camera(Arc<CameraEntityData>),
    Group,
}

#[derive(Debug)]
pub struct ModelData {
    pub model_id: ModelId,
    pub texture_id: TextureId,
}

#[derive(Debug)]
pub struct CameraEntityData {
    pub camera: CameraId,
    pub speed: f32,
}

pub struct Node {
    update_fn: UpdateFn,
    pub transformation: Transform,
    pub node_type: NodeType,
    children: Vec<Box<Node>>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("transformation", &self.transformation)
            .field("node_type", &self.node_type)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

impl Node {
    pub fn new(update_fn: UpdateFn, transformation: Transform, node_type: NodeType) -> Self {
        Self {
            update_fn,
            transformation,
            node_type,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }

    pub fn get_transformation(&self) -> Mat4 {
        self.transformation.get_model_matrix()
    }

    pub fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn get_children(&self) -> &Vec<Box<Node>> {
        &self.children
    }

    pub fn traverse<F: FnMut(&Node, Mat4)> (&self, parent_transformation: Mat4, action: &mut F) -> Vec<(Mat4, &NodeType)> {
        let mut result = Vec::new();
        let transformation = parent_transformation * self.transformation.get_model_matrix();
        result.push((transformation, &self.node_type));
        action(self, transformation);
        for child in &self.children {
            result.append(&mut child.traverse(transformation, action));
        }
        result
    }

    fn update(&self, input_manager: &InputManager, delta_time: f32) {
        (self.update_fn)(self, input_manager, delta_time);
        for child in &self.children {
            child.update(input_manager, delta_time);
        }
    }
}
