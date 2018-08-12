use std::ffi::CString;

use {ffi, Color};

//------------------------------------------------------------------------------
// Window-related functions
//------------------------------------------------------------------------------

/// Initialize window and OpenGL context
pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe { ffi::InitWindow(width, height, CString::new(title).unwrap().as_ptr()) }
}
/// Close window and unload OpenGL context
pub fn close_window() {
    unsafe { ffi::CloseWindow() }
}
/// Check if window has been initialized successfully
pub fn is_window_ready() -> bool {
    unsafe { ffi::IsWindowReady() == ffi::bool_::true_ }
}
/// Check if KEY_ESCAPE pressed or Close icon pressed
pub fn window_should_close() -> bool {
    unsafe { ffi::WindowShouldClose() == ffi::bool_::true_ }
}
/// Check if window has been minimized (or lost focus)
pub fn is_window_minimized() -> bool {
    unsafe { ffi::IsWindowMinimized() == ffi::bool_::true_ }
}
/// Toggle fullscreen mode (only PLATFORM_DESKTOP)
pub fn toggle_fullscreen() {
    unsafe { ffi::ToggleFullscreen() }
}
// /// Set icon for window (only PLATFORM_DESKTOP)
// pub fn set_window_icon(image: Image) {
//     unsafe { ffi::SetWindowIcon(image) }
// }
/// Set title for window (only PLATFORM_DESKTOP)
pub fn set_window_title(title: &str) {
    unsafe { ffi::SetWindowTitle(CString::new(title).unwrap().as_ptr()) }
}
/// Set window position on screen (only PLATFORM_DESKTOP)
pub fn set_window_position(x: i32, y: i32) {
    unsafe { ffi::SetWindowPosition(x, y) }
}
/// Set monitor for the current window (fullscreen mode)
pub fn set_window_monitor(monitor: i32) {
    unsafe { ffi::SetWindowMonitor(monitor) }
}
/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_min_size(width: i32, height: i32) {
    unsafe { ffi::SetWindowMinSize(width, height) }
}
/// Set window dimensions
pub fn set_window_size(width: i32, height: i32) {
    unsafe { ffi::SetWindowSize(width, height) }
}
/// Get current screen width
pub fn get_screen_width() -> i32 {
    unsafe { ffi::GetScreenWidth() }
}
/// Get current screen height
pub fn get_screen_height() -> i32 {
    unsafe { ffi::GetScreenHeight() }
}

//------------------------------------------------------------------------------
// Cursor-related functions
//------------------------------------------------------------------------------

/// Shows cursor
pub fn show_cursor() {
    unsafe { ffi::ShowCursor() }
}
/// Hides cursor
pub fn hide_cursor() {
    unsafe { ffi::HideCursor() }
}
/// Check if cursor is not visible
pub fn is_cursor_hidden() -> bool {
    unsafe { ffi::IsCursorHidden() == ffi::bool_::true_ }
}
/// Enables cursor (unlock cursor)
pub fn enable_cursor() {
    unsafe { ffi::EnableCursor() }
}
/// Disables cursor (lock cursor)
pub fn disable_cursor() {
    unsafe { ffi::DisableCursor() }
}

//------------------------------------------------------------------------------
// Drawing-related functions
//------------------------------------------------------------------------------

/// Set background color (framebuffer clear color)
pub fn clear_background(color: Color) {
    unsafe { ffi::ClearBackground(color.into_ffi()) }
}
