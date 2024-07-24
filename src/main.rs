extern crate nalgebra_glm as glm;

use glm::{Vec2, vec2};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Polígonos
    let polygons = vec![
        vec![
            vec2(165.0, 380.0), vec2(185.0, 360.0), vec2(180.0, 330.0),
            vec2(207.0, 345.0), vec2(233.0, 330.0), vec2(230.0, 360.0),
            vec2(250.0, 380.0), vec2(220.0, 385.0), vec2(205.0, 410.0),
            vec2(193.0, 383.0),
        ],
        vec![
            vec2(321.0, 335.0), vec2(288.0, 286.0), vec2(339.0, 251.0), vec2(374.0, 302.0),
        ],
        vec![
            vec2(377.0, 249.0), vec2(411.0, 197.0), vec2(436.0, 249.0),
        ],
        vec![
            vec2(413.0, 177.0), vec2(448.0, 159.0), vec2(502.0, 88.0), vec2(553.0, 53.0),
            vec2(535.0, 36.0), vec2(676.0, 37.0), vec2(660.0, 52.0), vec2(750.0, 145.0),
            vec2(761.0, 179.0), vec2(672.0, 192.0), vec2(659.0, 214.0), vec2(615.0, 214.0),
            vec2(632.0, 230.0), vec2(580.0, 230.0), vec2(597.0, 215.0), vec2(552.0, 214.0),
            vec2(517.0, 144.0), vec2(466.0, 180.0),
        ],
        vec![
            vec2(682.0, 175.0), vec2(708.0, 120.0), vec2(735.0, 148.0), vec2(739.0, 170.0),
        ],
    ];

    // Crea un image buffer
    let width = 800;
    let height = 600;
    let mut buffer = vec![vec![0; width]; height];

    for polygon in polygons {
        scanline_fill(&polygon, &mut buffer);
    }

    // Buffer a BMP
    save_to_bmp(&buffer, "out.bmp").expect("Fallo en Guardar el BMP");
}

fn scanline_fill(polygon: &Vec<Vec2>, buffer: &mut Vec<Vec<i32>>) {
    let min_y = polygon.iter().map(|v| v.y as usize).min().unwrap();
    let max_y = polygon.iter().map(|v| v.y as usize).max().unwrap();

    for y in min_y..=max_y {
        // Encuentra intersecciones
        let mut intersections = vec![];
        for i in 0..polygon.len() {
            let p1 = &polygon[i];
            let p2 = &polygon[(i + 1) % polygon.len()];
            if (p1.y as usize <= y && p2.y as usize > y) || (p2.y as usize <= y && p1.y as usize > y) {
                let x = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                intersections.push(x as usize);
            }
        }
        intersections.sort();

        // Rellenar entre intersecciones
        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                for x in pair[0]..=pair[1] {
                    buffer[y][x] = 1;
                }
            }
        }
    }
}

fn save_to_bmp(buffer: &Vec<Vec<i32>>, filename: &str) -> std::io::Result<()> {
    let width = buffer[0].len() as u32;
    let height = buffer.len() as u32;
    let mut file = File::create(filename)?;
    file.write_all(b"BM")?;
    file.write_all(&((54 + 3 * width * height) as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&54u32.to_le_bytes())?;
    file.write_all(&40u32.to_le_bytes())?;
    file.write_all(&width.to_le_bytes())?;
    file.write_all(&height.to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&24u16.to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&((3 * width * height) as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&[0; 4])?;
    file.write_all(&[0; 4])?;
    file.write_all(&[0; 4])?;
    for row in buffer.iter().rev() {
        for &pixel in row.iter() {
            let color = if pixel == 0 { [0u8, 0u8, 0u8] } else { [255u8, 255u8, 255u8] };
            file.write_all(&color)?;
        }
    }
    Ok(())
}
