//! Responsive presentation and touch routing for Mastermind.

use crate::{accessibility, mastermind::MastermindStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
#[path = "../tests/legacy/mastermind_ui/tests.rs"]
mod tests;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    row_height: f32,
    palette: Rect,
    submit: Rect,
    clear: Rect,
    undo: Rect,
    new_board: Rect,
    hint: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(10., 45., 250., 280.),
            row_height: 27.,
            palette: Rect::new(350., 105., 240., 34.),
            submit: Rect::new(350., 160., 110., 42.),
            clear: Rect::new(480., 160., 110., 42.),
            undo: Rect::new(350., 220., 110., 42.),
            new_board: Rect::new(480., 220., 110., 42.),
            hint: Rect::new(350., 275., 240., 42.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(20., 105., 320., 420.),
            row_height: 42.,
            palette: Rect::new(20., 550., 320., 34.),
            submit: Rect::new(20., 610., 100., 42.),
            clear: Rect::new(130., 610., 100., 42.),
            undo: Rect::new(240., 610., 100., 42.),
            new_board: Rect::new(20., 670., 155., 42.),
            hint: Rect::new(185., 670., 155., 42.),
        }
    } else {
        Layout {
            board: Rect::new(380., 95., 320., 420.),
            row_height: 42.,
            palette: Rect::new(380., 550., 320., 42.),
            submit: Rect::new(720., 550., 110., 42.),
            clear: Rect::new(720., 605., 110., 42.),
            undo: Rect::new(720., 660., 110., 42.),
            new_board: Rect::new(520., 660., 180., 42.),
            hint: Rect::new(860., 550., 180., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(layout.submit, point) {
        return vec![UiAction::MastermindSubmit];
    }
    if crate::ui::hit(layout.clear, point) {
        return vec![UiAction::MastermindClear];
    }
    if crate::ui::hit(layout.undo, point) {
        return vec![UiAction::MastermindUndo];
    }
    if crate::ui::hit(layout.new_board, point) {
        return vec![UiAction::MastermindNew];
    }
    if crate::ui::hit(layout.hint, point) {
        return vec![UiAction::MastermindHint];
    }
    if layout.palette.contains(point) {
        let color = (((point.x - layout.palette.x) / (layout.palette.w / 6.)) as usize).min(5);
        return vec![UiAction::MastermindPick(color as u8)];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.games.mastermind;
    let header_y = if crate::ui::is_compact_landscape() {
        32.
    } else if crate::ui::is_portrait() {
        76.
    } else {
        68.
    };
    let header_x = if crate::ui::is_compact_landscape() {
        120.
    } else if crate::ui::is_portrait() {
        20.
    } else {
        layout.board.x
    };
    let body_x = if crate::ui::is_compact_landscape() {
        layout.palette.x
    } else {
        header_x
    };
    let body_y = if crate::ui::is_compact_landscape() {
        76.
    } else {
        header_y + 25.
    };
    text(
        "CABINET",
        back_rect().x,
        back_rect().y + 20.,
        accessibility::text_size(14., state.large_text),
        muted(),
    );
    text(
        "MASTERMIND",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let status = status_text(game.status, game.row);
    let instruction = state.card_hint.as_deref().unwrap_or(&status);
    text(
        instruction,
        body_x,
        body_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for row in 0..10 {
        let y = layout.board.y + row as f32 * layout.row_height;
        draw_line(
            layout.board.x,
            y + layout.row_height - 2.,
            layout.board.right(),
            y + layout.row_height - 2.,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        let guess = if row == game.row as usize {
            game.current
        } else {
            game.guesses[row]
        };
        for (peg, color) in guess.iter().enumerate() {
            let color = *color;
            draw_circle(
                layout.board.x + 30. + peg as f32 * 52.,
                y + layout.row_height * 0.48,
                12.,
                if color == 255 {
                    accessibility::board_fill(state.high_contrast)
                } else {
                    color_for(color, state.high_contrast)
                },
            );
            draw_circle_lines(
                layout.board.x + 30. + peg as f32 * 52.,
                y + layout.row_height * 0.48,
                13.,
                1.,
                accessibility::grid_line(state.high_contrast),
            );
        }
        if row < game.row as usize {
            text(
                &format!("{}+ {}o", game.exact[row], game.partial[row]),
                layout.board.x + 235.,
                y + layout.row_height * 0.62,
                accessibility::text_size(body_size(), state.large_text),
                muted(),
            );
        }
    }
    for color in 0..6 {
        let x = layout.palette.x + (color as f32 + 0.5) * layout.palette.w / 6.;
        draw_circle(
            x,
            layout.palette.y + layout.palette.h * 0.5,
            layout.palette.h * 0.30,
            color_for(color, state.high_contrast),
        );
        draw_circle_lines(
            x,
            layout.palette.y + layout.palette.h * 0.5,
            layout.palette.h * 0.34,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
    }
    button(layout.submit, "GUESS", state.large_text);
    button(layout.clear, "CLEAR", state.large_text);
    button(layout.undo, "UNDO", state.large_text);
    button(layout.new_board, "NEW BOARD", state.large_text);
    button(layout.hint, "HINT", state.large_text);
}

fn status_text(status: MastermindStatus, row: u8) -> String {
    match status {
        MastermindStatus::Playing => format!("Build a four-color code  -  GUESS {}/10", row + 1),
        MastermindStatus::Won => "The code is open. Start another board to play again.".into(),
        MastermindStatus::Lost => {
            "The code stayed hidden. Start another board to try again.".into()
        }
    }
}

fn color_for(color: u8, high_contrast: bool) -> Color {
    let palette = if high_contrast {
        [
            Color::new(1., 0.15, 0.20, 1.),
            Color::new(1., 0.80, 0.05, 1.),
            Color::new(0.05, 0.95, 0.30, 1.),
            Color::new(0.05, 0.60, 1., 1.),
            crate::theme::MOSS,
            Color::new(0.05, 0.95, 0.95, 1.),
        ]
    } else {
        [
            Color::new(0.92, 0.38, 0.36, 1.),
            Color::new(0.96, 0.70, 0.30, 1.),
            Color::new(0.42, 0.76, 0.45, 1.),
            Color::new(0.38, 0.70, 0.86, 1.),
            crate::theme::MOSS,
            Color::new(0.82, 0.76, 0.38, 1.),
        ]
    };
    palette[color as usize % 6]
}

fn back_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(10., 6., 100., 28.)
    } else if crate::ui::is_portrait() {
        Rect::new(0., 0., 110., 42.)
    } else {
        Rect::new(48., 24., 130., 34.)
    }
}

fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., accent());
    text(
        label,
        rect.x + 12.,
        rect.y + rect.h * 0.64,
        accessibility::text_size(body_size(), large_text),
        WHITE,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn accent() -> Color {
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        20.
    } else {
        32.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        14.
    }
}
