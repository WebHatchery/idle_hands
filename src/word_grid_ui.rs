//! Responsive touch presentation for Word Grid.

use crate::{
    accessibility,
    state::AppState,
    ui::UiAction,
    word_grid::{LetterState, WordGrid, WordGridMode, WordGridPhase, MAX_GUESSES, WORD_LENGTH},
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
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    mode: Rect,
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
            hint: Rect::new(560., 140., 100., 44.),
            undo: Rect::new(560., 90., 100., 44.),
            new_game: Rect::new(670., 90., 150., 44.),
            mode: Rect::new(670., 140., 150., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(55., 105., 250., 300.),
            keyboard: Rect::new(12., 515., 336., 176.),
            key_w: 48.,
            key_h: 44.,
            columns: 7,
            backspace: Rect::new(15., 420., 100., 44.),
            submit: Rect::new(125., 420., 100., 44.),
            hint: Rect::new(235., 420., 100., 44.),
            undo: Rect::new(15., 700., 100., 44.),
            new_game: Rect::new(125., 700., 150., 44.),
            mode: Rect::new(285., 700., 75., 44.),
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
            hint: Rect::new(1000., 370., 100., 44.),
            undo: Rect::new(1000., 250., 100., 44.),
            new_game: Rect::new(760., 310., 150., 44.),
            mode: Rect::new(920., 310., 110., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    if l.backspace.contains(point) {
        return vec![UiAction::WordGridBackspace];
    }
    if crate::ui::hit(l.submit, point) {
        return vec![UiAction::WordGridSubmit];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::WordGridHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::WordGridUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::WordGridNew];
    }
    if crate::ui::hit(l.mode, point) {
        return vec![UiAction::WordGridMode(match state.games.word_grid.mode {
            WordGridMode::Classic => WordGridMode::Hard,
            WordGridMode::Hard => WordGridMode::Classic,
        })];
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
    let game = &state.games.word_grid;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        crate::ui::COMPACT_HEADER_TITLE_X
    } else if portrait {
        25.
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
    crate::ui::draw_text(
        "CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    crate::ui::draw_text(
        "WORD GRID",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let scoreline = if portrait {
        format!("{}/6 • {} left", game.moves, game.remaining_words().len())
    } else if compact || crate::ui::display_width() < 360. {
        format!(
            "{}/6 • {} left • {}",
            game.moves,
            game.remaining_words().len(),
            game.mode.label()
        )
    } else {
        format!(
            "Guesses {}/6  •  {} candidates  •  {}",
            game.moves,
            game.remaining_words().len(),
            game.mode.label()
        )
    };
    crate::ui::draw_text(
        scoreline,
        if compact {
            compact_scoreline_position().0
        } else {
            title_x
        },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast, state.large_text);
    draw_keyboard(l, game, state.high_contrast, state.large_text);
    button(l.backspace, "BACKSPACE", state.large_text);
    button(l.submit, "SUBMIT", state.large_text);
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW WORD", state.large_text);
    mode_button(l.mode, game.mode.label(), state.large_text);
    let (status_x, status_y) = if portrait {
        portrait_scoreline_position()
    } else if compact {
        (300., 220.)
    } else {
        (760., 450.)
    };
    crate::ui::draw_text(
        state.card_hint.as_deref().unwrap_or_else(|| {
            if game.notice.is_empty() {
                status(game.phase)
            } else {
                &game.notice
            }
        }),
        status_x,
        status_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
}

fn draw_board(board: Rect, game: &WordGrid, high_contrast: bool, large_text: bool) {
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
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                tile_color(state, high_contrast),
            );
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                1.,
                line_color(high_contrast),
            );
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
                    accessibility::text_size(
                        if crate::ui::is_portrait() { 20. } else { 27. },
                        large_text,
                    ),
                    WHITE,
                );
                if state != LetterState::Unknown {
                    center_text(
                        match state {
                            LetterState::Correct => "=",
                            LetterState::Present => "?",
                            LetterState::Absent => "X",
                            LetterState::Unknown => "",
                        },
                        Rect::new(rect.x + rect.w * 0.62, rect.y, rect.w * 0.35, rect.h * 0.4),
                        accessibility::text_size(9., large_text),
                        WHITE,
                    );
                }
            }
        }
    }
}

fn draw_keyboard(l: Layout, game: &WordGrid, high_contrast: bool, large_text: bool) {
    for index in 0..26 {
        let rect = Rect::new(
            l.keyboard.x + (index % l.columns) as f32 * l.key_w,
            l.keyboard.y + (index / l.columns) as f32 * l.key_h,
            l.key_w - 3.,
            l.key_h - 3.,
        );
        let state = game.used[index];
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            key_color(state, high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(high_contrast),
        );
        center_text(
            &(char::from(b'A' + index as u8)).to_string(),
            rect,
            accessibility::text_size(12., large_text),
            WHITE,
        );
    }
}

fn status(phase: WordGridPhase) -> &'static str {
    match phase {
        WordGridPhase::Playing => "Build five letters, then tap SUBMIT",
        WordGridPhase::Won => "Word found — tap NEW WORD",
        WordGridPhase::Lost => "Word revealed — tap NEW WORD",
    }
}

fn mode_button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.22, 0.30, 0.20, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, accessibility::text_size(9., large_text), WHITE);
}
fn tile_color(state: LetterState, high_contrast: bool) -> Color {
    if high_contrast {
        return match state {
            LetterState::Unknown => accessibility::board_fill(true),
            LetterState::Absent => Color::new(0.22, 0.22, 0.26, 1.),
            LetterState::Present => Color::new(1., 0.65, 0.05, 1.),
            LetterState::Correct => Color::new(0.05, 0.90, 0.32, 1.),
        };
    }
    match state {
        LetterState::Unknown => Color::new(0.12, 0.09, 0.20, 1.),
        LetterState::Absent => Color::new(0.20, 0.16, 0.28, 1.),
        LetterState::Present => Color::new(0.70, 0.48, 0.20, 1.),
        LetterState::Correct => Color::new(0.22, 0.55, 0.38, 1.),
    }
}
fn key_color(state: LetterState, high_contrast: bool) -> Color {
    if high_contrast {
        return match state {
            LetterState::Unknown => Color::new(0.18, 0.14, 0.26, 1.),
            LetterState::Absent => Color::new(0.08, 0.08, 0.10, 1.),
            LetterState::Present => Color::new(0.95, 0.55, 0.02, 1.),
            LetterState::Correct => Color::new(0.03, 0.75, 0.25, 1.),
        };
    }
    match state {
        LetterState::Unknown => crate::theme::SURFACE,
        LetterState::Absent => Color::new(0.12, 0.09, 0.17, 1.),
        LetterState::Present => Color::new(0.55, 0.35, 0.15, 1.),
        LetterState::Correct => Color::new(0.20, 0.45, 0.30, 1.),
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        label,
        rect,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    crate::ui::draw_text(
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
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}

fn portrait_scoreline_position() -> (f32, f32) {
    (12., 100.)
}

fn compact_scoreline_position() -> (f32, f32) {
    (280., 28.)
}

#[cfg(test)]
#[path = "../tests/legacy/word_grid_ui/tests.rs"]
mod tests;
