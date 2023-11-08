mod renderer;
mod model;
mod texture;
mod camera;
mod node;

use std::sync::Arc;

use node::ModelData;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::LogicalSize,
};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const WINDOW_TITLE: &str = "SlothEngine";

fn main() {
    env_logger::init();

    let screenshot = std::env::args().any(|arg| arg == "--screenshot");

    let mut model_manager = model::ModelManager::new();
    let mut texture_manager = texture::TextureManager::new();

    let model_id = model_manager.load_model("objs/african_head.obj");
    let texture_id = texture_manager.load_texture("objs/african_head_diffuse.tga");

    let model_data = Arc::new(ModelData {
        model_id,
        texture_id,
    });

    println!("model_data: {:?}", model_data);

    let mut scene_root = node::Node::new(glam::Mat4::IDENTITY, node::NodeType::Group);
    scene_root.add_child(node::Node::new(glam::Mat4::IDENTITY, node::NodeType::Mesh(Arc::clone(&model_data))));
    //scene_root.add_child(node::Node::new(glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -5.0)), node::NodeType::Mesh(Arc::clone(&model_data))));

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
        draw(&mut renderer, &mut pixels, &model_manager, &texture_manager, &scene_root, screenshot);

    } else {
        let start_time = std::time::Instant::now();
        let animation_duration = std::time::Duration::from_secs(10);
        let mut last_frame_start = start_time;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;


            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::RedrawRequested(_) => {
                    let delta = start_time.elapsed();
                    draw(&mut renderer, &mut pixels, &model_manager, &texture_manager, &scene_root, screenshot);
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

fn draw(image: &mut renderer::Renderer, pixels: &mut Pixels, model_manager: &model::ModelManager, texture_manager: &texture::TextureManager, node: &node::Node, is_screenshot: bool) {
    image.reset_z_buffer();
    image.clear(renderer::Color::new(0, 0, 0, 255));
    image.render_scene(node, model_manager, texture_manager);
    image.flip_vertically();
    if is_screenshot {
        image.save("screenshot.png");
    } else {
        image.write_to_buffer(pixels.frame_mut());
        pixels.render().unwrap();
    }
}
