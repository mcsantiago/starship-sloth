use num_traits::Float;
use std::ops::{Add, Sub};

macro_rules! impl_vec2_ops {
    ($Vec2Type:ident, $T:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $Vec2Type {
            pub x: $T,
            pub y: $T,
        }

        impl $Vec2Type {
            pub fn new(x: $T, y: $T) -> Self {
                Self { x, y }
            }
        }

        impl std::ops::Add for $Vec2Type {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
        impl std::ops::Sub for $Vec2Type {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }
    };
}

impl_vec2_ops!(Vec2f, f32);
impl_vec2_ops!(Vec2d, f64);
impl_vec2_ops!(Vec2i, i32);
impl_vec2_ops!(Vec2u, u32);

pub trait PixelPoint {
    fn to_i32_tuple(&self) -> (i32, i32);
}

impl PixelPoint for (i32, i32) {
    fn to_i32_tuple(&self) -> (i32, i32) {
        *self
    }
}

impl PixelPoint for Vec2f {
    fn to_i32_tuple(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

impl PixelPoint for Vec2i {
    fn to_i32_tuple(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

trait Cross {
    fn cross(&self, other: &Self) -> Self;
}

macro_rules! impl_vec3_ops {
    ($Vec3Type:ident, $T:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $Vec3Type {
            pub x: $T,
            pub y: $T,
            pub z: $T,
        }

        impl $Vec3Type {
            pub fn new(x: $T, y: $T, z: $T) -> Self {
                Self { x, y, z }
            }

            pub fn cross(&self, other: Self) -> Self {
                Self {
                    x: self.y * other.z - self.z * other.y,
                    y: self.z * other.x - self.x * other.z,
                    z: self.x * other.y - self.y * other.x,
                }
            }

            pub fn dot(&self, other: &Self) -> f32 {
                (self.x * other.x + self.y * other.y + self.z * other.z) as f32
            }

            pub fn length(&self) -> f32 {
                self.dot(self).sqrt()
            }

            pub fn normalize(&self) -> Self {
                let len = (self.length()) as $T;
                Self {
                    x: self.x / len,
                    y: self.y / len,
                    z: self.z / len,
                }
            }
        }

        impl std::ops::Add for $Vec3Type {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }
        impl std::ops::Sub for $Vec3Type {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }
    };
}

impl_vec3_ops!(Vec3f, f32);
impl_vec3_ops!(Vec3d, f64);
impl_vec3_ops!(Vec3i, i32);

pub fn compute_line_parameters<P: PixelPoint + Copy>(p0: P, p1: P) -> Option<(f32, f32)> {
    let p0_tuple = p0.to_i32_tuple();
    let p1_tuple = p1.to_i32_tuple();

    let dx = p1_tuple.0 - p0_tuple.0;
    let dy = p1_tuple.1 - p0_tuple.1;

    if dx == 0 {
        None
    } else {
        let gradient = dy as f32 / dx as f32;
        let intercept = p0_tuple.1 as f32 - gradient * p0_tuple.0 as f32;
        Some((gradient, intercept))
    }
}
