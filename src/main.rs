mod color;
mod framebuffer;
mod bmp;
mod line;

use framebuffer::FrameBuffer;
use color::Color;

fn main() {
    let mut framebuffer = FrameBuffer::new(800, 600);
    let green = Color::new(0, 255, 0); // Color verde para el relleno del polígono 4
    let red = Color::new(255, 0, 0);  // Color rojo para el relleno del polígono 3
    let blue = Color::new(0, 0, 255);  // Color azul para el relleno del polígono 2 
    let yellow = Color::new(255, 255, 0); // Color amarillo para el relleno del polígono 1
    let white = Color::new(255, 255, 255); // Color para las orillas

    // Polígono 1
    let points = vec![
        (165, 380), (185, 360), (180, 330), (207, 345),
        (233, 330), (230, 360), (250, 380), (220, 385),
        (205, 410), (193, 383)
    ];

    // Polígono 2
    let points_poly2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    // Polígono 3
    let points_poly3 = vec![
        (377, 249), (411, 197), (436, 249)
    ];

    // Polígono 4
    let points_poly4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    // Polígono 5
    let points_poly5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    // Rellenar el polígono 1
    fill_polygon(&points, &yellow, &mut framebuffer, None);

    // Rellenar y dibujar el polígono 2
    fill_polygon(&points_poly2, &blue, &mut framebuffer, None);
    draw_edges(&points_poly2, &white, &mut framebuffer);

    // Rellenar y dibujar el polígono 3
    fill_polygon(&points_poly3, &red, &mut framebuffer, None);
    draw_edges(&points_poly3, &white, &mut framebuffer);

    // Rellenar y dibujar el polígono 4
    fill_polygon(&points_poly4, &green, &mut framebuffer, Some(&points_poly5));
    draw_edges(&points_poly4, &white, &mut framebuffer);

    // Dibujar orilla del polígono 5
    draw_edges(&points_poly5, &white, &mut framebuffer);

    // Dibujar las orillas del polígono después
    for i in 0..points.len() {
        let next_index = (i + 1) % points.len();
        line::draw_line(points[i].0, points[i].1, points[next_index].0, points[next_index].1, &white, &mut framebuffer);
    }

    bmp::save_framebuffer_to_bmp(&framebuffer, "out.bmp");
}



fn fill_polygon(points: &[(i32, i32)], color: &Color, framebuffer: &mut FrameBuffer, exclude: Option<&[(i32, i32)]>) {
    if points.len() < 3 { return; }

    // Estructura para almacenar los bordes activos
    let mut edges: Vec<(i32, i32, f64, f64)> = Vec::new();  // (y_min, y_max, x_start, dx)

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

        // Rellenar entre pares de aristas activas, excluyendo el polígono del agujero
        for pair in active_edges.chunks(2) {
            if pair.len() < 2 { continue; }
            let start_x = pair[0].1.round() as i32;
            let end_x = pair[1].1.round() as i32;
            for x in start_x..=end_x {
                if exclude.map_or(true, |exclude_points| !is_point_in_polygon(x, y, exclude_points)) {
                    framebuffer.set_pixel(x, y, color);
                }
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

fn is_point_in_polygon(x: i32, y: i32, polygon: &[(i32, i32)]) -> bool {
    // Implementación simple del algoritmo de ray-casting para determinar si un punto está dentro de un polígono
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        let intersect = ((yi > y) != (yj > y))
            && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}



fn draw_edges(points: &[(i32, i32)], color: &Color, framebuffer: &mut FrameBuffer) {
    for i in 0..points.len() {
        let next_index = (i + 1) % points.len();
        line::draw_line(points[i].0, points[i].1, points[next_index].0, points[next_index].1, color, framebuffer);
    }
}