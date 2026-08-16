//! Responsive touch presentation for Color Sort.

use crate::{
    accessibility,
    color_sort::{ColorSort, ColorSortPhase, TUBES},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    tubes: [Rect; TUBES],
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let board = Rect::new(220., 54., 360., 285.);
        Layout {
            board,
            tubes: tube_rects(board),
            hint: Rect::new(630., 220., 105., 44.),
            undo: Rect::new(630., 110., 105., 44.),
            new_game: Rect::new(630., 165., 140., 44.),
        }
    } else if crate::ui::is_portrait() {
        let board = Rect::new(15., 100., 300., 270.);
        Layout {
            board,
            tubes: tube_rects(board),
            hint: Rect::new(15., 405., 145., 44.),
            undo: Rect::new(15., 460., 145., 44.),
            new_game: Rect::new(170., 460., 145., 44.),
        }
    } else {
        let board = Rect::new(280., 95., 540., 360.);
        Layout {
            board,
            tubes: tube_rects(board),
            hint: Rect::new(860., 245., 120., 44.),
            undo: Rect::new(860., 190., 120., 44.),
            new_game: Rect::new(1000., 190., 145., 44.),
        }
    }
}

fn tube_rects(board: Rect) -> [Rect; TUBES] {
    let gap = board.w / TUBES as f32;
    core::array::from_fn(|index| Rect::new(board.x + index as f32 * gap, board.y, gap, board.h))
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (tube, rect) in l.tubes.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::ColorSortTap(tube)];
        }
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::ColorSortHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::ColorSortUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::ColorSortNew];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.color_sort;
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
    draw_text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    draw_text(
        "COLOR SORT",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    draw_text(
        format!("{} moves  •  {}", game.moves, status(game.phase)),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, l.tubes, game, state.high_contrast);
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW BOARD", state.large_text);
    let status_y = if portrait {
        385.
    } else if compact {
        355.
    } else {
        480.
    };
    draw_text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Tap a source tube, then a matching destination"),
        if compact { 220. } else { title_x },
        status_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
}

fn draw_board(board: Rect, tubes: [Rect; TUBES], game: &ColorSort, high_contrast: bool) {
    draw_rectangle(
        board.x,
        board.y,
        board.w,
        board.h,
        accessibility::board_fill(high_contrast),
    );
    draw_rectangle_lines(
        board.x,
        board.y,
        board.w,
        board.h,
        2.,
        accessibility::grid_line(high_contrast),
    );
    for (index, rect) in tubes.iter().enumerate() {
        let selected = game.selected == Some(index);
        draw_rectangle_lines(
            rect.x + 8.,
            rect.y + 8.,
            rect.w - 16.,
            rect.h - 16.,
            if selected { 3. } else { 1. },
            if selected {
                accent()
            } else {
                line_color(high_contrast)
            },
        );
        let tube = &game.tubes[index];
        let slot = (rect.h - 38.) / 4.;
        for (level, &color) in tube.iter().enumerate() {
            let ball = Rect::new(
                rect.x + 14.,
                rect.y + rect.h - 22. - (level + 1) as f32 * slot,
                rect.w - 28.,
                slot - 5.,
            );
            draw_rectangle(
                ball.x,
                ball.y,
                ball.w,
                ball.h,
                palette(color, high_contrast),
            );
            draw_rectangle_lines(ball.x, ball.y, ball.w, ball.h, 1., WHITE);
        }
    }
}

fn status(phase: ColorSortPhase) -> &'static str {
    match phase {
        ColorSortPhase::Playing => "SORT THE COLORS",
        ColorSortPhase::Won => "TUBES COMPLETE",
    }
}

fn palette(color: u8, high_contrast: bool) -> Color {
    let palette = if high_contrast {
        [
            Color::new(1., 0.18, 0.22, 1.),
            Color::new(1., 0.82, 0.05, 1.),
            Color::new(0.05, 0.95, 0.30, 1.),
            Color::new(0.05, 0.60, 1., 1.),
        ]
    } else {
        [
            Color::new(0.94, 0.35, 0.42, 1.),
            Color::new(0.98, 0.72, 0.28, 1.),
            Color::new(0.35, 0.82, 0.58, 1.),
            Color::new(0.32, 0.64, 0.95, 1.),
        ]
    };
    palette[color as usize % 4]
}

fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        label,
        rect,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = measure_text(label, None, size as u16, 1.);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.63,
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
fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}
