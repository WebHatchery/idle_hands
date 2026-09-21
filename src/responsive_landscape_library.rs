//! Medium landscape layouts for library and settings screens.

use macroquad::prelude::*;

mod info;
mod records;
mod settings;

pub use info::{credits_clicks, draw_credits, draw_help, help_clicks};
pub use records::{draw_records, records_clicks};
pub use settings::{draw_settings, settings_clicks};

pub fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
pub fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub fn back(rect: Rect) {
    panel(rect, crate::theme::MOSS_DARK);
    text("BACK", rect.x + 30., rect.y + 28., 12., WHITE);
}

pub const RECORDS_VISIBLE_ROWS: usize = 10;

pub fn scroll(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 16., rect.y + 28., 10., WHITE);
}
