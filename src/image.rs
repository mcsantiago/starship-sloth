use pixels::Pixels;

#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
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
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![0; (width * height * 4) as usize];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let index = (x + y * self.width) as usize;
        self.pixels[index * 4] = color.r;
        self.pixels[index * 4 + 1] = color.g;
        self.pixels[index * 4 + 2] = color.b;
        self.pixels[index * 4 + 3] = color.a;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let index = (x + y * self.width) as usize;
        Color {
            r: self.pixels[index * 4],
            g: self.pixels[index * 4 + 1],
            b: self.pixels[index * 4 + 2],
            a: self.pixels[index * 4 + 3],
        }
    }

    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.set_pixel(i, j, color);
            }
        }
    }

    pub fn save(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let color = self.get_pixel(x, y);
            *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
        }
        imgbuf.save(filename).unwrap();
    }

    pub fn write_to_buffer(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i as u32 % self.width;
            let y = i as u32 / self.width;
            let color = self.get_pixel(x, y);
            pixel.copy_from_slice(&[color.r, color.g, color.b, color.a]);
        }
    }
}
