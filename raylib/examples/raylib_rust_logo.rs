extern crate raylib;

use raylib::{colors, core, shapes, text, Color};

fn main() {
    let w = 800;
    let h = 450;
    let rust_orange = Color::new(222, 165, 132, 255);
    core::init_window(w, h, "raylib-rust logo");
    core::set_target_fps(60);
    while !core::window_should_close() {
        core::begin_drawing();
        core::clear_background(colors::RAYWHITE);
        shapes::draw_rectangle(w / 2 - 128, h / 2 - 128, 256, 256, rust_orange);
        shapes::draw_rectangle(w / 2 - 112, h / 2 - 112, 224, 224, colors::RAYWHITE);
        text::draw_text("rust", w / 2 - 69, h / 2 + 18, 50, rust_orange);
        text::draw_text("raylib", w / 2 - 44, h / 2 + 48, 50, rust_orange);
        core::end_drawing();
    }
    core::close_window();
}
