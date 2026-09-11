//! Layout contract for the rule-card control above every game drawer.

use macroquad::prelude::Rect;

#[derive(Debug, Clone, Copy)]
pub struct CardLayout {
    pub rect: Rect,
    pub label_rect: Rect,
    pub title_baseline: f32,
    pub label_baseline: f32,
    pub title_size: f32,
    pub label_size: f32,
}

pub fn layout(width: f32, portrait: bool, compact_landscape: bool) -> CardLayout {
    let (rect, title_offset, label_offset, title_size, label_size) = if portrait {
        (Rect::new(width - 170., 48., 162., 42.), 15., 32., 8., 8.)
    } else if compact_landscape {
        (Rect::new(width - 350., 5., 166., 34.), 14., 27., 9., 8.)
    } else {
        (Rect::new(width - 370., 16., 176., 38.), 14., 27., 9., 9.)
    };
    let label_height = if portrait { 17. } else { 14. };
    CardLayout {
        rect,
        label_rect: Rect::new(
            rect.x + 9.,
            rect.bottom() - label_height - 3.,
            rect.w - 35.,
            label_height,
        ),
        title_baseline: rect.y + title_offset,
        label_baseline: rect.y + label_offset,
        title_size,
        label_size,
    }
}

#[cfg(test)]
mod tests;
