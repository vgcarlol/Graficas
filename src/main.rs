mod color;
mod framebuffer;
mod bmp;
mod line;

use framebuffer::FrameBuffer;
use color::Color;

fn main() {
    let mut framebuffer = FrameBuffer::new(800, 600);
    let yellow = Color::new(255, 255, 0);
    let white = Color::new(255, 255, 255);

    let points = vec![
        (165, 380), (185, 360), (180, 330), (207, 345),
        (233, 330), (230, 360), (250, 380), (220, 385),
        (205, 410), (193, 383)
    ];

    // Rellenar el polígono primero
    fill_polygon(&points, &yellow, &mut framebuffer);

    // Dibujar las orillas del polígono después
    for i in 0..points.len() {
        let next_index = (i + 1) % points.len();
        line::draw_line(points[i].0, points[i].1, points[next_index].0, points[next_index].1, &white, &mut framebuffer);
    }

    bmp::save_framebuffer_to_bmp(&framebuffer, "out.bmp");
}



fn fill_polygon(points: &[(i32, i32)], color: &Color, framebuffer: &mut FrameBuffer) {
    if points.len() < 3 { return; }

    // Estructura para almacenar los bordes activos
    let mut edges: Vec<(i32, i32, f64, f64)> = Vec::new();  // (x_min, y_max, x, dx)

    // Construir la lista de bordes, ignorando los bordes horizontales
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let (mut x1, mut y1, mut x2, mut y2) = (points[i].0, points[i].1, points[j].0, points[j].1);

        if y1 != y2 {
            if y1 > y2 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }
            let dx = (x2 as f64 - x1 as f64) / (y2 as f64 - y1 as f64);
            edges.push((y1, y2, x1 as f64, dx));
        }
    }

    // Ordenar los bordes por y_min y luego por x para bordes con el mismo y_min
    edges.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.2.partial_cmp(&b.2).unwrap()));

    // Rellenar el polígono
    let mut active_edges: Vec<(i32, f64, f64)> = Vec::new();  // (y_max, x, dx)
    let mut y = edges.first().map(|e| e.0).unwrap_or(0);

    while !edges.is_empty() || !active_edges.is_empty() {
        // Añadir nuevas aristas activas que empiezan en la línea actual
        while let Some(edge) = edges.first() {
            if edge.0 > y { break; }
            active_edges.push((edge.1, edge.2, edge.3));
            edges.remove(0);
        }

        // Eliminar aristas activas que ya no son válidas
        active_edges.retain(|e| e.0 > y);

        // Ordenar aristas activas por x
        active_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Rellenar entre pares de aristas activas
        for pair in active_edges.chunks(2) {
            if pair.len() < 2 { continue; }
            let start_x = pair[0].1.round() as i32;
            let end_x = pair[1].1.round() as i32;
            for x in start_x..=end_x {
                framebuffer.set_pixel(x, y, color);
            }
        }

        // Avanzar a la siguiente línea
        y += 1;

        // Actualizar x para aristas activas
        for edge in active_edges.iter_mut() {
            edge.1 += edge.2;
        }
    }
}
