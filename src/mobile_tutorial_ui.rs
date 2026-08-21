//! Compact touch tutorials with exact visible-control instructions.

use crate::{state::GameId, tutorial_ui, ui::UiAction};
use macroquad::prelude::*;

const PORTRAIT_PANEL: Rect = Rect::new(15., 70., 330., 650.);
const LANDSCAPE_PANEL: Rect = Rect::new(40., 24., 760., 354.);

fn continue_rect(compact_landscape: bool) -> Rect {
    if compact_landscape {
        Rect::new(584., 320., 150., 48.)
    } else {
        Rect::new(105., 650., 150., 50.)
    }
}

pub(crate) fn replay_rect(compact_landscape: bool) -> Rect {
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

pub fn draw_replay_button(compact_landscape: bool) {
    let rect = replay_rect(compact_landscape);
    panel(
        rect,
        Color::new(
            crate::theme::SURFACE_DARK.r,
            crate::theme::SURFACE_DARK.g,
            crate::theme::SURFACE_DARK.b,
            0.96,
        ),
    );
    crate::ui::draw_text(
        "TUTORIAL",
        rect.x + if compact_landscape { 14. } else { 13. },
        rect.y + rect.h * 0.64,
        if compact_landscape { 13. } else { 11. },
        WHITE,
    );
}

pub fn draw_tutorial(game: GameId, compact_landscape: bool) {
    if compact_landscape {
        draw_landscape(game);
    } else {
        draw_portrait(game);
    }
}

fn draw_portrait(game: GameId) {
    panel(PORTRAIT_PANEL, Color::new(0.07, 0.045, 0.13, 0.98));
    crate::ui::draw_text("HOW TO PLAY", 35., 160., 25., crate::theme::BRASS);
    crate::ui::draw_text(game.title(), 35., 198., 20., WHITE);
    let mut y = 240.;
    for (index, instruction) in tutorial_ui::instructions(game).iter().enumerate() {
        for (line_index, line) in wrap(instruction, 39).iter().enumerate() {
            let prefix = if line_index == 0 {
                format!("{}. ", index + 1)
            } else {
                "   ".to_owned()
            };
            crate::ui::draw_text(
                format!("{}{}", prefix, line),
                35.,
                y,
                14.,
                Color::new(0.82, 0.78, 0.89, 1.),
            );
            y += 22.;
        }
        y += 9.;
    }
    draw_continue(false);
}

fn draw_landscape(game: GameId) {
    panel(LANDSCAPE_PANEL, Color::new(0.07, 0.045, 0.13, 0.98));
    crate::ui::draw_text("HOW TO PLAY", 122., 90., 28., crate::theme::BRASS);
    crate::ui::draw_text(game.title(), 122., 123., 19., WHITE);
    let mut y = 154.;
    for (index, instruction) in tutorial_ui::instructions(game).iter().enumerate() {
        for (line_index, line) in wrap(instruction, 76).iter().enumerate() {
            let prefix = if line_index == 0 {
                format!("{}. ", index + 1)
            } else {
                "   ".to_owned()
            };
            crate::ui::draw_text(
                format!("{}{}", prefix, line),
                122.,
                y,
                13.,
                Color::new(0.82, 0.78, 0.89, 1.),
            );
            y += 19.;
        }
        y += 4.;
    }
    draw_continue(true);
}

fn draw_continue(compact_landscape: bool) {
    let rect = continue_rect(compact_landscape);
    panel(rect, crate::theme::MOSS_DARK);
    crate::ui::draw_text("CONTINUE", rect.x + 34., rect.y + rect.h * 0.64, 15., WHITE);
}

fn wrap(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let needed = current.len() + usize::from(!current.is_empty()) + word.len();
        if needed > max_chars && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.55, 0.43, 0.70, 0.9),
    );
}

#[cfg(test)]
mod tests;
