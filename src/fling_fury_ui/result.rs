//! Resume-safe result overlays for completed and exhausted Fling Fury forts.

use crate::{
    fling_fury::{FlingFury, FlingStatus},
    theme,
};
use macroquad::prelude::*;

pub(super) fn draw(board: Rect, game: &FlingFury) {
    let won = game.status == FlingStatus::Won;
    draw_rectangle(
        board.x,
        board.y,
        board.w,
        board.h,
        Color::new(0.03, 0.05, 0.08, 0.72),
    );
    let portrait = crate::ui::is_portrait();
    let compact = crate::ui::is_compact_landscape();
    let panel_width = if portrait {
        board.w - 18.
    } else if compact {
        board.w - 24.
    } else {
        380.
    };
    let panel_height = if portrait {
        board.h - 18.
    } else if compact {
        190.
    } else {
        230.
    };
    let panel = Rect::new(
        board.x + (board.w - panel_width) * 0.5,
        board.y + (board.h - panel_height) * 0.5,
        panel_width,
        panel_height,
    );
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        Color::new(0.12, 0.15, 0.19, 0.96),
    );
    draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 3., theme::BRASS);

    let title_size = if portrait { 21. } else { 31. };
    centered_label(
        if won { "YOU WON!" } else { "OUT OF SHOTS" },
        panel.x + panel.w * 0.5,
        panel.y + if portrait { 31. } else { 48. },
        title_size,
        if won {
            theme::BRASS
        } else {
            Color::new(1., 0.52, 0.28, 1.)
        },
    );
    centered_label(
        &if won {
            format!("LEVEL {} CLEARED", game.level_number())
        } else {
            format!("LEVEL {} NOT CLEARED", game.level_number())
        },
        panel.x + panel.w * 0.5,
        panel.y + if portrait { 52. } else { 75. },
        if portrait { 10. } else { 13. },
        theme::CREAM,
    );

    let star_y = panel.y + if portrait { 84. } else { 112. };
    let star_spacing = if portrait { 33. } else { 47. };
    let first_star = panel.x + panel.w * 0.5 - star_spacing;
    for index in 0..3 {
        let color = if won && index < usize::from(game.stars()) {
            Color::new(1., 0.76, 0.24, 1.)
        } else {
            Color::new(0.28, 0.33, 0.37, 1.)
        };
        draw_star(
            vec2(first_star + index as f32 * star_spacing, star_y),
            if portrait { 12. } else { 17. },
            color,
        );
    }
    let result_line = if won {
        format!("{} STARS", game.stars())
    } else {
        format!("NO CLEAR · TARGETS LEFT: {}", remaining_targets(game))
    };
    centered_label(
        &result_line,
        panel.x + panel.w * 0.5,
        panel.y + if portrait { 112. } else { 146. },
        if portrait { 9. } else { 13. },
        if won {
            Color::new(1., 0.84, 0.42, 1.)
        } else {
            Color::new(1., 0.68, 0.36, 1.)
        },
    );
    centered_label(
        if portrait {
            if won {
                "Choose NEXT LEVEL or RESTART"
            } else {
                "Tap RESTART LEVEL to try again"
            }
        } else if won {
            "Your fort is safe — choose NEXT LEVEL or RESTART LEVEL"
        } else {
            "The fort still stands — choose RESTART LEVEL to try again"
        },
        panel.x + panel.w * 0.5,
        panel.y + panel.h - if portrait { 14. } else { 20. },
        if portrait { 8. } else { 11. },
        theme::SECONDARY,
    );
}

fn remaining_targets(game: &FlingFury) -> usize {
    game.targets.iter().filter(|target| target.alive).count()
}

fn draw_star(center: Vec2, radius: f32, color: Color) {
    let mut points = [Vec2::ZERO; 10];
    for (index, point) in points.iter_mut().enumerate() {
        let angle = -std::f32::consts::FRAC_PI_2 + index as f32 * std::f32::consts::PI / 5.;
        let length = if index % 2 == 0 {
            radius
        } else {
            radius * 0.45
        };
        *point = center + vec2(angle.cos(), angle.sin()) * length;
    }
    for index in 0..10 {
        draw_triangle(center, points[index], points[(index + 1) % 10], color);
    }
    for index in 0..10 {
        let next = (index + 1) % 10;
        draw_line(
            points[index].x,
            points[index].y,
            points[next].x,
            points[next].y,
            1.,
            theme::CREAM,
        );
    }
}

fn centered_label(value: &str, center_x: f32, y: f32, size: f32, color: Color) {
    let readable = crate::ui::readable_text_size(size);
    let width = crate::ui::measure_text(value, None, readable as u16, 1.).width;
    crate::ui::draw_text(value, center_x - width * 0.5, y, size, color);
}
