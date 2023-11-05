use std::collections::HashMap;

use crate::renderer::Color;
use crate::geometry;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn sample(&self, uv: geometry::Vec2f) -> Color {
        // Clamp the values of u and v to ensure they're within the texture bounds
        let x = (uv.x.clamp(0.0, 1.0) * self.width as f32) as usize;
        let y = (uv.y.clamp(0.0, 1.0) * self.height as f32) as usize;

        // Ensure the index does not go beyond the texture's pixel array
        let clamped_x = x.min(self.width - 1);
        let clamped_y = y.min(self.height - 1);

        let index = (clamped_y * self.width + clamped_x) * 4;

        // Assuming the pixel buffer is laid out as RGBA
        Color::new(
            self.pixels[index],     // Red
            self.pixels[index + 1], // Green
            self.pixels[index + 2], // Blue
            self.pixels[index + 3], // Alpha
        )
    }}

pub struct TextureManager {
    textures: HashMap<u8, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, filename: &str) -> u8 {
        // Validate filename is tga
        if !filename.ends_with(".tga") {
            panic!("Invalid file type. Must be .tga");
        }

        let image = image::open(filename).unwrap().to_rgba8();

        let texture = Texture {
            width: image.width() as usize,
            height: image.height() as usize,
            pixels: image.into_raw(),
        };

        let texture_id: u8 = 0;
        self.textures.insert(texture_id, texture);

        texture_id
    }

    pub fn get_texture(&self, texture_id: u8) -> &Texture {
        self.textures.get(&texture_id).unwrap()
    }
}
