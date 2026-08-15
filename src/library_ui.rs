//! Static collection Rules and Credits screens.

use crate::ui::UiAction;
use macroquad::prelude::*;

fn panel(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}
fn back_button() {
    draw_rectangle(1030., 635., 180., 48., Color::new(0.25, 0.16, 0.32, 1.));
    draw_text("BACK", 1090., 666., 18., WHITE);
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
    draw_text("RULES", 170., 125., 46., Color::new(0.98, 0.83, 0.45, 1.));
    draw_text(
        "Every drawer keeps its controls visible and touch-complete.",
        174.,
        157.,
        19.,
        Color::new(0.72, 0.68, 0.82, 1.),
    );
    let lines = ["2048 — swipe or tap an arrow to merge matching tiles.", "Minesweeper — reveal safely, flag with FLAG MODE or a long press, and chord marked numbers.", "Sudoku — select a cell, then use the number pad; pencil marks and erase are optional.", "Nonogram — fill or cross cells from the clues; drag in a straight row or column.", "Solitaire / FreeCell — tap a card, then tap a legal destination.", "Fivefold — roll up to three times, hold dice, then choose one score row.", "Reversi — place on a glowing legal square; pass only when no move exists."];
    for (index, line) in lines.iter().enumerate() {
        draw_text(
            line,
            174.,
            215. + index as f32 * 42.,
            17.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
    }
    back_button();
}
pub fn draw_credits() {
    panel(Rect::new(230., 95., 820., 520.));
    draw_text("CREDITS", 300., 175., 46., Color::new(0.98, 0.83, 0.45, 1.));
    draw_text("IDLE HANDS", 305., 240., 28., WHITE);
    draw_text(
        "A quiet collection for small pauses.",
        305.,
        280.,
        20.,
        Color::new(0.78, 0.73, 0.86, 1.),
    );
    draw_text(
        "Built with Rust, macroquad, and the shared macroquad-toolkit.",
        305.,
        335.,
        18.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    draw_text(
        "All games are deterministic where practical and designed for touch.",
        305.,
        375.,
        18.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    draw_text(
        "Thank you for spending a quiet minute at the cabinet.",
        305.,
        450.,
        19.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    back_button();
}
