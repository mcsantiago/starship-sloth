use std::sync::Arc;

use glam::{Vec3, Mat4};

use crate::{texture::TextureId, model::ModelId};

#[derive(Debug)]
pub enum NodeType {
    Mesh(Arc<ModelData>),
    Light(Arc<Vec3>),
    Group,
}

#[derive(Debug)]
pub struct ModelData {
    pub model_id: ModelId,
    pub texture_id: TextureId,
}

#[derive(Debug)]
pub struct Node {
    pub transformation: Mat4,
    pub node_type: NodeType,
    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(transformation: Mat4, node_type: NodeType) -> Self {
        Self {
            transformation,
            node_type,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }

    pub fn get_transformation(&self) -> Mat4 {
        self.transformation
    }

    pub fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn get_children(&self) -> &Vec<Box<Node>> {
        &self.children
    }

    pub fn traverse<F: FnMut(&Node, Mat4)> (&self, parent_transformation: Mat4, action: &mut F) -> Vec<(Mat4, &NodeType)> {
        let mut result = Vec::new();
        let transformation = parent_transformation * self.transformation;
        result.push((transformation, &self.node_type));
        action(self, transformation);
        for child in &self.children {
            result.append(&mut child.traverse(transformation, action));
        }
        result
    }
}
