extern crate raylib_sys as ffi;

pub mod audio;
/// Custom raylib color palette for amazing visuals
pub mod colors;
pub mod core;
pub mod models;
pub mod shaders;
pub mod shapes;
pub mod text;
pub mod textures;

/// Color type, RGBA (32bit)
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    fn into_ffi(self) -> ffi::Color {
        ffi::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}
