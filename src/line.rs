use crate::framebuffer::FrameBuffer;
use crate::color::Color;

pub fn draw_line(mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: &Color, framebuffer: &mut FrameBuffer) {
    let dx = (x1 - x0).abs();
    let dy = -((y1 - y0).abs());
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        framebuffer.set_pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { 
            err += dy; 
            x0 += sx; 
        }
        if e2 <= dx { 
            err += dx; 
            y0 += sy; 
        }
    }
}