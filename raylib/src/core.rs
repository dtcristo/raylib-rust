use std::ffi::CString;

use {ffi, Color, Image};

//------------------------------------------------------------------------------
// Window-related functions
//------------------------------------------------------------------------------

/// Initialize window and OpenGL context
pub fn init_window(width: i32, height: i32, title: &str) {
    let c_title = CString::new(title).unwrap();
    unsafe { ffi::InitWindow(width, height, c_title.as_ptr()) }
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
/// Set icon for window (only PLATFORM_DESKTOP)
pub fn set_window_icon(image: Image) {
    unsafe { ffi::SetWindowIcon(image.into_ffi()) }
}
/// Set title for window (only PLATFORM_DESKTOP)
pub fn set_window_title(title: &str) {
    let c_title = CString::new(title).unwrap();
    unsafe { ffi::SetWindowTitle(c_title.as_ptr()) }
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
/// Setup canvas (framebuffer) to start drawing
pub fn begin_drawing() {
    unsafe { ffi::BeginDrawing() }
}
/// End canvas drawing and swap buffers (double buffering)
pub fn end_drawing() {
    unsafe { ffi::EndDrawing() }
}
// /// Initialize 2D mode with custom camera (2D)
// pub fn begin_mode_2d(camera: Camera2D) {
//     unsafe { ffi::BeginMode2D(camera) }
// }
/// Ends 2D mode with custom camera
pub fn end_mode_2d() {
    unsafe { ffi::EndMode2D() }
}
// /// Initializes 3D mode with custom camera (3D)
// pub fn begin_mode_3d(camera: Camera3D) {
//     unsafe { ffi::BeginMode3D(camera) }
// }
/// Ends 3D mode and returns to default 2D orthographic mode
pub fn end_mode_3d() {
    unsafe { ffi::EndMode3D() }
}
// /// Initializes render texture for drawing
// pub fn begin_texture_mode(target: RenderTexture2D) {
//     unsafe { ffi::BeginTextureMode(target) }
// }
/// Ends drawing to render texture
pub fn end_texture_mode() {
    unsafe { ffi::EndTextureMode() }
}

//------------------------------------------------------------------------------
// Screen-space-related functions
//------------------------------------------------------------------------------

// /// Returns a ray trace from mouse position
// pub fn get_mouse_ray(mouse_position: Vector2, camera: Camera) -> Ray {
//     unsafe { ffi::GetMouseRay(mouse_position, camera) }
// }
// /// Returns the screen space position for a 3d world space position
// pub fn get_world_to_screen(position: Vector3, camera: Camera) -> Vector2 {
//     unsafe { ffi::GetWorldToScreen(position, camera) }
// }
// /// Returns camera transform matrix (view matrix)
// pub fn get_camera_matrix(camera: Camera) -> Matrix {
//     unsafe { ffi::GetCameraMatrix(camera) }
// }

//------------------------------------------------------------------------------
// Timming-related functions
//------------------------------------------------------------------------------

/// Set target FPS (maximum)
pub fn set_target_fps(fps: i32) {
    unsafe { ffi::SetTargetFPS(fps) }
}
/// Returns current FPS
pub fn get_fps() -> i32 {
    unsafe { ffi::GetFPS() }
}
/// Returns time in seconds for last frame drawn
pub fn get_frame_time() -> f32 {
    unsafe { ffi::GetFrameTime() }
}
/// Returns elapsed time in seconds since [`init_window`]: #method.init_window
pub fn get_time() -> f64 {
    unsafe { ffi::GetTime() }
}
