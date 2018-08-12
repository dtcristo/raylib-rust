extern crate raylib;
extern crate raylib_sys;

use std::ffi::CString;

fn main() {
    let screen_width = 800;
    let screen_height = 450;
    let rust_orange = raylib::Color::new(222, 165, 132, 255);
    raylib::core::init_window(screen_width, screen_height, "raylib-rust logo");
    unsafe { raylib_sys::SetTargetFPS(60) }
    while !raylib::core::window_should_close() {
        unsafe { raylib_sys::BeginDrawing() };
        raylib::core::clear_background(raylib::colors::RAYWHITE);
        unsafe {
            raylib_sys::DrawRectangle(
                screen_width / 2 - 128,
                screen_height / 2 - 128,
                256,
                256,
                rust_orange.into_ffi(),
            )
        }
        unsafe {
            raylib_sys::DrawRectangle(
                screen_width / 2 - 112,
                screen_height / 2 - 112,
                224,
                224,
                raylib::colors::RAYWHITE.into_ffi(),
            )
        }
        unsafe {
            raylib_sys::DrawText(
                CString::new("rust").unwrap().as_ptr(),
                screen_width / 2 - 69,
                screen_height / 2 + 18,
                50,
                rust_orange.into_ffi(),
            )
        }
        unsafe {
            raylib_sys::DrawText(
                CString::new("raylib").unwrap().as_ptr(),
                screen_width / 2 - 44,
                screen_height / 2 + 48,
                50,
                rust_orange.into_ffi(),
            )
        }
        unsafe { raylib_sys::EndDrawing() }
    }
    raylib::core::close_window();
}
