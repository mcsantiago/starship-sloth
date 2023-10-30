mod error;

use std::mem::size_of;

use error::Error;

use crate::{geometry::{PixelPoint, Vec3f, Vec2i, compute_line_parameters}, model};

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    z_buffer: Vec<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * size_of::<Color>()) as usize],
            z_buffer: vec![std::f32::MAX; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, index: usize, color: Color) -> Result<(), Error> {
        if index >= self.width as usize * self.height as usize {
            return Err(Error::OutOfBounds);
        }
        self.pixels[index * 4] = color.r;
        self.pixels[index * 4 + 1] = color.g;
        self.pixels[index * 4 + 2] = color.b;
        self.pixels[index * 4 + 3] = color.a;
        Ok(())
    }

    pub fn get_pixel_color(&self, index: usize) -> Result<Color, Error> {
        if index >= self.width as usize * self.height as usize {
            return Err(Error::OutOfBounds);
        }

        Ok(Color {
            r: self.pixels[index * 4],
            g: self.pixels[index * 4 + 1],
            b: self.pixels[index * 4 + 2],
            a: self.pixels[index * 4 + 3],
        })
    }

    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                let index = (i + j * self.width) as usize;
                self.set_pixel(index, color).unwrap();
            }
        }
    }

    ////////////////////////////////////////////////////////////////
    // Bresenham's line algorithm
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    //
    // Assumes that (0, 0) is at the center of the screen
    // and that the coordinates are discrete values for pixels
    ////////////////////////////////////////////////////////////////
    pub fn line(&mut self, start: (i32, i32), end: (i32, i32), color: Color) {
        let mut x0 = start.0;
        let mut y0 = start.1;
        let mut x1 = end.0;
        let mut y1 = end.1;


        if x0 < 0 || x0 >= self.width as i32 || y0 < 0 || y0 >= self.height as i32 ||
           x1 < 0 || x1 >= self.width as i32 || y1 < 0 || y1 >= self.height as i32 {
            // Maybe we consider a partial rendering
            return;
        }

        let mut steep = false;
        if (x0 - x1).abs() < (y0 - y1).abs() {
            // swap x0, y0
            let temp = x0;
            x0 = y0;
            y0 = temp;

            // swap x1, y1
            let temp = x1;
            x1 = y1;
            y1 = temp;

            steep = true;
        }

        if x0 > x1 {
            // swap x0, x1
            let temp = x0;
            x0 = x1;
            x1 = temp;

            // swap y0, y1
            let temp = y0;
            y0 = y1;
            y1 = temp;
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs()*2;
        let mut error2 = 0;

        let mut y = y0;

        for x in x0..x1 {
            if steep {
                let index = (y + x * self.width as i32) as usize;
                self.set_pixel(index, color).unwrap();
            } else {
                let index = (x + y * self.width as i32) as usize;
                self.set_pixel(index, color).unwrap();
            }
            error2 += derror2;
            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }

    pub fn triangle2d<P: PixelPoint + Copy>(&mut self, mut p0: P, mut p1: P, mut p2: P, color: Color) {
        // TODO: Maybe consider barycentric coordinates for interpolation of values within the triangle

        // Bubble sort the points by y coordinate
        if p0.to_i32_tuple().1 > p1.to_i32_tuple().1 {
            let temp = p0;
            p0 = p1;
            p1 = temp;
        }
        if p0.to_i32_tuple().1 > p2.to_i32_tuple().1 {
            let temp = p0;
            p0 = p2;
            p2 = temp;
        }
        if p1.to_i32_tuple().1 > p2.to_i32_tuple().1 {
            let temp = p1;
            p1 = p2;
            p2 = temp;
        }

        let p0_tuple = p0.to_i32_tuple();
        let p1_tuple = p1.to_i32_tuple();
        let p2_tuple = p2.to_i32_tuple();

        let line_a = compute_line_parameters(p0_tuple, p2_tuple);
        let line_b = compute_line_parameters(p0_tuple, p1_tuple);
        let line_c = compute_line_parameters(p1_tuple, p2_tuple);

        for y in p0_tuple.1 .. p1_tuple.1 {
            let start_x = match line_a {
                Some((gradient, intercept)) => ((y as f32 - intercept) / gradient) as i32,
                None => p0_tuple.0,
            };
            let end_x = match line_b {
                Some((gradient, intercept)) => ((y as f32 - intercept) / gradient) as i32,
                None => p0_tuple.0,
            };
            self.line((start_x, y), (end_x, y), color);
        }
        for y in p1_tuple.1 .. p2_tuple.1 {
            let start_x = match line_a {
                Some((gradient, intercept)) => ((y as f32 - intercept) / gradient) as i32,
                None => p0_tuple.0,
            };
            let end_x = match line_c {
                Some((gradient, intercept)) => ((y as f32 - intercept) / gradient) as i32,
                None => p1_tuple.0,
            };
            self.line((start_x, y), (end_x, y), color);
        }
    }


    fn reset_z_buffer(&mut self) {
        for i in 0..self.z_buffer.len() {
            self.z_buffer[i] = std::f32::MAX;
        }
    }

    pub fn draw_model(&mut self, model: &model::Model, color: Color) {
        let light_dir = Vec3f::new(0.0, 0.0, -1.0); // This should come from scene

        for (_, face) in model.faces.iter().enumerate() {
            let mut screen_coords: Vec<Vec2i> = Vec::new(); // This should come from Camera
            let mut world_coords: Vec<Vec3f> = Vec::new();  // This should come from Scene

            for idx in face.iter() {
                let v = model.verts.get(*idx as usize).unwrap();
                let x = ((v.x + 1.0) * self.width as f32 / 2.0) as i32;
                let y = ((v.y + 1.0) * self.height as f32 / 2.0) as i32;
                screen_coords.push(Vec2i::new(x, y));
                world_coords.push(Vec3f::new(v.x, v.y, v.z));
            }

            let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]).normalize();
            let intensity = n.dot(&light_dir);
            if intensity > 0.0 {
                self.triangle2d(screen_coords[0],
                                screen_coords[1],
                                screen_coords[2],
                                Color::new((intensity * color.r as f32) as u8,
                                           (intensity * color.g as f32) as u8,
                                           (intensity * color.b as f32) as u8,
                                           color.a));
            }
        }
    }

    pub fn flip_vertically(&mut self) {
        for i in 0..self.height / 2 {
            for j in 0..self.width {
                let current_index = i * self.width + j;
                let opposite_index = (self.height - i - 1) * self.width + j;

                self.pixels.swap(current_index * 4, opposite_index * 4);
                self.pixels.swap(current_index * 4 + 1, opposite_index * 4 + 1);
                self.pixels.swap(current_index * 4 + 2, opposite_index * 4 + 2);
                self.pixels.swap(current_index * 4 + 3, opposite_index * 4 + 3);
            }
        }
    }

    #[allow(dead_code)]
    pub fn save(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let idx = (x + y * self.width as u32) as usize;
            match self.get_pixel_color(idx) {
                Ok(color) => {
                    *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            }
        }
        imgbuf.save(filename).unwrap();
    }

    pub fn write_to_buffer(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            match self.get_pixel_color(i as usize) {
                Ok(color) => {
                    pixel.copy_from_slice(&[color.r, color.g, color.b, color.a]);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            }
        }
    }
}
