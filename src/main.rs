mod image;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::LogicalSize,
};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const WINDOW_TITLE: &str = "Sloth Rasterizer";

fn main() {
    env_logger::init();

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

    let mut frame = 0;
    let mut last_start_frame_time = std::time::Instant::now();
    let animation_start_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let animation_duration = std::time::Duration::from_secs(10);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let delta = animation_start_time.elapsed();
                println!("Delta: {:?}", delta);
                draw(&mut image, &mut pixels, delta, animation_duration);
            }
            _ => (),
        }

        window.request_redraw();
        println!("Frame {} rendered", frame);
        frame += 1;
    });
}

fn draw(image: &mut image::Image, pixels: &mut Pixels, elapsed: std::time::Duration, duration: std::time::Duration) {
    if elapsed > duration {
        return;
    }
    println!("Drawing");
    image.clear(image::Color::new(0, 0, 0, 255));
    image.set_pixel(200, image::Color::new(255, 0, 0, 255)).unwrap();

    let total_change = 600.0;
    let rate_of_change = total_change / 10.0;
    let current_y = (rate_of_change * elapsed.as_secs_f32()).min(600.0);

    image.line(100, current_y as i32, 400, 400, image::Color::new(255, 255, 0, 255));
    image.flip_vertically();
    image.write_to_buffer(pixels.frame_mut());
    pixels.render().unwrap();
}
