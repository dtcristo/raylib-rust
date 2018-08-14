use std::ffi::CString;
use std::os::raw::{c_int, c_uchar};

use {raw, Color, ConfigFlags, Image, LogType};

//------------------------------------------------------------------------------
// Window-related functions
//------------------------------------------------------------------------------

/// Initialize window and OpenGL context
pub fn init_window(width: i32, height: i32, title: &str) {
    let raw_title = CString::new(title).unwrap();
    unsafe { raw::InitWindow(width, height, raw_title.as_ptr()) }
}
/// Close window and unload OpenGL context
pub fn close_window() {
    unsafe { raw::CloseWindow() }
}
/// Check if window has been initialized successfully
pub fn is_window_ready() -> bool {
    unsafe { raw::IsWindowReady() == raw::bool_::true_ }
}
/// Check if KEY_ESCAPE pressed or Close icon pressed
pub fn window_should_close() -> bool {
    unsafe { raw::WindowShouldClose() == raw::bool_::true_ }
}
/// Check if window has been minimized (or lost focus)
pub fn is_window_minimized() -> bool {
    unsafe { raw::IsWindowMinimized() == raw::bool_::true_ }
}
/// Toggle fullscreen mode (only PLATFORM_DESKTOP)
pub fn toggle_fullscreen() {
    unsafe { raw::ToggleFullscreen() }
}
/// Set icon for window (only PLATFORM_DESKTOP)
pub fn set_window_icon(image: Image) {
    let raw_image = image.into_raw();
    unsafe { raw::SetWindowIcon(raw_image) }
}
/// Set title for window (only PLATFORM_DESKTOP)
pub fn set_window_title(title: &str) {
    let raw_title = CString::new(title).unwrap();
    unsafe { raw::SetWindowTitle(raw_title.as_ptr()) }
}
/// Set window position on screen (only PLATFORM_DESKTOP)
pub fn set_window_position(x: i32, y: i32) {
    unsafe { raw::SetWindowPosition(x, y) }
}
/// Set monitor for the current window (fullscreen mode)
pub fn set_window_monitor(monitor: i32) {
    unsafe { raw::SetWindowMonitor(monitor) }
}
/// Set window minimum dimensions (for [`ConfigFlags::WINDOW_RESIZABLE`](../struct.ConfigFlags.html#associatedconstant.WINDOW_RESIZABLE))
pub fn set_window_min_size(width: i32, height: i32) {
    unsafe { raw::SetWindowMinSize(width, height) }
}
/// Set window dimensions
pub fn set_window_size(width: i32, height: i32) {
    unsafe { raw::SetWindowSize(width, height) }
}
/// Get current screen width
pub fn get_screen_width() -> i32 {
    unsafe { raw::GetScreenWidth() }
}
/// Get current screen height
pub fn get_screen_height() -> i32 {
    unsafe { raw::GetScreenHeight() }
}

//------------------------------------------------------------------------------
// Cursor-related functions
//------------------------------------------------------------------------------

/// Shows cursor
pub fn show_cursor() {
    unsafe { raw::ShowCursor() }
}
/// Hides cursor
pub fn hide_cursor() {
    unsafe { raw::HideCursor() }
}
/// Check if cursor is not visible
pub fn is_cursor_hidden() -> bool {
    unsafe { raw::IsCursorHidden() == raw::bool_::true_ }
}
/// Enables cursor (unlock cursor)
pub fn enable_cursor() {
    unsafe { raw::EnableCursor() }
}
/// Disables cursor (lock cursor)
pub fn disable_cursor() {
    unsafe { raw::DisableCursor() }
}

//------------------------------------------------------------------------------
// Drawing-related functions
//------------------------------------------------------------------------------

