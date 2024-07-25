use crate::framebuffer::FrameBuffer;
use image::{ImageBuffer, RgbImage};

pub fn save_framebuffer_to_bmp(framebuffer: &FrameBuffer, file_name: &str) {
    let mut img: RgbImage = ImageBuffer::new(framebuffer.width, framebuffer.height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = &framebuffer.data[(x + y * framebuffer.width) as usize];
        *pixel = image::Rgb([color.r, color.g, color.b]);
    }

    img.save(file_name).unwrap();
}