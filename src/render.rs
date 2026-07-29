use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn render(
    framebuffer: &mut Framebuffer,
    translate_x: f32,
    translate_y: f32,
) {
    framebuffer.set_current_color(Color::WHITE);
    line(
        framebuffer,
        Vector2::new(100.0 + translate_x, 100.0 + translate_y), 
        Vector2::new(200.0 + translate_x, 200.0 + translate_y)
    );

    framebuffer.set_current_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(200.0 + translate_x, 100.0 + translate_y), 
        Vector2::new(100.0 + translate_x, 200.0 + translate_y)
    );  

}

