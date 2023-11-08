use std::mem::size_of;

use crate::{model, texture, node, camera};
use glam::{IVec2, Vec2, Vec3, Vec4, Mat4};


#[derive(Debug)]
pub struct Renderer {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    z_buffer: Vec<f32>,
}

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
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

    pub fn scale_by_intensity(&self, intensity: f32) -> Self {
        let r = (self.r as f32 * intensity) as u8;
        let g = (self.g as f32 * intensity) as u8;
        let b = (self.b as f32 * intensity) as u8;
        let a = (self.a as f32 * intensity) as u8;
        Self { r, g, b, a }
    }
}

impl Renderer {
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
    #[allow(dead_code)]
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

    pub fn triangle2d(&mut self, p0: Vec3, p1: Vec3, p2: Vec3, uv0: Vec2, uv1: Vec2, uv2: Vec2, texture: &texture::Texture, intensity: f32) {
        let mut bbox_min = IVec2::new(self.width as i32 - 1, self.height as i32 - 1);
        let mut bbox_max = IVec2::new(0, 0);

        for v in &[p0, p1, p2] {
            bbox_min.x = bbox_min.x.min(v.x as i32);
            bbox_min.y = bbox_min.y.min(v.y as i32);
            bbox_max.x = bbox_max.x.max(v.x as i32);
            bbox_max.y = bbox_max.y.max(v.y as i32);
        }

        // Clipping
        bbox_min.x = bbox_min.x.max(0);
        bbox_min.y = bbox_min.y.max(0);
        bbox_max.x = bbox_max.x.min(self.width as i32 - 1);
        bbox_max.y = bbox_max.y.min(self.height as i32 - 1);


        /*
        self.line((bbox_min.x, bbox_min.y), (bbox_max.x, bbox_min.y), Color::new(255, 0, 0, 255));
        self.line((bbox_min.x, bbox_max.y), (bbox_max.x, bbox_max.y), Color::new(255, 0, 0, 255));
        self.line((bbox_min.x, bbox_min.y), (bbox_min.x, bbox_max.y), Color::new(255, 0, 0, 255));
        self.line((bbox_max.x, bbox_min.y), (bbox_max.x, bbox_max.y), Color::new(255, 0, 0, 255));
        */
        
        for x in bbox_min.x..=bbox_max.x {
            for y in bbox_min.y..=bbox_max.y {
                if y == self.height as i32 || x == self.width as i32{
                    continue;
                }
                let pos = Vec3::new(x as f32, y as f32, 0.0);
                let (w1, w2, w3) = self.barycentric(p0, p1, p2, pos);

                let is_inside = w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0;

                if is_inside {
                    let z_interpolated = w1 * p0.z + w2 * p1.z + w3 * p2.z;
                    let index = (x + (y * self.width as i32)) as usize;
                    if self.z_buffer[index] < z_interpolated {
                        self.z_buffer[index] = z_interpolated;
                        let uv_interpolated = self.interpolate_uv(uv0, uv1, uv2, w1, w2, w3);
                        let mut color = texture.sample(uv_interpolated);
                        color = color.scale_by_intensity(intensity);
                        self.set_pixel(index, color).unwrap();
                    }
                }
            }
        }
    }

    fn interpolate_uv(&self, uv0: Vec2, uv1: Vec2, uv2: Vec2, w1: f32, w2: f32, w3: f32) -> Vec2 {
        let u = w1 * uv0.x + w2 * uv1.x + w3 * uv2.x;
        let v = w1 * uv0.y + w2 * uv1.y + w3 * uv2.y;
        Vec2::new(u, v)
    }

    fn barycentric(&mut self, p0: Vec3, p1: Vec3, p2: Vec3, p: Vec3) -> (f32, f32, f32) {
        let w1 = (p0.x * (p2.y - p0.y) + (p.y - p0.y) * (p2.x - p0.x) - p.x * (p2.y - p0.y)) / ((p1.y - p0.y) * (p2.x - p0.x) - (p1.x - p0.x) * (p2.y - p0.y));
        let w2 = (p.y - p0.y - w1 * (p1.y - p0.y))  / (p2.y - p0.y);
        let w3 = 1.0 - w1 - w2;
        (w1, w2, w3)
    }

