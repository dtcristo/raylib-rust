extern crate raylib_sys as raw;

use std::os::raw::{c_int, c_void};

//------------------------------------------------------------------------------
// Modules
//------------------------------------------------------------------------------

/// Audio Loading and Playing Functions
pub mod audio;
/// Camera System Functions
pub mod camera;
/// Custom raylib color palette for amazing visuals
pub mod colors;
/// Window, Graphics Device and Input Handling Functions
pub mod core;
/// Gestures and Touch Handling Functions
pub mod gestures;
/// Basic 3D Shape and 3D Model Loading and Drawing Functions
pub mod models;
/// Shaders System Functions
///
/// NOTE: This functions are useless when using OpenGL 1.1
pub mod shaders;
/// Basic Shape Drawing Functions
pub mod shapes;
/// Font Loading and Text Drawing Functions
pub mod text;
/// Texture Loading and Drawing Functions
pub mod textures;

//------------------------------------------------------------------------------
// Constants
//------------------------------------------------------------------------------

pub const PI: f64 = raw::PI;
pub const DEG2RAD: f64 = raw::DEG2RAD;
pub const RAD2DEG: f64 = raw::RAD2DEG;

//------------------------------------------------------------------------------
// Structs
//------------------------------------------------------------------------------

/// Vector2 type
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }
    fn from_raw(raw: raw::Vector2) -> Vector2 {
        Vector2 { x: raw.x, y: raw.y }
    }
    fn into_raw(self) -> raw::Vector2 {
        raw::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

pub type Vector3 = raw::Vector3;
pub type Vector4 = raw::Vector4;

/// Color type/functions, RGBA (32bit)
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
    fn from_raw(raw: raw::Color) -> Color {
        Color {
            r: raw.r,
            g: raw.g,
            b: raw.b,
            a: raw.a,
        }
    }
    fn into_raw(self) -> raw::Color {
        raw::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
    /// Returns hexadecimal value for a Color
    pub fn to_int(self) -> u32 {
        let raw = self.into_raw();
        unsafe { raw::ColorToInt(raw) as u32 }
    }
    /// Returns color normalized as float [0..1]
    pub fn normalize(self) -> Vector4 {
        let raw = self.into_raw();
        unsafe { raw::ColorNormalize(raw) }
    }
    /// Returns HSV values for a Color
    pub fn to_hsv(self) -> Vector3 {
        let raw = self.into_raw();
        unsafe { raw::ColorToHSV(raw) }
    }
    /// Returns a Color struct from hexadecimal value
    pub fn from_int(value: u32) -> Color {
        Color::from_raw(unsafe { raw::GetColor(value as c_int) })
    }
    /// Color fade-in or fade-out, alpha goes from 0.0f to 1.0f
    pub fn fade(self, alpha: f32) -> Color {
        let raw = self.into_raw();
        Color::from_raw(unsafe { raw::Fade(raw, alpha) })
    }
}

/// Image type, bpp always RGBA (32bit)
///
/// NOTE: Data stored in CPU memory (RAM)
#[derive(Debug, Clone)]
pub struct Image {
    /// Image raw data
    data: *mut c_void,
    /// Image base width
    pub width: i32,
    /// Image base height
    pub height: i32,
    /// Mipmap levels, 1 by default
    pub mipmaps: i32,
    /// Data format (PixelFormat type)
    pub format: PixelFormat,
}
impl Image {
    fn from_raw(raw: raw::Image) -> Image {
        Image {
            data: raw.data,
            width: raw.width,
            height: raw.height,
            mipmaps: raw.mipmaps,
            format: PixelFormat::from_raw(raw.format),
        }
    }
    fn into_raw(self) -> raw::Image {
        raw::Image {
            data: self.data,
            width: self.width,
            height: self.height,
            mipmaps: self.mipmaps,
            format: self.format.into_raw(),
        }
    }
}

//------------------------------------------------------------------------------
// Enums
//------------------------------------------------------------------------------

/// Pixel formats
///
/// NOTE: Support depends on OpenGL version and platform
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// 8 bit per pixel (no alpha)
    UncompressedGrayscale = raw::PixelFormat::UNCOMPRESSED_GRAYSCALE,
    /// 8*2 bpp (2 channels)
    UncompressedGrayAlpha = raw::PixelFormat::UNCOMPRESSED_GRAY_ALPHA,
    /// 16 bpp
    UncompressedR5G6B5 = raw::PixelFormat::UNCOMPRESSED_R5G6B5,
    /// 24 bpp
    UncompressedR8g8b8 = raw::PixelFormat::UNCOMPRESSED_R8G8B8,
    /// 16 bpp (1 bit alpha)
    UncompressedR5g5b5a1 = raw::PixelFormat::UNCOMPRESSED_R5G5B5A1,
    /// 16 bpp (4 bit alpha)
    UncompressedR4g4b4a4 = raw::PixelFormat::UNCOMPRESSED_R4G4B4A4,
    /// 32 bpp
    UncompressedR8g8b8a8 = raw::PixelFormat::UNCOMPRESSED_R8G8B8A8,
    /// 32 bpp (1 channel - float)
    UncompressedR32 = raw::PixelFormat::UNCOMPRESSED_R32,
    /// 32*3 bpp (3 channels - float)
    UncompressedR32g32b32 = raw::PixelFormat::UNCOMPRESSED_R32G32B32,
    /// 32*4 bpp (4 channels - float)
    UncompressedR32g32b32a32 = raw::PixelFormat::UNCOMPRESSED_R32G32B32A32,
    /// 4 bpp (no alpha)
    CompressedDxt1Rgb = raw::PixelFormat::COMPRESSED_DXT1_RGB,
    /// 4 bpp (1 bit alpha)
    CompressedDxt1Rgba = raw::PixelFormat::COMPRESSED_DXT1_RGBA,
    /// 8 bpp
    CompressedDxt3Rgba = raw::PixelFormat::COMPRESSED_DXT3_RGBA,
    /// 8 bpp
    CompressedDxt5Rgba = raw::PixelFormat::COMPRESSED_DXT5_RGBA,
    /// 4 bpp
    CompressedEtc1Rgb = raw::PixelFormat::COMPRESSED_ETC1_RGB,
    /// 4 bpp
    CompressedEtc2Rgb = raw::PixelFormat::COMPRESSED_ETC2_RGB,
    /// 8 bpp
    CompressedEtc2EacRgba = raw::PixelFormat::COMPRESSED_ETC2_EAC_RGBA,
    /// 4 bpp
    CompressedPvrtRgb = raw::PixelFormat::COMPRESSED_PVRT_RGB,
    /// 4 bpp
    CompressedPvrtRgba = raw::PixelFormat::COMPRESSED_PVRT_RGBA,
    /// 8 bpp
    CompressedAstc4x4Rgba = raw::PixelFormat::COMPRESSED_ASTC_4x4_RGBA,
    /// 2 bpp
    CompressedAstc8x8Rgba = raw::PixelFormat::COMPRESSED_ASTC_8x8_RGBA,
}
impl PixelFormat {
    fn from_raw(raw: c_int) -> PixelFormat {
        match raw as u32 {
            raw::PixelFormat::UNCOMPRESSED_GRAYSCALE => PixelFormat::UncompressedGrayscale,
            raw::PixelFormat::UNCOMPRESSED_GRAY_ALPHA => PixelFormat::UncompressedGrayAlpha,
            raw::PixelFormat::UNCOMPRESSED_R5G6B5 => PixelFormat::UncompressedR5G6B5,
            raw::PixelFormat::UNCOMPRESSED_R8G8B8 => PixelFormat::UncompressedR8g8b8,
            raw::PixelFormat::UNCOMPRESSED_R5G5B5A1 => PixelFormat::UncompressedR5g5b5a1,
            raw::PixelFormat::UNCOMPRESSED_R4G4B4A4 => PixelFormat::UncompressedR4g4b4a4,
            raw::PixelFormat::UNCOMPRESSED_R8G8B8A8 => PixelFormat::UncompressedR8g8b8a8,
            raw::PixelFormat::UNCOMPRESSED_R32 => PixelFormat::UncompressedR32,
            raw::PixelFormat::UNCOMPRESSED_R32G32B32 => PixelFormat::UncompressedR32g32b32,
            raw::PixelFormat::UNCOMPRESSED_R32G32B32A32 => PixelFormat::UncompressedR32g32b32a32,
            raw::PixelFormat::COMPRESSED_DXT1_RGB => PixelFormat::CompressedDxt1Rgb,
            raw::PixelFormat::COMPRESSED_DXT1_RGBA => PixelFormat::CompressedDxt1Rgba,
            raw::PixelFormat::COMPRESSED_DXT3_RGBA => PixelFormat::CompressedDxt3Rgba,
            raw::PixelFormat::COMPRESSED_DXT5_RGBA => PixelFormat::CompressedDxt5Rgba,
            raw::PixelFormat::COMPRESSED_ETC1_RGB => PixelFormat::CompressedEtc1Rgb,
            raw::PixelFormat::COMPRESSED_ETC2_RGB => PixelFormat::CompressedEtc2Rgb,
            raw::PixelFormat::COMPRESSED_ETC2_EAC_RGBA => PixelFormat::CompressedEtc2EacRgba,
            raw::PixelFormat::COMPRESSED_PVRT_RGB => PixelFormat::CompressedPvrtRgb,
            raw::PixelFormat::COMPRESSED_PVRT_RGBA => PixelFormat::CompressedPvrtRgba,
            raw::PixelFormat::COMPRESSED_ASTC_4x4_RGBA => PixelFormat::CompressedAstc4x4Rgba,
            raw::PixelFormat::COMPRESSED_ASTC_8x8_RGBA => PixelFormat::CompressedAstc8x8Rgba,
            _ => panic!("Invalid PixelFormat value `{}`.", raw),
        }
    }
    fn into_raw(self) -> c_int {
        self as c_int
    }
}
