mod image;
mod model;
mod geometry;

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

    let model = model::Model::new("objs/african_head.obj");
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

    let start_time = std::time::Instant::now();
    let animation_duration = std::time::Duration::from_secs(10);
    let mut last_frame_start = start_time;


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;


        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let delta = start_time.elapsed();
                draw(&mut image, &mut pixels, &model);
                let time_since_last_frame = last_frame_start.elapsed();
                //println!("FPS: {}", 1.0 / time_since_last_frame.as_secs_f32());
                last_frame_start = std::time::Instant::now();
            }
            _ => (),
        }

        window.request_redraw();
    });
}

fn draw(image: &mut image::Image, pixels: &mut Pixels, model: &model::Model) {
    image.clear(image::Color::new(0, 0, 0, 255));
    image.draw_model(model, image::Color::new(255, 0, 255, 255));
    let p0 = geometry::Vec2f::new(10.0, 70.0);
    let p1 = geometry::Vec2f::new(50.0, 160.0);
    let p2 = geometry::Vec2f::new(70.0, 80.0);
    let color = image::Color::new(255, 0, 255, 255);
    image.triangle2d(&p0, &p1, &p2, color);
    image.flip_vertically();
    image.write_to_buffer(pixels.frame_mut());
    pixels.render().unwrap();
}
