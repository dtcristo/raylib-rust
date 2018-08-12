extern crate raylib_sys;

use std::ffi::CString;

fn main() {
    let screen_width = 800;
    let screen_height = 450;
    let ray_white = raylib_sys::Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };
    let rust_orange = raylib_sys::Color {
        r: 222,
        g: 165,
        b: 132,
        a: 255,
    };
    unsafe {
        raylib_sys::InitWindow(
            screen_width,
            screen_height,
            CString::new("raylib-rust logo").unwrap().as_ptr(),
        );
        raylib_sys::SetTargetFPS(60);
        while raylib_sys::WindowShouldClose() == 0 {
            raylib_sys::BeginDrawing();
            raylib_sys::ClearBackground(ray_white);
            raylib_sys::DrawRectangle(
                screen_width / 2 - 128,
                screen_height / 2 - 128,
                256,
                256,
                rust_orange,
            );
            raylib_sys::DrawRectangle(
                screen_width / 2 - 112,
                screen_height / 2 - 112,
                224,
                224,
                ray_white,
            );
            raylib_sys::DrawText(
                CString::new("rust").unwrap().as_ptr(),
                screen_width / 2 - 69,
                screen_height / 2 + 18,
                50,
                rust_orange,
            );
            raylib_sys::DrawText(
                CString::new("raylib").unwrap().as_ptr(),
                screen_width / 2 - 44,
                screen_height / 2 + 48,
                50,
                rust_orange,
            );
            raylib_sys::EndDrawing();
        }
        raylib_sys::CloseWindow();
    }
}
