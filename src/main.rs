mod image;

use log::{info, debug, error};
use pixels::{Pixels, Error, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::LogicalSize,
};


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "Sloth Rasterizer";

fn main() {
    env_logger::init();
    error!("Debug message");

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
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut image = image::Image::new(WIDTH, HEIGHT);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                image.clear(image::Color::new(255, 0, 0, 255));
                image.write_to_buffer(pixels.frame_mut());
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}
