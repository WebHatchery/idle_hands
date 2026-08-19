//! Responsive touch presentation for Match Three.

use crate::{
    accessibility,
    match_three::{MatchThree, MatchThreeDifficulty, MatchThreePhase},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    difficulty: [Rect; 3],
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(250., 44., 300., 300.),
            hint: Rect::new(620., 110., 105., 44.),
            undo: Rect::new(620., 165., 105., 44.),
            new_game: Rect::new(735., 165., 105., 44.),
            difficulty: [
                Rect::new(620., 44., 62., 38.),
                Rect::new(686., 44., 62., 38.),
                Rect::new(752., 44., 78., 38.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(5., 105., 330., 330.),
            hint: Rect::new(15., 465., 145., 44.),
            undo: Rect::new(5., 520., 145., 44.),
            new_game: Rect::new(165., 520., 170., 44.),
            difficulty: [
                Rect::new(15., 600., 90., 38.),
                Rect::new(115., 600., 90., 38.),
                Rect::new(215., 600., 100., 38.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            hint: Rect::new(810., 180., 120., 44.),
            undo: Rect::new(810., 235., 120., 44.),
            new_game: Rect::new(950., 235., 140., 44.),
            difficulty: [
                Rect::new(810., 90., 85., 38.),
                Rect::new(900., 90., 85., 38.),
                Rect::new(990., 90., 95., 38.),
            ],
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.difficulty.iter().enumerate() {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::MatchThreeDifficulty(
                MatchThreeDifficulty::ALL[index],
            )];
        }
    }
    if l.board.contains(point) {
        let side = _state.match_three.side();
        let cell = l.board.w / side as f32;
        let col = ((point.x - l.board.x) / cell) as usize;
        let row = ((point.y - l.board.y) / cell) as usize;
        if row < side && col < side {
            return vec![UiAction::MatchThreeTap(row * side + col)];
        }
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::MatchThreeHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::MatchThreeUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::MatchThreeNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.match_three;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
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
    crate::ui::draw_text("‹ CABINET", 8., 30., 13., muted());
    crate::ui::draw_text(
        "MATCH THREE",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    crate::ui::draw_text(
        format!(
            "{} points  •  {}  •  {} / {}",
            game.score,
            status(game.phase),
            game.difficulty.label(),
            game.target_score()
        ),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast);
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW BOARD", state.large_text);
    for (index, rect) in l.difficulty.iter().enumerate() {
        mode_button(
            *rect,
            ["STD", "HARD", "EXPERT"][index],
            MatchThreeDifficulty::ALL[index] == game.difficulty,
            state.large_text,
        );
    }
    let status_y = if portrait {
        585.
    } else if compact {
        365.
    } else {
        545.
    };
    crate::ui::draw_text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap two adjacent tiles to clear matching colors"),
        if compact {
            250.
        } else if portrait {
            15.
        } else {
            title_x
        },
        status_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
}

fn draw_board(board: Rect, game: &MatchThree, high_contrast: bool) {
    let side = game.side();
    let cell = board.w / side as f32;
    for index in 0..side * side {
        let rect = Rect::new(
            board.x + (index % side) as f32 * cell,
            board.y + (index / side) as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            palette(game.cells[index], high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if game.selected == Some(index) { 3. } else { 1. },
            if game.selected == Some(index) {
                WHITE
            } else {
                line_color(high_contrast)
            },
        );
    }
}

fn status(phase: MatchThreePhase) -> &'static str {
    match phase {
        MatchThreePhase::Playing => "REACH THE TARGET",
        MatchThreePhase::Won => "FIELD CLEARED",
    }
}
fn palette(color: u8, high_contrast: bool) -> Color {
    if high_contrast {
        [
            Color::new(1., 0.20, 0.25, 1.),
            Color::new(1., 0.82, 0.05, 1.),
            Color::new(0.10, 0.95, 0.30, 1.),
            Color::new(0.10, 0.55, 1., 1.),
            Color::new(1., 0.25, 0.95, 1.),
            Color::new(0.96, 0.42, 0.08, 1.),
            Color::new(0.20, 0.90, 0.88, 1.),
        ][color as usize % 7]
    } else {
        [
            Color::new(0.94, 0.35, 0.42, 1.),
            Color::new(0.98, 0.72, 0.28, 1.),
            Color::new(0.35, 0.82, 0.58, 1.),
            Color::new(0.32, 0.64, 0.95, 1.),
            Color::new(0.68, 0.45, 0.90, 1.),
            Color::new(0.95, 0.48, 0.25, 1.),
            Color::new(0.28, 0.78, 0.76, 1.),
        ][color as usize % 7]
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        label,
        rect,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn mode_button(rect: Rect, label: &str, selected: bool, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            crate::theme::MOSS_DARK
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 2. } else { 1. },
        accent(),
    );
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
