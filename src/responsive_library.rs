//! Compact portrait layouts for collection-wide library screens.

use macroquad::prelude::*;

mod info;
mod records;
mod rules;

pub use info::{credits_clicks, draw_credits, draw_help, help_clicks};
pub use records::{draw_records, records_clicks};
pub use rules::{draw_rules, rules_clicks};

#[cfg(test)]
#[path = "../tests/legacy/responsive_library/tests.rs"]
mod tests;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn back_button(y: f32) {
    panel(Rect::new(10., y, 150., 44.), crate::theme::MOSS_DARK);
    text("BACK", 62., y + 28., 12., WHITE);
}
fn value(value: Option<u32>) -> String {
    value.map_or_else(|| "-".into(), |number| number.to_string())
}

const RECORDS_VISIBLE_ROWS: usize = 11;
const RULES_VISIBLE_ROWS: usize = 8;

fn scroll_button(rect: Rect, label: &str) {
    panel(rect, crate::theme::SURFACE_DARK);
    text(label, rect.x + 18., rect.y + 28., 11., WHITE);
}
