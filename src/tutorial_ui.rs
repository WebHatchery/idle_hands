//! First-run and replayable touch tutorials shared by all game drawers.

use crate::state::GameId;
use crate::ui::UiAction;
use macroquad::prelude::*;

pub const REPLAY_RECT: Rect = Rect::new(1080., 8., 150., 42.);
const OVERLAY_RECT: Rect = Rect::new(250., 120., 780., 560.);
const CONTINUE_RECT: Rect = Rect::new(800., 600., 180., 52.);

pub fn clicks(p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(CONTINUE_RECT, p) {
        vec![UiAction::TutorialContinue]
    } else {
        vec![]
    }
}
pub fn draw_replay_button(large_text: bool, high_contrast: bool) {
    draw_rectangle(
        REPLAY_RECT.x,
        REPLAY_RECT.y,
        REPLAY_RECT.w,
        REPLAY_RECT.h,
        Color::new(
            crate::theme::GAME_PANEL.r,
            crate::theme::GAME_PANEL.g,
            crate::theme::GAME_PANEL.b,
            0.96,
        ),
    );
    draw_rectangle_lines(
        REPLAY_RECT.x,
        REPLAY_RECT.y,
        REPLAY_RECT.w,
        REPLAY_RECT.h,
        2.,
        if high_contrast {
            WHITE
        } else {
            Color::new(
                crate::theme::BRASS.r,
                crate::theme::BRASS.g,
                crate::theme::BRASS.b,
                0.8,
            )
        },
    );
    crate::ui::draw_text(
        "TUTORIAL",
        REPLAY_RECT.x + 22.,
        REPLAY_RECT.y + 27.,
        crate::accessibility::text_size(16., large_text),
        WHITE,
    );
}
pub fn draw_overlay(game: GameId, large_text: bool, high_contrast: bool) {
    let title_size = crate::accessibility::text_size(38., large_text);
    let game_size = crate::accessibility::text_size(25., large_text);
    let instruction_size = crate::accessibility::text_size(17., large_text);
    let button_size = crate::accessibility::text_size(17., large_text);
    draw_rectangle(
        OVERLAY_RECT.x,
        OVERLAY_RECT.y,
        OVERLAY_RECT.w,
        OVERLAY_RECT.h,
        Color::new(
            crate::theme::BACKGROUND_DEEP.r,
            crate::theme::BACKGROUND_DEEP.g,
            crate::theme::BACKGROUND_DEEP.b,
            0.98,
        ),
    );
    draw_rectangle_lines(
        OVERLAY_RECT.x,
        OVERLAY_RECT.y,
        OVERLAY_RECT.w,
        OVERLAY_RECT.h,
        3.,
        if high_contrast {
            WHITE
        } else {
            Color::new(0.78, 0.58, 0.30, 0.95)
        },
    );
    crate::ui::draw_text("HOW TO PLAY", 315., 225., title_size, crate::theme::BRASS);
    crate::ui::draw_text(game.title(), 315., 270., game_size, WHITE);
    let mut y = 330.;
    for line in instructions(game) {
        for wrapped in macroquad_toolkit::ui::wrap_text(line, 650., instruction_size) {
            crate::ui::draw_text(wrapped, 315., y, instruction_size, crate::theme::CREAM);
            y += instruction_size + 10.;
        }
        y += 4.;
    }
    draw_rectangle(
        CONTINUE_RECT.x,
        CONTINUE_RECT.y,
        CONTINUE_RECT.w,
        CONTINUE_RECT.h,
        crate::theme::MOSS_DARK,
    );
    crate::ui::draw_text(
        "CONTINUE",
        CONTINUE_RECT.x + 42.,
        CONTINUE_RECT.y + 33.,
        button_size,
        WHITE,
    );
}
pub(crate) use crate::tutorial_data::instructions;
#[cfg(test)]
mod tests;
