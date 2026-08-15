//! Shared presentation choices for readable dense game boards.

use macroquad::prelude::Color;

pub fn text_size(base: f32, large_text: bool) -> f32 {
    if large_text {
        base * 1.18
    } else {
        base
    }
}

pub fn grid_line(high_contrast: bool) -> Color {
    if high_contrast {
        Color::new(1., 1., 1., 0.95)
    } else {
        Color::new(0.48, 0.40, 0.60, 0.8)
    }
}

pub fn board_fill(high_contrast: bool) -> Color {
    if high_contrast {
        Color::new(0.015, 0.015, 0.02, 1.)
    } else {
        Color::new(0.10, 0.07, 0.16, 1.)
    }
}

pub fn mine_cell(revealed: bool, high_contrast: bool) -> Color {
    if high_contrast {
        if revealed {
            Color::new(0.72, 0.72, 0.72, 1.)
        } else {
            Color::new(0.08, 0.08, 0.10, 1.)
        }
    } else if revealed {
        Color::new(0.24, 0.19, 0.30, 1.)
    } else {
        Color::new(0.15, 0.11, 0.23, 1.)
    }
}

pub fn nonogram_cell(mark: u8, high_contrast: bool) -> Color {
    if high_contrast {
        match mark {
            1 => Color::new(1., 1., 1., 1.),
            2 => Color::new(0.22, 0.22, 0.25, 1.),
            _ => Color::new(0.02, 0.02, 0.03, 1.),
        }
    } else {
        match mark {
            1 => Color::new(0.80, 0.52, 0.26, 1.),
            2 => Color::new(0.20, 0.14, 0.28, 1.),
            _ => Color::new(0.15, 0.11, 0.23, 1.),
        }
    }
}