    pub fn reset_z_buffer(&mut self) {
        for i in 0..self.z_buffer.len() {
            self.z_buffer[i] = std::f32::MIN;
        }
    }

    pub fn draw_model(&mut self, model: &model::Model, texture: &texture::Texture, model_matrix: Mat4, view_matrix: Mat4, projection_matrix: Mat4, _: Color) {
        let light_dir = Vec3::new(0.0, 0.0, -1.0); // This should come from scene

        for (_, face) in model.faces.iter().enumerate() {
            let mut screen_coords: Vec<Vec3> = Vec::new(); // This should come from Camera
            let mut world_coords: Vec<Vec3> = Vec::new();  // This should come from Scene
            let mut texture_coords: Vec<Vec2> = Vec::new(); // This should come from Model

            for (v_idx, vt_idx, vn_idx) in face.iter() {
                let v = model.verts.get(*v_idx as usize).unwrap();
                let vt = model.tex_coords.get(*vt_idx as usize).unwrap();
                let _vn = model.normals.get(*vn_idx as usize).unwrap();

                screen_coords.push(self.transform_vertex(v.clone(), model_matrix, view_matrix, projection_matrix));
                texture_coords.push(Vec2::new(vt.x, (1.0 - vt.y).abs()));
                world_coords.push(Vec3::new(v.x, v.y, v.z));
            }

            let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]).normalize();
            let intensity = n.dot(light_dir);

            if intensity > 0.0 {
                self.triangle2d(screen_coords[0],
                                screen_coords[1],
                                screen_coords[2],
                                // Idk why the texture coords are in this order
                                texture_coords[1],
                                texture_coords[2],
                                texture_coords[0],
                                &texture,
                                intensity);

            }
        }
    }

    fn transform_vertex(&self, vertex: Vec3, model_matrix: Mat4, view_matrix: Mat4, projection_matrix: Mat4) -> Vec3 {
        // Model matrix
        let homogeneous_vertex = Vec4::new(vertex.x, vertex.y, vertex.z, 1.0);
        let world_vertex = model_matrix * homogeneous_vertex;
        let view_vertex = view_matrix * world_vertex;
        let clip_space_vertex = projection_matrix * view_vertex;
        let normalized_vertex = clip_space_vertex / clip_space_vertex.w;

        let screen_vertex = Vec3::new(
            (normalized_vertex.x + 1.0) * self.width as f32 / 2.0 + 0.5,
            (normalized_vertex.y + 1.0) * self.height as f32 / 2.0 + 0.5,
            normalized_vertex.z
        );

        screen_vertex
    }

    pub fn render_scene(&mut self,
                        node: &node::Node,
                        model_manager: &model::ModelManager,
                        texture_manager: &texture::TextureManager,
                        camera_manager: &camera::CameraManager) {
        let root_transform = Mat4::IDENTITY;
        println!("Node: {:?}", node);
        let camera = camera_manager.get_active_camera();
        
        node.traverse(root_transform, &mut |node, world_transform| {
            match &node.node_type {
                node::NodeType::Mesh(mesh) => {
                    match camera {
                        Some(camera) => {
                            let model_matrix = world_transform;
                            let view_matrix = camera.get_view_matrix();
                            let projection_matrix = camera.get_projection_matrix();
                            let model = model_manager.get_model(mesh.model_id);
                            let texture = texture_manager.get_texture(mesh.texture_id);
                            self.draw_model(model, texture, model_matrix, view_matrix, projection_matrix, Color::new(255, 255, 255, 255));
                        },
                        None => {}
                    }
                },
                node::NodeType::Light(_) => {},
                _ => {}
            }
        });
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

