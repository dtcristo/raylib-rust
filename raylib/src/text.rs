use std::ffi::CString;

use {raw, Color};

//------------------------------------------------------------------------------
// Text drawing functions
//------------------------------------------------------------------------------

/// Draw text (using default font)
pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).unwrap();
    unsafe { raw::DrawText(c_text.as_ptr(), pos_x, pos_y, font_size, color.into_raw()) }
}
