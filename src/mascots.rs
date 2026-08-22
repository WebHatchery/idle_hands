//! Small, touch-safe animal marks for the cabinet's six game categories.
//!
//! These are intentionally drawn as simple vector sprites instead of loaded
//! textures. They stay crisp at each cabinet scale, add no asset-loading
//! failure mode, and remain purely decorative: the category cards keep their
//! existing full-card hit targets.

use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum Mascot {
    Fox,
    Owl,
    Beaver,
    Rabbit,
    Squirrel,
    Raccoon,
}

pub fn draw_for_filter(filter: u8, center: Vec2, scale: f32) {
    let Some(mascot) = mascot_for_filter(filter) else {
        return;
    };
    let scale = scale.max(0.1);
    let accent = mascot_accent(mascot);

    draw_ellipse(
        center.x,
        center.y + 18. * scale,
        16. * scale,
        4. * scale,
        0.,
        Color::new(0.05, 0.035, 0.025, 0.28),
    );
    draw_circle(
        center.x,
        center.y,
        24. * scale,
        Color::new(0.94, 0.87, 0.72, 0.12),
    );
    draw_circle_lines(
        center.x,
        center.y,
        23. * scale,
        1.3 * scale,
        Color::new(accent.r, accent.g, accent.b, 0.42),
    );

    match mascot {
        Mascot::Fox => draw_fox(center, scale),
        Mascot::Owl => draw_owl(center, scale),
        Mascot::Beaver => draw_beaver(center, scale),
        Mascot::Rabbit => draw_rabbit(center, scale),
        Mascot::Squirrel => draw_squirrel(center, scale),
        Mascot::Raccoon => draw_raccoon(center, scale),
    }
}

fn mascot_for_filter(filter: u8) -> Option<Mascot> {
    match filter {
        3 => Some(Mascot::Fox),
        4 => Some(Mascot::Owl),
        5 => Some(Mascot::Beaver),
        6 => Some(Mascot::Rabbit),
        7 => Some(Mascot::Squirrel),
        8 => Some(Mascot::Raccoon),
        _ => None,
    }
}

fn mascot_accent(mascot: Mascot) -> Color {
    match mascot {
        Mascot::Fox => crate::theme::LEATHER,
        Mascot::Owl => crate::theme::SLATE_BRONZE,
        Mascot::Beaver => crate::theme::WALNUT,
        Mascot::Rabbit => crate::theme::PAPER_LIGHT,
        Mascot::Squirrel => crate::theme::BRASS,
        Mascot::Raccoon => crate::theme::SURFACE_DARK,
    }
}

