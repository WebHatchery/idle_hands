//! Compact touch tutorials with exact visible-control instructions.

use crate::{content::GameContent, state::GameId, tutorial_data, ui::UiAction};
use macroquad::prelude::*;

pub const PORTRAIT_PANEL: Rect = Rect::new(15., 70., 330., 650.);
pub const LANDSCAPE_PANEL: Rect = Rect::new(40., 24., 760., 354.);

pub fn continue_rect(compact_landscape: bool) -> Rect {
    if compact_landscape {
        Rect::new(584., 320., 150., 48.)
    } else {
        Rect::new(105., 650., 150., 50.)
    }
}

pub fn replay_rect(compact_landscape: bool) -> Rect {
    if compact_landscape {
        Rect::new(700., 2., 130., 44.)
    } else {
        Rect::new(245., 0., 105., 44.)
    }
}

pub fn tutorial_clicks(point: Vec2, compact_landscape: bool) -> Vec<UiAction> {
    if crate::ui::hit(continue_rect(compact_landscape), point) {
        vec![UiAction::TutorialContinue]
    } else {
        Vec::new()
    }
}

pub fn replay_clicks(point: Vec2, compact_landscape: bool) -> bool {
    crate::ui::hit(replay_rect(compact_landscape), point)
}

pub fn draw_replay_button(compact_landscape: bool, large_text: bool, high_contrast: bool) {
    let rect = replay_rect(compact_landscape);
    panel(
        rect,
        Color::new(
            crate::theme::SURFACE_DARK.r,
            crate::theme::SURFACE_DARK.g,
            crate::theme::SURFACE_DARK.b,
            0.96,
        ),
        high_contrast,
    );
    crate::ui::draw_text(
        "TUTORIAL",
        rect.x + if compact_landscape { 14. } else { 13. },
        rect.y + rect.h * 0.64,
        crate::accessibility::text_size(if compact_landscape { 13. } else { 11. }, large_text),
        WHITE,
    );
}

pub fn draw_tutorial(
    game: GameId,
    content: &GameContent,
    compact_landscape: bool,
    large_text: bool,
    high_contrast: bool,
) {
    if compact_landscape {
        draw_landscape(game, content, large_text, high_contrast);
    } else {
        draw_portrait(game, content, large_text, high_contrast);
    }
}

pub fn draw_portrait(game: GameId, content: &GameContent, large_text: bool, high_contrast: bool) {
    panel(
        PORTRAIT_PANEL,
        Color::new(0.07, 0.045, 0.13, 0.98),
        high_contrast,
    );
    let title_size = crate::accessibility::text_size(25., large_text);
    let game_size = crate::accessibility::text_size(20., large_text);
    let instruction_size = crate::accessibility::text_size(14., large_text);
    crate::ui::draw_text("HOW TO PLAY", 35., 160., title_size, crate::theme::BRASS);
    crate::ui::draw_text(
        content
            .game(game)
            .map_or_else(|| game.title(), |entry| entry.title.as_str()),
        35.,
        198.,
        game_size,
        WHITE,
    );
    let mut y = 240.;
    for (index, instruction) in tutorial_data::instructions(content, game)
        .iter()
        .enumerate()
    {
        for (line_index, line) in macroquad_toolkit::ui::wrap_text(
            instruction,
            PORTRAIT_PANEL.right() - 35. - 44.,
            instruction_size,
        )
        .iter()
        .enumerate()
        {
            let prefix = if line_index == 0 {
                format!("{}. ", index + 1)
            } else {
                "   ".to_owned()
            };
            crate::ui::draw_text(
                format!("{}{}", prefix, line),
                35.,
                y,
                instruction_size,
                Color::new(0.82, 0.78, 0.89, 1.),
            );
            y += instruction_size + 8.;
        }
        y += 9.;
    }
    draw_continue(false, large_text, high_contrast);
}

pub fn draw_landscape(game: GameId, content: &GameContent, large_text: bool, high_contrast: bool) {
    panel(
        LANDSCAPE_PANEL,
        Color::new(0.07, 0.045, 0.13, 0.98),
        high_contrast,
    );
    let title_size = crate::accessibility::text_size(28., large_text);
    let game_size = crate::accessibility::text_size(19., large_text);
    let instruction_size = crate::accessibility::text_size(13., large_text);
    crate::ui::draw_text("HOW TO PLAY", 122., 90., title_size, crate::theme::BRASS);
    crate::ui::draw_text(
        content
            .game(game)
            .map_or_else(|| game.title(), |entry| entry.title.as_str()),
        122.,
        123.,
        game_size,
        WHITE,
    );
    let mut y = 154.;
    for (index, instruction) in tutorial_data::instructions(content, game)
        .iter()
        .enumerate()
    {
        for (line_index, line) in macroquad_toolkit::ui::wrap_text(
            instruction,
            LANDSCAPE_PANEL.right() - 122. - 44.,
            instruction_size,
        )
        .iter()
        .enumerate()
        {
            let prefix = if line_index == 0 {
                format!("{}. ", index + 1)
            } else {
                "   ".to_owned()
            };
            crate::ui::draw_text(
                format!("{}{}", prefix, line),
                122.,
                y,
                instruction_size,
                Color::new(0.82, 0.78, 0.89, 1.),
            );
            y += instruction_size + 6.;
        }
        y += 4.;
    }
    draw_continue(true, large_text, high_contrast);
}

pub fn draw_continue(compact_landscape: bool, large_text: bool, high_contrast: bool) {
    let rect = continue_rect(compact_landscape);
    panel(rect, crate::theme::MOSS_DARK, high_contrast);
    crate::ui::draw_text(
        "CONTINUE",
        rect.x + 34.,
        rect.y + rect.h * 0.64,
        crate::accessibility::text_size(15., large_text),
        WHITE,
    );
}

pub fn panel(rect: Rect, fill: Color, high_contrast: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        if high_contrast {
            WHITE
        } else {
            Color::new(0.55, 0.43, 0.70, 0.9)
        },
    );
}
