extern crate raylib_sys as ffi;

use std::os::raw::{c_int, c_void};

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
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
    // TODO: Remove pub from this function
    #[doc(hidden)]
    pub fn into_ffi(self) -> ffi::Color {
        ffi::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

/// Pixel formats
///
/// NOTE: Support depends on OpenGL version and platform
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// 8 bit per pixel (no alpha)
    UncompressedGrayscale,
    /// 8*2 bpp (2 channels)
    UncompressedGrayAlpha,
    /// 16 bpp
    UncompressedR5G6B5,
    /// 24 bpp
    UncompressedR8g8b8,
    /// 16 bpp (1 bit alpha)
    UncompressedR5g5b5a1,
    /// 16 bpp (4 bit alpha)
    UncompressedR4g4b4a4,
    /// 32 bpp
    UncompressedR8g8b8a8,
    /// 32 bpp (1 channel - float)
    UncompressedR32,
    /// 32*3 bpp (3 channels - float)
    UncompressedR32g32b32,
    /// 32*4 bpp (4 channels - float)
    UncompressedR32g32b32a32,
    /// 4 bpp (no alpha)
    CompressedDxt1Rgb,
    /// 4 bpp (1 bit alpha)
    CompressedDxt1Rgba,
    /// 8 bpp
    CompressedDxt3Rgba,
    /// 8 bpp
    CompressedDxt5Rgba,
    /// 4 bpp
    CompressedEtc1Rgb,
    /// 4 bpp
    CompressedEtc2Rgb,
    /// 8 bpp
    CompressedEtc2EacRgba,
    /// 4 bpp
    CompressedPvrtRgb,
    /// 4 bpp
    CompressedPvrtRgba,
    /// 8 bpp
    CompressedAstc4x4Rgba,
    /// 2 bpp
    CompressedAstc8x8Rgba,
}
impl PixelFormat {
    fn from_ffi(ffi: c_int) -> PixelFormat {
        match ffi as u32 {
            ffi::PixelFormat::UNCOMPRESSED_GRAYSCALE => PixelFormat::UncompressedGrayscale,
            ffi::PixelFormat::UNCOMPRESSED_GRAY_ALPHA => PixelFormat::UncompressedGrayAlpha,
            ffi::PixelFormat::UNCOMPRESSED_R5G6B5 => PixelFormat::UncompressedR5G6B5,
            ffi::PixelFormat::UNCOMPRESSED_R8G8B8 => PixelFormat::UncompressedR8g8b8,
            ffi::PixelFormat::UNCOMPRESSED_R5G5B5A1 => PixelFormat::UncompressedR5g5b5a1,
            ffi::PixelFormat::UNCOMPRESSED_R4G4B4A4 => PixelFormat::UncompressedR4g4b4a4,
            ffi::PixelFormat::UNCOMPRESSED_R8G8B8A8 => PixelFormat::UncompressedR8g8b8a8,
            ffi::PixelFormat::UNCOMPRESSED_R32 => PixelFormat::UncompressedR32,
            ffi::PixelFormat::UNCOMPRESSED_R32G32B32 => PixelFormat::UncompressedR32g32b32,
            ffi::PixelFormat::UNCOMPRESSED_R32G32B32A32 => PixelFormat::UncompressedR32g32b32a32,
            ffi::PixelFormat::COMPRESSED_DXT1_RGB => PixelFormat::CompressedDxt1Rgb,
            ffi::PixelFormat::COMPRESSED_DXT1_RGBA => PixelFormat::CompressedDxt1Rgba,
            ffi::PixelFormat::COMPRESSED_DXT3_RGBA => PixelFormat::CompressedDxt3Rgba,
            ffi::PixelFormat::COMPRESSED_DXT5_RGBA => PixelFormat::CompressedDxt5Rgba,
            ffi::PixelFormat::COMPRESSED_ETC1_RGB => PixelFormat::CompressedEtc1Rgb,
            ffi::PixelFormat::COMPRESSED_ETC2_RGB => PixelFormat::CompressedEtc2Rgb,
            ffi::PixelFormat::COMPRESSED_ETC2_EAC_RGBA => PixelFormat::CompressedEtc2EacRgba,
            ffi::PixelFormat::COMPRESSED_PVRT_RGB => PixelFormat::CompressedPvrtRgb,
            ffi::PixelFormat::COMPRESSED_PVRT_RGBA => PixelFormat::CompressedPvrtRgba,
            ffi::PixelFormat::COMPRESSED_ASTC_4x4_RGBA => PixelFormat::CompressedAstc4x4Rgba,
            ffi::PixelFormat::COMPRESSED_ASTC_8x8_RGBA => PixelFormat::CompressedAstc8x8Rgba,
            _ => panic!("Invalid PixelFormat value `{}`.", ffi),
        }
    }
    fn into_ffi(self) -> c_int {
        (match self {
            PixelFormat::UncompressedGrayscale => ffi::PixelFormat::UNCOMPRESSED_GRAYSCALE,
            PixelFormat::UncompressedGrayAlpha => ffi::PixelFormat::UNCOMPRESSED_GRAY_ALPHA,
            PixelFormat::UncompressedR5G6B5 => ffi::PixelFormat::UNCOMPRESSED_R5G6B5,
            PixelFormat::UncompressedR8g8b8 => ffi::PixelFormat::UNCOMPRESSED_R8G8B8,
            PixelFormat::UncompressedR5g5b5a1 => ffi::PixelFormat::UNCOMPRESSED_R5G5B5A1,
            PixelFormat::UncompressedR4g4b4a4 => ffi::PixelFormat::UNCOMPRESSED_R4G4B4A4,
            PixelFormat::UncompressedR8g8b8a8 => ffi::PixelFormat::UNCOMPRESSED_R8G8B8A8,
            PixelFormat::UncompressedR32 => ffi::PixelFormat::UNCOMPRESSED_R32,
            PixelFormat::UncompressedR32g32b32 => ffi::PixelFormat::UNCOMPRESSED_R32G32B32,
            PixelFormat::UncompressedR32g32b32a32 => ffi::PixelFormat::UNCOMPRESSED_R32G32B32A32,
            PixelFormat::CompressedDxt1Rgb => ffi::PixelFormat::COMPRESSED_DXT1_RGB,
            PixelFormat::CompressedDxt1Rgba => ffi::PixelFormat::COMPRESSED_DXT1_RGBA,
            PixelFormat::CompressedDxt3Rgba => ffi::PixelFormat::COMPRESSED_DXT3_RGBA,
            PixelFormat::CompressedDxt5Rgba => ffi::PixelFormat::COMPRESSED_DXT5_RGBA,
            PixelFormat::CompressedEtc1Rgb => ffi::PixelFormat::COMPRESSED_ETC1_RGB,
            PixelFormat::CompressedEtc2Rgb => ffi::PixelFormat::COMPRESSED_ETC2_RGB,
            PixelFormat::CompressedEtc2EacRgba => ffi::PixelFormat::COMPRESSED_ETC2_EAC_RGBA,
            PixelFormat::CompressedPvrtRgb => ffi::PixelFormat::COMPRESSED_PVRT_RGB,
            PixelFormat::CompressedPvrtRgba => ffi::PixelFormat::COMPRESSED_PVRT_RGBA,
            PixelFormat::CompressedAstc4x4Rgba => ffi::PixelFormat::COMPRESSED_ASTC_4x4_RGBA,
            PixelFormat::CompressedAstc8x8Rgba => ffi::PixelFormat::COMPRESSED_ASTC_8x8_RGBA,
        } as c_int)
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
    fn from_ffi(ffi: ffi::Image) -> Image {
        Image {
            data: ffi.data,
            width: ffi.width,
            height: ffi.height,
            mipmaps: ffi.mipmaps,
            format: PixelFormat::from_ffi(ffi.format),
        }
    }
    fn into_ffi(self) -> ffi::Image {
        ffi::Image {
            data: self.data,
            width: self.width,
            height: self.height,
            mipmaps: self.mipmaps,
            format: self.format.into_ffi(),
        }
    }
}
