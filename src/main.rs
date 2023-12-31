mod renderer;
mod model;
mod texture;
mod camera;
mod scene;
mod command;
mod behaviors;

use std::sync::Arc;

use behaviors::standard_camera_update;
use glam::Vec3;
use scene::{ModelData, Transform};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::LogicalSize,
};
use command::InputManager;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const WINDOW_TITLE: &str = "Sloth Engine";

fn main() {
    env_logger::init();

    let screenshot = std::env::args().any(|arg| arg == "--screenshot");

    let mut model_manager = model::ModelManager::new();
    let mut texture_manager = texture::TextureManager::new();
    let mut camera_manager = camera::CameraManager::new();

    let model_id = model_manager.load_model("objs/african_head.obj");
    let texture_id = texture_manager.load_texture("objs/african_head_diffuse.tga");
    let camera_id = camera_manager.add_camera(
        camera::Camera::new(Vec3::new(0.0, 0.0, 8.0),
                            Vec3::new(0.0, 0.0, -5.0),
                            Vec3::new(0.0, 1.0, 0.0),
                            45.0,
                            WIDTH as f32 / HEIGHT as f32,
                            0.1,
                            100.0,
                            0.05));
    camera_manager.set_active_camera(camera_id);

    let model_data = Arc::new(ModelData {
        model_id,
        texture_id,
    });

    let camera_data = Arc::new(scene::CameraEntityData {
        camera: camera_id,
        speed: 0.1,
    });

    //println!("model_data: {:?}", model_data);

    let position = glam::Vec3::new(0.0, 0.0, 0.0);
    let rotation = glam::Quat::from_rotation_y(0.0);
    let scale = glam::Vec3::new(1.0, 1.0, 1.0);
    let mut scene_root = scene::Node::new(
        Box::new(|_node, _input_manager, _delta_time| {}), 
        Transform::new(position, rotation, scale), 
        scene::NodeType::Group);

    scene_root.add_child(
        scene::Node::new(
            Box::new(|_node, _input_manager, _delta_time| {}), 
            Transform::new(
                glam::Vec3::new(3.0, 0.0, -5.0), 
                rotation, 
                scale), 
            scene::NodeType::Mesh(Arc::clone(&model_data))));

    scene_root.add_child(
        scene::Node::new(
            Box::new(|_node, _input_manager, _delta_time| {}), 
            Transform::new(
                glam::Vec3::new(-3.0, 0.0, -2.0), 
                rotation, 
                scale), 
            scene::NodeType::Mesh(Arc::clone(&model_data))));

    scene_root.add_child(
        scene::Node::new(
            Box::new(|_node, _input_manager, _delta_time| {}), 
            Transform::new(
                glam::Vec3::new(-5.0, 2.0, -5.0), 
                rotation, 
                scale), 
            scene::NodeType::Mesh(Arc::clone(&model_data))));

    scene_root.add_child(
        scene::Node::new(
            Box::new(|node, input_manager, delta_time| {
                standard_camera_update(node, input_manager, delta_time)
            }), 
            Transform::new(
                glam::Vec3::new(0.0, 0.0, -5.0), 
                rotation, 
                scale), 
            scene::NodeType::Camera(Arc::clone(&camera_data))));

    /*
    scene_root.add_child(scene::Node::new(standard_camera_update, glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -5.0)), scene::NodeType::Camera(Arc::clone(&camera_data))));
    */

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);

        WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let mut renderer = renderer::Renderer::new(WIDTH, HEIGHT);

    if screenshot {
        println!("Taking screenshot...");
        renderer.render_scene(&scene_root, &model_manager, &texture_manager, &camera_manager, &mut pixels, screenshot);

    } else {
        let start_time = std::time::Instant::now();
        let animation_duration = std::time::Duration::from_secs(10);
        let mut last_frame_start = start_time;
        let mut input_manager = InputManager::new();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                        // TODO: Handle mouse movement
                        match button {
                            winit::event::MouseButton::Left => {
                                if state == winit::event::ElementState::Pressed {
                                    println!("Left mouse button pressed");
                                } else {
                                    println!("Left mouse button released");
                                }
                            },
                            winit::event::MouseButton::Right => {
                                if state == winit::event::ElementState::Pressed {
                                    println!("Right mouse button pressed");
                                } else {
                                    println!("Right mouse button released");
                                }
                            },
                            winit::event::MouseButton::Middle => {
                                if state == winit::event::ElementState::Pressed {
                                    println!("Middle mouse button pressed");
                                } else {
                                    println!("Middle mouse button released");
                                }
                            },
                            winit::event::MouseButton::Other(_) => (),
                        }
                    },
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        println!("Cursor moved: {:?}", position);
                        let mut camera = camera_manager.get_active_camera_mut();
                        match camera {
                            Some(camera) => {
                            },
                            None => (),
                        }
                    },
                    WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                        if let Some(keycode) = input.virtual_keycode {
                            let is_pressed = input.state == winit::event::ElementState::Pressed;
                            input_manager.set_key_pressed(keycode, is_pressed);
                        }
                    },
                    _ => (), 
                },
                Event::RedrawRequested(_) => {
                    let delta = start_time.elapsed();
                    renderer.render_scene(&scene_root, &model_manager, &texture_manager, &camera_manager, &mut pixels, screenshot);
                    let time_since_last_frame = last_frame_start.elapsed();
                    //println!("FPS: {}", 1.0 / time_since_last_frame.as_secs_f32());
                    last_frame_start = std::time::Instant::now();
                }
                _ => (),
            }

            window.request_redraw();
        });
    }
}
