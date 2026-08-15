//! Shared furniture marks for the responsive game cabinet layouts.

use macroquad::prelude::*;

pub fn draw_shelves(x: f32, y: f32, width: f32, height: f32, accent: Color) {
    let shelf_gap = height / 2.;
    for shelf in 0..=2 {
        let shelf_y = y + shelf as f32 * shelf_gap;
        let rail = Color::new(accent.r, accent.g, accent.b, 0.32);
        draw_line(x, shelf_y, x + width, shelf_y, 3., rail);
        draw_circle(x + 8., shelf_y, 3., accent);
        draw_circle(x + width - 8., shelf_y, 3., accent);
    }
}

pub fn draw_header_motif(x: f32, y: f32, radius: f32, accent: Color) {
    draw_circle_lines(
        x,
        y,
        radius,
        2.,
        Color::new(accent.r, accent.g, accent.b, 0.55),
    );
    draw_circle_lines(
        x,
        y,
        radius * 0.45,
        1.,
        Color::new(accent.r, accent.g, accent.b, 0.75),
    );
    draw_circle(x, y, 3., accent);
}
