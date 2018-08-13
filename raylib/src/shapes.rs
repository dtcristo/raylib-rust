use {raw, Color};

//------------------------------------------------------------------------------
// Basic shapes drawing functions
//------------------------------------------------------------------------------

/// Draw a color-filled rectangle
pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    let raw_color = color.into_raw();
    unsafe { raw::DrawRectangle(pos_x, pos_y, width, height, raw_color) }
}
