use std::ffi::CString;

// Window-related functions

/// Initialize window and OpenGL context
pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe { ::ffi::InitWindow(width, height, CString::new(title).unwrap().as_ptr()) }
}

/// Close window and unload OpenGL context
pub fn close_window() {
    unsafe { ::ffi::CloseWindow() }
}

/// Check if window has been initialized successfully
pub fn is_window_ready() -> bool {
    unsafe { ::ffi::IsWindowReady() == ::ffi::bool__true_ }
}

/// Check if KEY_ESCAPE pressed or Close icon pressed
pub fn whindow_should_close() -> bool {
    unsafe { ::ffi::WindowShouldClose() == ::ffi::bool__true_ }
}

/// Check if window has been minimized (or lost focus)
pub fn is_window_minimized() -> bool {
    unsafe { ::ffi::IsWindowMinimized() == ::ffi::bool__true_ }
}

/// Toggle fullscreen mode (only PLATFORM_DESKTOP)
pub fn toggle_fullscreen() {
    unsafe { ::ffi::ToggleFullscreen() }
}

// /// Set icon for window (only PLATFORM_DESKTOP)
// pub fn set_window_icon(image: Image) {
//     unsafe { ::ffi::SetWindowIcon(image) }
// }

/// Set title for window (only PLATFORM_DESKTOP)
pub fn set_window_title(title: &str) {
    unsafe { ::ffi::SetWindowTitle(CString::new(title).unwrap().as_ptr()) }
}

/// Set window position on screen (only PLATFORM_DESKTOP)
pub fn set_window_position(x: i32, y: i32) {
    unsafe { ::ffi::SetWindowPosition(x, y) }
}

/// Set monitor for the current window (fullscreen mode)
pub fn set_window_monitor(monitor: i32) {
    unsafe { ::ffi::SetWindowMonitor(monitor) }
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_min_size(width: i32, height: i32) {
    unsafe { ::ffi::SetWindowMinSize(width, height) }
}

/// Set window dimensions
pub fn set_window_size(width: i32, height: i32) {
    unsafe { ::ffi::SetWindowSize(width, height) }
}

/// Get current screen width
pub fn get_screen_width() -> i32 {
    unsafe { ::ffi::GetScreenWidth() }
}

/// Get current screen height
pub fn get_screen_height() -> i32 {
    unsafe { ::ffi::GetScreenHeight() }
}
