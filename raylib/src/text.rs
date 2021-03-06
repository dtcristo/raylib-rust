//! Font Loading and Text Drawing Functions

use std::ffi::CString;

use crate::{raw, Color};

//------------------------------------------------------------------------------
// Text drawing functions
//------------------------------------------------------------------------------

/// Draw text (using default font)
pub fn draw_text<S: Into<String>>(text: S, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let raw_text = CString::new(text.into()).unwrap();
    let raw_color = color.into_raw();
    unsafe { raw::DrawText(raw_text.as_ptr(), pos_x, pos_y, font_size, raw_color) }
}
