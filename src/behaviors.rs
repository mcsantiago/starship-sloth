use glam::Vec3;

use crate::{command::InputManager, camera::Camera, scene::{Node, NodeType, self}};

pub fn standard_camera_update(node: &Node, input_manager: &InputManager, delta: f32) {
    /*
    if let NodeType::Camera(camera) = &node.node_type {
        let mut direction = Vec3::ZERO;
        let speed = 5.0;
        if input_manager.is_key_pressed(winit::event::VirtualKeyCode::W) {
            direction += camera.get_front();
        }
        if input_manager.is_key_pressed(winit::event::VirtualKeyCode::S) {
            direction -= camera.get_front();
        }
        if input_manager.is_key_pressed(winit::event::VirtualKeyCode::A) {
            direction -= camera.get_right();
        }
        if input_manager.is_key_pressed(winit::event::VirtualKeyCode::D) {
            direction += camera.get_right();
        }
        if input_manager.is_key_pressed(winit::event::VirtualKeyCode::Space) {
            direction += camera.get_up();
        }
        if input_manager.is_key_pressed(winit::event::VirtualKeyCode::LShift) {
            direction -= camera.get_up();
        }
        camera.translate(direction * delta * speed);
    }
    */
}

