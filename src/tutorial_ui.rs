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
pub fn draw_replay_button() {
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
        Color::new(
            crate::theme::BRASS.r,
            crate::theme::BRASS.g,
            crate::theme::BRASS.b,
            0.8,
        ),
    );
    crate::ui::draw_text(
        "TUTORIAL",
        REPLAY_RECT.x + 22.,
        REPLAY_RECT.y + 27.,
        16.,
        WHITE,
    );
}
pub fn draw_overlay(game: GameId) {
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
        Color::new(0.78, 0.58, 0.30, 0.95),
    );
    crate::ui::draw_text("HOW TO PLAY", 315., 225., 38., crate::theme::BRASS);
    crate::ui::draw_text(game.title(), 315., 270., 25., WHITE);
    for (index, line) in instructions(game).iter().enumerate() {
        crate::ui::draw_text(
            line,
            315.,
            330. + index as f32 * 38.,
            19.,
            crate::theme::CREAM,
        );
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
        17.,
        WHITE,
    );
}
pub(crate) use crate::tutorial_data::instructions;
#[cfg(test)]
mod tests;
