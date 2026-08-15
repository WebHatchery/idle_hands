//! Responsive touch presentation for Color Sort.

use crate::{
    color_sort::{ColorSort, ColorSortPhase, TUBES},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    tubes: [Rect; TUBES],
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let board = Rect::new(220., 54., 360., 285.);
        Layout {
            board,
            tubes: tube_rects(board),
            undo: Rect::new(630., 110., 105., 44.),
            new_game: Rect::new(630., 165., 140., 44.),
        }
    } else if crate::ui::is_portrait() {
        let board = Rect::new(5., 105., 330., 270.);
        Layout {
            board,
            tubes: tube_rects(board),
            undo: Rect::new(5., 410., 145., 44.),
            new_game: Rect::new(165., 410., 170., 44.),
        }
    } else {
        let board = Rect::new(280., 95., 540., 360.);
        Layout {
            board,
            tubes: tube_rects(board),
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
    if Rect::new(0., 0., 110., 42.).contains(point) {
        return vec![UiAction::Cabinet];
    }
    for (tube, rect) in l.tubes.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::ColorSortTap(tube)];
        }
    }
    if l.undo.contains(point) {
        return vec![UiAction::ColorSortUndo];
    }
    if l.new_game.contains(point) {
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
    draw_text("COLOR SORT", title_x, title_y, title_size(), accent());
    draw_text(
        format!("{} moves  •  {}", game.moves, status(game.phase)),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    draw_board(l.board, l.tubes, game);
    button(l.undo, "UNDO");
    button(l.new_game, "NEW BOARD");
    let status_y = if portrait {
        475.
    } else if compact {
        355.
    } else {
        480.
    };
    draw_text(
        "Tap a source tube, then a matching destination",
        if compact { 220. } else { title_x },
        status_y,
        body_size(),
        muted(),
    );
}

fn draw_board(board: Rect, tubes: [Rect; TUBES], game: &ColorSort) {
    draw_rectangle(
        board.x,
        board.y,
        board.w,
        board.h,
        Color::new(0.10, 0.07, 0.16, 1.),
    );
    draw_rectangle_lines(board.x, board.y, board.w, board.h, 2., accent());
    for (index, rect) in tubes.iter().enumerate() {
        let selected = game.selected == Some(index);
        draw_rectangle_lines(
            rect.x + 8.,
            rect.y + 8.,
            rect.w - 16.,
            rect.h - 16.,
            if selected { 3. } else { 1. },
            if selected { accent() } else { line_color() },
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
            draw_rectangle(ball.x, ball.y, ball.w, ball.h, palette(color));
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

fn palette(color: u8) -> Color {
    [
        Color::new(0.94, 0.35, 0.42, 1.),
        Color::new(0.98, 0.72, 0.28, 1.),
        Color::new(0.35, 0.82, 0.58, 1.),
        Color::new(0.32, 0.64, 0.95, 1.),
    ][color as usize % 4]
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
    center_text(label, rect, 11., WHITE);
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
fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
