mod framebuffer;
mod line;
mod polygon;
mod filling;
mod render;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::Polygon;
use line::line;
use render::render;
use std::thread;
use std::time::Duration;

fn main() {

    let window_width: i32 = 800;
    let window_height: i32 = 600;
    
    let framebuffer_width = 800;
    let framebuffer_height: u32 = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Polygon Drawing")
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::BLACK);
    
    let mut translate_x: f32 = 0.0;
    let mut translate_y: f32 = 0.0;

    while !window.window_should_close() {
        translate_x += 1.0;
        translate_y += 1.0;

        framebuffer.clear();


        render(&mut framebuffer, translate_x, translate_y);

        framebuffer.swap_buffers(&mut window, &thread);

        thread::sleep(Duration::from_millis(16));

    }
}
