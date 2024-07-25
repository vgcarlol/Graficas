use crate::framebuffer::FrameBuffer;
use std::fs::File;
use std::io::{self, Write, BufWriter};

pub fn save_framebuffer_to_bmp(framebuffer: &FrameBuffer, file_name: &str) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(file_name)?);

    let file_size = 14 + 40 + (framebuffer.width * framebuffer.height * 3) as usize; // 14-byte file header + 40-byte DIB header + pixel data
    let pixel_data_offset = 14 + 40; // File header + DIB header size

    // BMP File Header (14 bytes)
    file.write_all(b"BM")?; // Signature
    file.write_all(&(file_size as u32).to_le_bytes())?; // File size
    file.write_all(&0u32.to_le_bytes())?; // Reserved
    file.write_all(&(pixel_data_offset as u32).to_le_bytes())?; // Pixel data offset

    // DIB Header (40 bytes)
    file.write_all(&40u32.to_le_bytes())?; // DIB header size
    file.write_all(&(framebuffer.width as u32).to_le_bytes())?; // Width
    file.write_all(&(framebuffer.height as u32).to_le_bytes())?; // Height
    file.write_all(&1u16.to_le_bytes())?; // Planes
    file.write_all(&24u16.to_le_bytes())?; // Bits per pixel
    file.write_all(&0u32.to_le_bytes())?; // Compression (none)
    file.write_all(&0u32.to_le_bytes())?; // Image size (no compression)
    file.write_all(&0u32.to_le_bytes())?; // X pixels per meter
    file.write_all(&0u32.to_le_bytes())?; // Y pixels per meter
    file.write_all(&0u32.to_le_bytes())?; // Total colors
    file.write_all(&0u32.to_le_bytes())?; // Important colors

    // Pixel Data
    for y in (0..framebuffer.height) { // BMP stores pixels bottom-to-top
        for x in 0..framebuffer.width {
            let idx = (x + y * framebuffer.width) as usize;
            let color = &framebuffer.data[idx];
            file.write_all(&[color.b, color.g, color.r])?; // Pixels in BGR format
        }
    }

    file.flush()?;
    Ok(())
}