fn draw_fox(center: Vec2, scale: f32) {
    let fur = crate::theme::LEATHER;
    let ear = crate::theme::BRASS;
    let cream = crate::theme::PAPER_LIGHT;
    let ink = crate::theme::INK;
    let s = scale;

    draw_triangle(
        center + vec2(-12., -7.) * s,
        center + vec2(-7., -20.) * s,
        center + vec2(-1., -9.) * s,
        fur,
    );
    draw_triangle(
        center + vec2(12., -7.) * s,
        center + vec2(7., -20.) * s,
        center + vec2(1., -9.) * s,
        fur,
    );
    draw_triangle(
        center + vec2(-9., -10.) * s,
        center + vec2(-7., -16.) * s,
        center + vec2(-4., -11.) * s,
        ear,
    );
    draw_triangle(
        center + vec2(9., -10.) * s,
        center + vec2(7., -16.) * s,
        center + vec2(4., -11.) * s,
        ear,
    );
    draw_ellipse(center.x, center.y + 10. * s, 11. * s, 12. * s, 0., fur);
    draw_circle(center.x, center.y - 3. * s, 13. * s, fur);
    draw_ellipse(center.x, center.y + 3. * s, 10. * s, 7. * s, 0., cream);
    draw_circle(center.x - 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_circle(center.x + 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_triangle(
        center + vec2(-2.5, 1.) * s,
        center + vec2(2.5, 1.) * s,
        center + vec2(0., 4.) * s,
        ink,
    );
    draw_line(
        center.x,
        center.y + 4. * s,
        center.x,
        center.y + 7. * s,
        1. * s,
        ink,
    );
}

fn draw_owl(center: Vec2, scale: f32) {
    let body = crate::theme::SLATE_BRONZE;
    let wing = crate::theme::SURFACE_DARK;
    let eye = crate::theme::PAPER_LIGHT;
    let ink = crate::theme::INK;
    let s = scale;

    draw_ellipse(center.x, center.y + 7. * s, 13. * s, 16. * s, 0., body);
    draw_ellipse(
        center.x - 10. * s,
        center.y + 7. * s,
        6. * s,
        11. * s,
        -0.28,
        wing,
    );
    draw_ellipse(
        center.x + 10. * s,
        center.y + 7. * s,
        6. * s,
        11. * s,
        0.28,
        wing,
    );
    draw_triangle(
        center + vec2(-11., -10.) * s,
        center + vec2(-8., -20.) * s,
        center + vec2(-2., -12.) * s,
        body,
    );
    draw_triangle(
        center + vec2(11., -10.) * s,
        center + vec2(8., -20.) * s,
        center + vec2(2., -12.) * s,
        body,
    );
    draw_circle(center.x, center.y - 4. * s, 13. * s, body);
    for eye_x in [-5., 5.] {
        draw_circle(center.x + eye_x * s, center.y - 5. * s, 6. * s, eye);
        draw_circle(center.x + eye_x * s, center.y - 5. * s, 2. * s, ink);
    }
    draw_triangle(
        center + vec2(-3., 1.) * s,
        center + vec2(3., 1.) * s,
        center + vec2(0., 6.) * s,
        crate::theme::BRASS,
    );
}

fn draw_beaver(center: Vec2, scale: f32) {
    let fur = crate::theme::WALNUT;
    let light = crate::theme::PARCHMENT_BROWN;
    let cream = crate::theme::PAPER_LIGHT;
    let ink = crate::theme::INK;
    let s = scale;

    draw_ellipse(
        center.x + 15. * s,
        center.y + 8. * s,
        9. * s,
        14. * s,
        -0.45,
        light,
    );
    draw_ellipse(center.x, center.y + 10. * s, 12. * s, 13. * s, 0., fur);
    draw_circle(center.x - 10. * s, center.y - 9. * s, 5. * s, fur);
    draw_circle(center.x + 10. * s, center.y - 9. * s, 5. * s, fur);
    draw_circle(center.x, center.y - 3. * s, 14. * s, fur);
    draw_circle(center.x - 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_circle(center.x + 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_ellipse(center.x, center.y + 4. * s, 8. * s, 6. * s, 0., light);
    draw_circle(center.x, center.y + 2. * s, 2. * s, ink);
    draw_rectangle(center.x - 5. * s, center.y + 5. * s, 4. * s, 6. * s, cream);
    draw_rectangle(center.x + 1. * s, center.y + 5. * s, 4. * s, 6. * s, cream);
}

fn draw_rabbit(center: Vec2, scale: f32) {
    let fur = crate::theme::PAPER_LIGHT;
    let inner = crate::theme::PARCHMENT_BROWN;
    let blush = crate::theme::LEATHER;
    let ink = crate::theme::INK;
    let s = scale;

    for (x, tilt) in [(-6., -0.12), (6., 0.12)] {
        draw_ellipse(
            center.x + x * s,
            center.y - 16. * s,
            5. * s,
            14. * s,
            tilt,
            fur,
        );
        draw_ellipse(
            center.x + x * s,
            center.y - 16. * s,
            2. * s,
            10. * s,
            tilt,
            inner,
        );
    }
    draw_ellipse(center.x, center.y + 10. * s, 11. * s, 12. * s, 0., fur);
    draw_circle(center.x, center.y - 3. * s, 13. * s, fur);
    draw_circle(center.x - 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_circle(center.x + 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_triangle(
        center + vec2(-2.5, 1.) * s,
        center + vec2(2.5, 1.) * s,
        center + vec2(0., 4.) * s,
        blush,
    );
    draw_circle(center.x - 8. * s, center.y + 1. * s, 2. * s, blush);
    draw_circle(center.x + 8. * s, center.y + 1. * s, 2. * s, blush);
}

fn draw_squirrel(center: Vec2, scale: f32) {
    let fur = crate::theme::WALNUT;
    let light = crate::theme::BRASS;
    let cream = crate::theme::PAPER_LIGHT;
    let ink = crate::theme::INK;
    let s = scale;

    draw_ellipse(
        center.x + 13. * s,
        center.y - 3. * s,
        11. * s,
        18. * s,
        0.55,
        fur,
    );
    draw_ellipse(center.x, center.y + 10. * s, 10. * s, 13. * s, 0., fur);
    draw_circle(center.x - 8. * s, center.y - 10. * s, 5. * s, fur);
    draw_circle(center.x + 8. * s, center.y - 10. * s, 5. * s, fur);
    draw_circle(center.x, center.y - 3. * s, 13. * s, fur);
    draw_circle(center.x - 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_circle(center.x + 5. * s, center.y - 5. * s, 1.7 * s, ink);
    draw_ellipse(center.x, center.y + 3. * s, 9. * s, 7. * s, 0., cream);
    draw_circle(center.x + 1. * s, center.y + 1. * s, 2. * s, ink);
    draw_circle(center.x + 10. * s, center.y + 12. * s, 4. * s, light);
    draw_circle_lines(
        center.x + 10. * s,
        center.y + 12. * s,
        4. * s,
        1. * s,
        crate::theme::PARCHMENT_BROWN,
    );
}

fn draw_raccoon(center: Vec2, scale: f32) {
    let fur = crate::theme::SURFACE_DARK;
    let mask = crate::theme::SLATE_BRONZE;
    let cream = crate::theme::PAPER_LIGHT;
    let ink = crate::theme::INK;
    let s = scale;

    draw_ellipse(
        center.x + 14. * s,
        center.y + 7. * s,
        10. * s,
        5. * s,
        0.28,
        fur,
    );
    draw_ellipse(center.x, center.y + 10. * s, 12. * s, 13. * s, 0., fur);
    draw_triangle(
        center + vec2(-11., -8.) * s,
        center + vec2(-8., -19.) * s,
        center + vec2(-2., -11.) * s,
        fur,
    );
    draw_triangle(
        center + vec2(11., -8.) * s,
        center + vec2(8., -19.) * s,
        center + vec2(2., -11.) * s,
        fur,
    );
    draw_circle(center.x, center.y - 3. * s, 14. * s, fur);
    draw_ellipse(center.x, center.y - 4. * s, 13. * s, 6. * s, 0., mask);
    draw_circle(center.x - 5. * s, center.y - 4. * s, 2. * s, cream);
    draw_circle(center.x + 5. * s, center.y - 4. * s, 2. * s, cream);
    draw_circle(center.x - 5. * s, center.y - 4. * s, 1. * s, ink);
    draw_circle(center.x + 5. * s, center.y - 4. * s, 1. * s, ink);
    draw_circle(center.x, center.y + 3. * s, 2. * s, ink);
}
