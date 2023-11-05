mod image;
mod model;
mod geometry;
mod texture;

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

    let model = model::Model::new("objs/african_head.obj");
    let mut texture_manager = texture::TextureManager::new();
    let texture_id = texture_manager.load_texture("objs/african_head_diffuse.tga");

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

    let mut image = image::Image::new(WIDTH, HEIGHT);


    if screenshot {
        println!("Taking screenshot...");
        draw(&mut image, &mut pixels, &model, &texture_manager, texture_id, screenshot);

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
                    draw(&mut image, &mut pixels, &model, &texture_manager, texture_id, screenshot);
                    let time_since_last_frame = last_frame_start.elapsed();
                    println!("FPS: {}", 1.0 / time_since_last_frame.as_secs_f32());
                    last_frame_start = std::time::Instant::now();
                }
                _ => (),
            }

            window.request_redraw();
        });
    }

}

fn draw(image: &mut image::Image, pixels: &mut Pixels, model: &model::Model, texture_manager: &texture::TextureManager, texture_id: u8, is_screenshot: bool) {
    let texture = texture_manager.get_texture(texture_id);
    image.reset_z_buffer();
    image.clear(image::Color::new(0, 0, 0, 255));
    image.draw_model(model, texture, image::Color::new(255, 0, 255, 255));
    image.flip_vertically();
    if is_screenshot {
        image.save("screenshot.png");
    } else {
        image.write_to_buffer(pixels.frame_mut());
        pixels.render().unwrap();
    }
}
