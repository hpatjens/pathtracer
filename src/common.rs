use std;

pub use hmath::*;

pub const PI: f32 = std::f32::consts::PI;

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;

pub type Vec2u = Vector2<u32>;

#[derive(Clone, Debug, new)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

// Source: https://de.wikipedia.org/wiki/Xorshift
static mut X32: u32 = 314159265;
pub fn xorshift32() -> u32 {
    unsafe { 
        X32 ^= X32 << 13;
        X32 ^= X32 >> 17;
        X32 ^= X32 << 5;
        X32
    }
}

pub fn random32() -> f32 {
    let r = xorshift32();
    r as f32 / std::u32::MAX as f32
}

pub fn clampf32(min: f32, max: f32, x: f32) -> f32 {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}

pub fn saturatef32(x: f32) -> f32 {
    clampf32(0.0, 1.0, x)
}

#[derive(Clone)]
pub struct Pixel(pub u8, pub u8, pub u8);

impl Pixel {
    pub fn from_unit(color: Vec3) -> Self {
        Pixel((color.x*255.0) as u8, (color.y*255.0) as u8, (color.z*255.0) as u8)
    }
}