/// Set background color (framebuffer clear color)
pub fn clear_background(color: Color) {
    let raw_color = color.into_raw();
    unsafe { raw::ClearBackground(raw_color) }
}
/// Setup canvas (framebuffer) to start drawing
pub fn begin_drawing() {
    unsafe { raw::BeginDrawing() }
}
/// End canvas drawing and swap buffers (double buffering)
pub fn end_drawing() {
    unsafe { raw::EndDrawing() }
}
// /// Initialize 2D mode with custom camera (2D)
// pub fn begin_mode_2d(camera: Camera2D) {
//     unsafe { raw::BeginMode2D(camera) }
// }
/// Ends 2D mode with custom camera
pub fn end_mode_2d() {
    unsafe { raw::EndMode2D() }
}
// /// Initializes 3D mode with custom camera (3D)
// pub fn begin_mode_3d(camera: Camera3D) {
//     unsafe { raw::BeginMode3D(camera) }
// }
/// Ends 3D mode and returns to default 2D orthographic mode
pub fn end_mode_3d() {
    unsafe { raw::EndMode3D() }
}
// /// Initializes render texture for drawing
// pub fn begin_texture_mode(target: RenderTexture2D) {
//     unsafe { raw::BeginTextureMode(target) }
// }
/// Ends drawing to render texture
pub fn end_texture_mode() {
    unsafe { raw::EndTextureMode() }
}

//------------------------------------------------------------------------------
// Screen-space-related functions
//------------------------------------------------------------------------------

// /// Returns a ray trace from mouse position
// pub fn get_mouse_ray(mouse_position: Vector2, camera: Camera) -> Ray {
//     unsafe { raw::GetMouseRay(mouse_position, camera) }
// }
// /// Returns the screen space position for a 3d world space position
// pub fn get_world_to_screen(position: Vector3, camera: Camera) -> Vector2 {
//     unsafe { raw::GetWorldToScreen(position, camera) }
// }
// /// Returns camera transform matrix (view matrix)
// pub fn get_camera_matrix(camera: Camera) -> Matrix {
//     unsafe { raw::GetCameraMatrix(camera) }
// }

//------------------------------------------------------------------------------
// Timming-related functions
//------------------------------------------------------------------------------

/// Set target FPS (maximum)
pub fn set_target_fps(fps: i32) {
    unsafe { raw::SetTargetFPS(fps) }
}
/// Returns current FPS
pub fn get_fps() -> i32 {
    unsafe { raw::GetFPS() }
}
/// Returns time in seconds for last frame drawn
pub fn get_frame_time() -> f32 {
    unsafe { raw::GetFrameTime() }
}
/// Returns elapsed time in seconds since [`init_window`](fn.init_window.html)
pub fn get_time() -> f64 {
    unsafe { raw::GetTime() }
}

//------------------------------------------------------------------------------
// Misc. functions
//------------------------------------------------------------------------------

/// Activate raylib logo at startup (can be done with flags)
pub fn show_logo() {
    unsafe { raw::ShowLogo() }
}
/// Setup window configuration flags (view [`ConfigFlags`](../struct.ConfigFlags.html))
pub fn set_config_flags(flags: ConfigFlags) {
    let raw_flags = flags.bits() as c_uchar;
    unsafe { raw::SetConfigFlags(raw_flags) }
}
/// Enable trace log message types (view [`LogType`](../struct.LogType.html))
pub fn set_trace_log(log_types: LogType) {
    let raw_log_types = log_types.bits() as c_uchar;
    unsafe { raw::SetTraceLog(raw_log_types) }
}
/// Show trace log messages (one of [`LogType`](../struct.LogType.html))
pub fn trace_log(log_type: LogType, text: &str) {
    let raw_log_type = log_type.bits() as c_int;
    let raw_text = CString::new(text).unwrap();
    unsafe { raw::TraceLog(raw_log_type, raw_text.as_ptr()) }
}
/// Takes a screenshot of current screen (saved a .png)
pub fn take_screenshot(file_name: &str) {
    let raw_file_name = CString::new(file_name).unwrap();
    unsafe { raw::TakeScreenshot(raw_file_name.as_ptr()) }
}
/// Returns a random value between min and max (both included)
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { raw::GetRandomValue(min, max) }
}
