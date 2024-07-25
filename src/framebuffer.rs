use crate::color::Color;

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let data = vec![Color::new(0, 0, 0); (width * height) as usize];
        FrameBuffer { width, height, data }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: &Color) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let index = (x + y * self.width as i32) as usize;
            self.data[index] = *color;
        }
    }
}