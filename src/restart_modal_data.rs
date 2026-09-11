//! Responsive geometry for the restart-confirmation modal.

use macroquad::prelude::{Rect, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub panel: Rect,
    pub cancel: Rect,
    pub start: Rect,
    pub title_position: Vec2,
    pub detail_position: Vec2,
    pub title_size: f32,
    pub detail_size: f32,
}

pub fn layout(portrait: bool, compact_landscape: bool) -> Layout {
    if compact_landscape {
        Layout {
            panel: Rect::new(270., 95., 320., 170.),
            cancel: Rect::new(290., 195., 115., 44.),
            start: Rect::new(445., 195., 115., 44.),
            title_position: Vec2::new(305., 135.),
            detail_position: Vec2::new(305., 160.),
            title_size: 21.,
            detail_size: 13.,
        }
    } else if portrait {
        Layout {
            panel: Rect::new(25., 255., 310., 190.),
            cancel: Rect::new(45., 360., 120., 44.),
            start: Rect::new(195., 360., 120., 44.),
            title_position: Vec2::new(55., 300.),
            detail_position: Vec2::new(55., 330.),
            title_size: 18.,
            detail_size: 11.,
        }
    } else {
        Layout {
            panel: Rect::new(390., 250., 500., 200.),
            cancel: Rect::new(450., 360., 160., 48.),
            start: Rect::new(670., 360., 160., 48.),
            title_position: Vec2::new(445., 305.),
            detail_position: Vec2::new(445., 335.),
            title_size: 21.,
            detail_size: 13.,
        }
    }
}

#[cfg(test)]
mod tests;
