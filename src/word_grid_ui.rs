//! Responsive touch presentation for Word Grid.

use crate::{
    state::AppState,
    ui::UiAction,
    word_grid::{LetterState, WordGrid, WordGridPhase, MAX_GUESSES, WORD_LENGTH},
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    keyboard: Rect,
    key_w: f32,
    key_h: f32,
    columns: usize,
    backspace: Rect,
    submit: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(35., 44., 240., 240.),
            keyboard: Rect::new(35., 300., 754., 76.),
            key_w: 58.,
            key_h: 38.,
            columns: 13,
            backspace: Rect::new(300., 90., 120., 44.),
            submit: Rect::new(430., 90., 120., 44.),
            undo: Rect::new(560., 90., 100., 44.),
            new_game: Rect::new(670., 90., 150., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 105., 300., 300.),
            keyboard: Rect::new(15., 555., 315., 126.),
            key_w: 35.,
            key_h: 42.,
            columns: 9,
            backspace: Rect::new(15., 455., 100., 44.),
            submit: Rect::new(125., 455., 100., 44.),
            undo: Rect::new(15., 700., 100., 44.),
            new_game: Rect::new(125., 700., 150., 44.),
        }
    } else {
        Layout {
            board: Rect::new(300., 95., 420., 420.),
            keyboard: Rect::new(760., 100., 494., 126.),
            key_w: 38.,
            key_h: 42.,
            columns: 13,
            backspace: Rect::new(760., 250., 110., 44.),
            submit: Rect::new(880., 250., 110., 44.),
            undo: Rect::new(1000., 250., 100., 44.),
            new_game: Rect::new(760., 310., 150., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    if l.backspace.contains(point) {
        return vec![UiAction::WordGridBackspace];
    }
    if l.submit.contains(point) {
        return vec![UiAction::WordGridSubmit];
    }
    if l.undo.contains(point) {
        return vec![UiAction::WordGridUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::WordGridNew];
    }
    if l.keyboard.contains(point) {
        let column = ((point.x - l.keyboard.x) / l.key_w) as usize;
        let row = ((point.y - l.keyboard.y) / l.key_h) as usize;
        let index = row * l.columns + column;
        if index < 26 {
            return vec![UiAction::WordGridLetter(index as u8)];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.word_grid;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        28.
    } else if portrait {
        62.
    } else {
        58.
    };
    draw_text("‹ CABINET", 8., 30., 13., muted());
    draw_text("WORD GRID", title_x, title_y, title_size(), accent());
    draw_text(
        format!("{} / 6 guesses  •  {}", game.moves, status(game.phase)),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, game);
    draw_keyboard(l, game);
    button(l.backspace, "BACKSPACE");
    button(l.submit, "SUBMIT");
    button(l.undo, "UNDO");
    button(l.new_game, "NEW WORD");
    let status_y = if portrait {
        760.
    } else if compact {
        285.
    } else {
        560.
    };
    draw_text(
        "Build five letters, then tap SUBMIT",
        if compact { 245. } else { title_x },
        status_y,
        body_size(),
        muted(),
    );
}

fn draw_board(board: Rect, game: &WordGrid) {
    let cell = board.w / WORD_LENGTH as f32;
    for row in 0..MAX_GUESSES {
        for col in 0..WORD_LENGTH {
            let rect = Rect::new(
                board.x + col as f32 * cell,
                board.y + row as f32 * cell,
                cell,
                cell,
            );
            let state = game
                .feedback
                .get(row)
                .map_or(LetterState::Unknown, |feedback| feedback[col]);
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, tile_color(state));
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
            let value = game
                .guesses
                .get(row)
                .and_then(|guess| guess.as_bytes().get(col))
                .copied()
                .or_else(|| {
                    (row == game.guesses.len())
                        .then(|| game.current.as_bytes().get(col).copied())
                        .flatten()
                });
            if let Some(letter) = value {
                center_text(
                    &(letter as char).to_string(),
                    rect,
                    if crate::ui::is_portrait() { 20. } else { 27. },
                    WHITE,
                );
            }
        }
    }
}

fn draw_keyboard(l: Layout, game: &WordGrid) {
    for index in 0..26 {
        let rect = Rect::new(
            l.keyboard.x + (index % l.columns) as f32 * l.key_w,
            l.keyboard.y + (index / l.columns) as f32 * l.key_h,
            l.key_w - 3.,
            l.key_h - 3.,
        );
        let state = game.used[index];
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, key_color(state));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
        center_text(
            &(char::from(b'A' + index as u8)).to_string(),
            rect,
            12.,
            WHITE,
        );
    }
}

fn status(phase: WordGridPhase) -> &'static str {
    match phase {
        WordGridPhase::Playing => "SOLVE THE WORD",
        WordGridPhase::Won => "WORD FOUND",
        WordGridPhase::Lost => "WORD REVEALED",
    }
}
fn tile_color(state: LetterState) -> Color {
    match state {
        LetterState::Unknown => Color::new(0.12, 0.09, 0.20, 1.),
        LetterState::Absent => Color::new(0.20, 0.16, 0.28, 1.),
        LetterState::Present => Color::new(0.70, 0.48, 0.20, 1.),
        LetterState::Correct => Color::new(0.22, 0.55, 0.38, 1.),
    }
}
fn key_color(state: LetterState) -> Color {
    match state {
        LetterState::Unknown => Color::new(0.20, 0.13, 0.30, 1.),
        LetterState::Absent => Color::new(0.12, 0.09, 0.17, 1.),
        LetterState::Present => Color::new(0.55, 0.35, 0.15, 1.),
        LetterState::Correct => Color::new(0.20, 0.45, 0.30, 1.),
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
    center_text(label, rect, 10., WHITE);
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = measure_text(label, None, size as u16, 1.);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.65,
        size,
        color,
    );
}
fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        21.
    } else {
        27.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}
fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}
fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}
fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
