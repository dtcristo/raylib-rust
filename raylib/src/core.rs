//! Window, Graphics Device and Input Handling Functions

use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_uchar};
use std::slice;

use {raw, BitFlags, Color, ConfigFlag, Image, LogType};

//------------------------------------------------------------------------------
// Window-related functions
//------------------------------------------------------------------------------

/// Initialize window and OpenGL context
pub fn init_window<S: Into<String>>(width: i32, height: i32, title: S) {
    let raw_title = CString::new(title.into()).unwrap();
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
pub fn set_window_title<S: Into<String>>(title: S) {
    let raw_title = CString::new(title.into()).unwrap();
    unsafe { raw::SetWindowTitle(raw_title.as_ptr()) }
}
/// Set window position on screen (only PLATFORM_DESKTOP)
pub fn set_window_position(x: i32, y: i32) {
    unsafe { raw::SetWindowPosition(x, y) }
}
/// Set monitor for the current window (for [`ConfigFlags::FullscreenMode`](../enum.ConfigFlag.html#variant.FullscreenMode))
pub fn set_window_monitor(monitor: i32) {
    unsafe { raw::SetWindowMonitor(monitor) }
}
/// Set window minimum dimensions (for [`ConfigFlags::WindowResizable`](../enum.ConfigFlag.html#variant.WindowResizable))
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
// /// Returns the screen space position for a 3D world space position
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
/// Setup window configuration flags
pub fn set_config_flags(flags: BitFlags<ConfigFlag>) {
    let raw_flags = flags.bits() as c_uchar;
    unsafe { raw::SetConfigFlags(raw_flags) }
}
/// Enable trace log message types
pub fn set_trace_log(log_types: BitFlags<LogType>) {
    let raw_log_types = log_types.bits() as c_uchar;
    unsafe { raw::SetTraceLog(raw_log_types) }
}
// TODO: Allow this to accept multiple text messages at once
/// Show trace log message
pub fn trace_log<S: Into<String>>(log_type: LogType, text: S) {
    let raw_log_type = log_type as c_int;
    let raw_text = CString::new(text.into()).unwrap();
    unsafe { raw::TraceLog(raw_log_type, raw_text.as_ptr()) }
}
/// Takes a screenshot of current screen (saved a .png)
pub fn take_screenshot<S: Into<String>>(file_name: S) {
    let raw_file_name = CString::new(file_name.into()).unwrap();
    unsafe { raw::TakeScreenshot(raw_file_name.as_ptr()) }
}
/// Returns a random value between min and max (both included)
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { raw::GetRandomValue(min, max) }
}

#[cfg(test)]
mod tests {
    use super::get_random_value;

    use super::get_extension;

    #[test]
    fn call_random_function() {
        let result = get_random_value(0, 10);
        assert!(result >= 0);
        assert!(result <= 10);
    }

    #[test]
    fn test_get_extension() {
        let result = get_extension("README.md");
        assert_eq!("md", result);
    }
}

//------------------------------------------------------------------------------
// Files management functions
//------------------------------------------------------------------------------

/// Check file extension
pub fn is_file_extension<S: Into<String>, T: Into<String>>(file_name: S, ext: T) -> bool {
    let raw_file_name = CString::new(file_name.into()).unwrap();
    let raw_ext = CString::new(ext.into()).unwrap();
    unsafe { raw::IsFileExtension(raw_file_name.as_ptr(), raw_ext.as_ptr()) == raw::bool_::true_ }
}

/// Get extension for a file name string
pub fn get_extension<S: Into<String>>(file_name: S) -> String {
    let raw_file_name = CString::new(file_name.into()).unwrap();
    let raw_ext = unsafe { CStr::from_ptr(raw::GetExtension(raw_file_name.as_ptr())) };
    raw_ext.to_str().unwrap().to_string()
}

/// Get file name for a path string
pub fn get_file_name<S: Into<String>>(file_path: S) -> String {
    let raw_file_path = CString::new(file_path.into()).unwrap();
    let raw_file_name = unsafe { CStr::from_ptr(raw::GetFileName(raw_file_path.as_ptr())) };
    raw_file_name.to_str().unwrap().to_string()
}

/// Get directory for a given path string
pub fn get_directory_path<S: Into<String>>(file_path: S) -> String {
    let raw_file_path = CString::new(file_path.into()).unwrap();
    let raw_directory_path =
        unsafe { CStr::from_ptr(raw::GetDirectoryPath(raw_file_path.as_ptr())) };
    raw_directory_path.to_str().unwrap().to_string()
}

/// Get current working directory
pub fn get_working_directory() -> String {
    let raw_working_directory = unsafe { CStr::from_ptr(raw::GetWorkingDirectory()) };
    raw_working_directory.to_str().unwrap().to_string()
}

/// Change working directory
pub fn change_directory<S: Into<String>>(dir: S) -> Result<(), ()> {
    let raw_dir = CString::new(dir.into()).unwrap();
    let result = unsafe { raw::ChangeDirectory(raw_dir.as_ptr()) == raw::bool_::true_ };
    match result {
        true => Ok(()),
        false => Err(()),
    }
}

/// Check if a file has been dropped into window
pub fn is_file_dropped() -> bool {
    unsafe { raw::IsFileDropped() == raw::bool_::true_ }
}

/// Get dropped files' paths
pub fn get_dropped_files() -> Vec<String> {
    let mut count: c_int = 0;
    let raw_paths_array_ptr = unsafe { raw::GetDroppedFiles(&mut count) };
    if raw_paths_array_ptr.is_null() {
        return vec![];
    }
    let raw_paths_slice = unsafe { slice::from_raw_parts(raw_paths_array_ptr, count as usize) };
    raw_paths_slice
        .iter()
        .map(|raw_str_ptr| {
            let raw_path = unsafe { CStr::from_ptr(*raw_str_ptr) };
            raw_path.to_str().unwrap().to_string()
        })
        .collect()
}

/// Clear dropped files' paths buffer
pub fn clear_dropped_files() {
    unsafe { raw::ClearDroppedFiles() }
}
