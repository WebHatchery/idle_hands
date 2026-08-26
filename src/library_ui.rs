//! Static collection Rules and Credits screens.

use crate::{state::GameId, ui::UiAction};
use macroquad::prelude::*;

fn panel(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::BACKGROUND_DEEP,
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn back_button() {
    draw_rectangle(1030., 635., 180., 48., crate::theme::MOSS_DARK);
    crate::ui::draw_text("BACK", 1090., 666., 18., WHITE);
}
pub fn rules_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(1030., 635., 180., 48.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
pub fn credits_clicks(p: Vec2) -> Vec<UiAction> {
    if Rect::new(1030., 635., 180., 48.).contains(p) {
        vec![UiAction::Cabinet]
    } else {
        vec![]
    }
}
pub fn draw_rules() {
    panel(Rect::new(120., 55., 1040., 610.));
    crate::ui::draw_text("RULES", 170., 125., 46., crate::theme::BRASS);
    crate::ui::draw_text(
        "Every drawer keeps its controls visible and touch-complete.",
        174.,
        157.,
        19.,
        crate::theme::SECONDARY,
    );
    for (index, game) in GameId::ALL.iter().enumerate() {
        let column = index / 11;
        let row = index % 11;
        let x = 160. + column as f32 * 245.;
        let y = 215. + row as f32 * 38.;
        crate::ui::draw_text(game.title(), x, y, 12., crate::theme::BRASS);
        crate::ui::draw_text(game.subtitle(), x, y + 15., 9., crate::theme::CREAM);
    }
    back_button();
}
pub fn draw_credits() {
    panel(Rect::new(230., 95., 820., 520.));
    crate::ui::draw_text("CREDITS", 300., 175., 46., crate::theme::BRASS);
    crate::ui::draw_text("IDLE HANDS", 305., 240., 28., WHITE);
    crate::ui::draw_text(
        "A warm collection for small pauses.",
        305.,
        280.,
        20.,
        crate::theme::CREAM,
    );
    crate::ui::draw_text(
        "Built with Rust, macroquad, and the shared macroquad-toolkit.",
        305.,
        335.,
        18.,
        crate::theme::SECONDARY,
    );
    crate::ui::draw_text(
        "All games are deterministic where practical and designed for touch.",
        305.,
        375.,
        18.,
        crate::theme::SECONDARY,
    );
    crate::ui::draw_text(
        "Original generated artwork created for Idle Hands; provenance ships with the game.",
        305.,
        415.,
        16.,
        crate::theme::SECONDARY,
    );
    crate::ui::draw_text(
        "PRIVACY: Anonymous playtime and progress help improve WebHatchery games.",
        305.,
        470.,
        14.,
        crate::theme::CREAM,
    );
    crate::ui::draw_text(
        "No advertising use; gameplay analytics collect no name or email.",
        305.,
        495.,
        14.,
        crate::theme::CREAM,
    );
    crate::ui::draw_text(
        "Thank you for spending a minute at the cabinet.",
        305.,
        540.,
        19.,
        crate::theme::BRASS,
    );
    back_button();
}
