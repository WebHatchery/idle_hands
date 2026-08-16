//! Responsive presentation and touch routing for Hangman.

use crate::{hangman::HangmanStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    keyboard: Rect,
    key_w: f32,
    key_h: f32,
    columns: usize,
    hint: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            keyboard: Rect::new(35., 235., 754., 92.),
            key_w: 54.,
            key_h: 38.,
            columns: 13,
            hint: Rect::new(490., 340., 150., 42.),
            new_game: Rect::new(650., 340., 150., 42.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            keyboard: Rect::new(10., 350., 340., 168.),
            key_w: 45.,
            key_h: 38.,
            columns: 7,
            hint: Rect::new(20., 650., 160., 42.),
            new_game: Rect::new(190., 650., 160., 42.),
        }
    } else {
        Layout {
            keyboard: Rect::new(250., 470., 780., 92.),
            key_w: 58.,
            key_h: 40.,
            columns: 13,
            hint: Rect::new(880., 620., 160., 44.),
            new_game: Rect::new(1060., 620., 160., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(layout.new_game, point) {
        return vec![UiAction::HangmanNew];
    }
    if crate::ui::hit(layout.hint, point) {
        return vec![UiAction::HangmanHint];
    }
    if layout.keyboard.contains(point) {
        let column = ((point.x - layout.keyboard.x) / layout.key_w) as usize;
        let row = ((point.y - layout.keyboard.y) / layout.key_h) as usize;
        let index = row * layout.columns + column;
        if index < 26 {
            return vec![UiAction::HangmanGuess(index as u8)];
        }
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.hangman;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let header_x = if compact {
        120.
    } else if portrait {
        10.
    } else {
        250.
    };
    let header_y = if compact {
        31.
    } else if portrait {
        78.
    } else {
        72.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("HANGMAN", header_x, header_y, title_size(), accent());
    let status = status_text(game.status, game.wrong_count);
    let instruction = state.card_hint.as_deref().unwrap_or(&status);
    text(
        instruction,
        if compact { 350. } else { header_x },
        if compact { 31. } else { header_y + 26. },
        body_size(),
        muted(),
    );
    draw_gallows(
        game.wrong_count,
        if compact {
            135.
        } else if portrait {
            180.
        } else {
            610.
        },
        110.,
    );
    draw_word(game, layout);
    draw_keyboard(game, layout);
    button(layout.new_game, "NEW WORD");
    button(layout.hint, "HINT");
}

fn draw_word(game: &crate::hangman::Hangman, layout: Layout) {
    let portrait = crate::ui::is_portrait();
    let compact = crate::ui::is_compact_landscape();
    let x = if compact {
        350.
    } else if portrait {
        20.
    } else {
        250.
    };
    let y = if compact {
        170.
    } else if portrait {
        260.
    } else {
        300.
    };
    let size = if portrait { 24. } else { 38. };
    let mut display = String::new();
    for letter in game.word.bytes() {
        if game.is_revealed(letter - b'A') || game.status == HangmanStatus::Lost {
            display.push(letter as char);
        } else {
            display.push('_');
        }
        display.push(' ');
    }
    text(&display, x, y, size, Color::new(0.98, 0.83, 0.45, 1.));
    if game.status == HangmanStatus::Lost {
        text("The word was", x, y + 42., body_size(), muted());
        text(&game.word, x + 105., y + 42., body_size(), accent());
    }
    let _ = layout;
}

fn draw_keyboard(game: &crate::hangman::Hangman, layout: Layout) {
    for index in 0..26 {
        let column = index % layout.columns;
        let row = index / layout.columns;
        let rect = Rect::new(
            layout.keyboard.x + column as f32 * layout.key_w,
            layout.keyboard.y + row as f32 * layout.key_h,
            layout.key_w - 4.,
            layout.key_h - 4.,
        );
        let fill = if game.wrong[index] {
            Color::new(0.16, 0.09, 0.14, 1.)
        } else if game.guessed[index] {
            Color::new(0.25, 0.45, 0.34, 1.)
        } else {
            Color::new(0.20, 0.13, 0.30, 1.)
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
        text(
            &char::from(b'A' + index as u8).to_string(),
            rect.x + rect.w * 0.4,
            rect.y + rect.h * 0.68,
            13.,
            WHITE,
        );
    }
}

fn draw_gallows(wrong: u8, x: f32, y: f32) {
    let color = Color::new(0.45, 0.38, 0.65, 1.);
    draw_line(x, y + 170., x + 120., y + 170., 3., color);
    draw_line(x + 22., y + 170., x + 22., y, 3., color);
    draw_line(x + 22., y, x + 82., y, 3., color);
    draw_line(x + 82., y, x + 82., y + 25., 3., color);
    if wrong >= 1 {
        draw_circle_lines(x + 82., y + 42., 17., 3., accent());
    }
    if wrong >= 2 {
        draw_line(x + 82., y + 59., x + 82., y + 112., 3., accent());
    }
    if wrong >= 3 {
        draw_line(x + 82., y + 72., x + 55., y + 92., 3., accent());
    }
    if wrong >= 4 {
        draw_line(x + 82., y + 72., x + 109., y + 92., 3., accent());
    }
    if wrong >= 5 {
        draw_line(x + 82., y + 112., x + 58., y + 145., 3., accent());
    }
    if wrong >= 6 {
        draw_line(x + 82., y + 112., x + 106., y + 145., 3., accent());
    }
}

fn status_text(status: HangmanStatus, wrong: u8) -> String {
    match status {
        HangmanStatus::Playing => format!("Wrong guesses {} / 6", wrong),
        HangmanStatus::Won => "The word is yours".into(),
        HangmanStatus::Lost => "The quiet word slipped away".into(),
    }
}
fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(label, rect.x + 12., rect.y + 28., 11., WHITE);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        29.
    } else {
        31.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
