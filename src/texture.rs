use std::collections::HashMap;

use crate::image::Color;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = (y * self.width + x) * 4;
        Color::new(
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
            self.pixels[index + 3],
        )
    }
}

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
