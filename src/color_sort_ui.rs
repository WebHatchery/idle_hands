//! Responsive touch presentation for Color Sort.

use crate::{
    accessibility,
    color_sort::{ColorSort, ColorSortDifficulty, ColorSortPhase},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone)]
struct Layout {
    board: Rect,
    tubes: Vec<Rect>,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    difficulty: [Rect; 3],
}

fn layout(tube_count: usize) -> Layout {
    if crate::ui::is_compact_landscape() {
        let board = Rect::new(220., 54., 360., 285.);
        Layout {
            board,
            tubes: tube_rects(board, tube_count),
            hint: Rect::new(630., 220., 105., 44.),
            undo: Rect::new(630., 110., 105., 44.),
            new_game: Rect::new(630., 165., 140., 44.),
            difficulty: [
                Rect::new(610., 52., 68., 38.),
                Rect::new(682., 52., 68., 38.),
                Rect::new(754., 52., 76., 38.),
            ],
        }
    } else if crate::ui::is_portrait() {
        let board = Rect::new(15., 100., 300., 270.);
        Layout {
            board,
            tubes: tube_rects(board, tube_count),
            hint: Rect::new(15., 405., 145., 44.),
            undo: Rect::new(15., 460., 145., 44.),
            new_game: Rect::new(170., 460., 145., 44.),
            difficulty: [
                Rect::new(15., 515., 90., 38.),
                Rect::new(115., 515., 90., 38.),
                Rect::new(215., 515., 100., 38.),
            ],
        }
    } else {
        let board = Rect::new(280., 95., 540., 360.);
        Layout {
            board,
            tubes: tube_rects(board, tube_count),
            hint: Rect::new(860., 245., 120., 44.),
            undo: Rect::new(860., 190., 120., 44.),
            new_game: Rect::new(1000., 190., 145., 44.),
            difficulty: [
                Rect::new(860., 95., 85., 38.),
                Rect::new(950., 95., 85., 38.),
                Rect::new(1040., 95., 95., 38.),
            ],
        }
    }
}

fn tube_rects(board: Rect, tube_count: usize) -> Vec<Rect> {
    let gap = board.w / tube_count as f32;
    (0..tube_count)
        .map(|index| Rect::new(board.x + index as f32 * gap, board.y, gap, board.h))
        .collect()
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout(_state.color_sort.tubes.len());
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.difficulty.iter().enumerate() {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::ColorSortDifficulty(ColorSortDifficulty::ALL[index])];
        }
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
    let l = layout(state.color_sort.tubes.len());
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
    crate::ui::draw_text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    crate::ui::draw_text(
        "COLOR SORT",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    crate::ui::draw_text(
        format!(
            "{} moves  •  {}  •  {}",
            game.moves,
            status(game.phase),
            game.difficulty.label()
        ),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, &l.tubes, game, state.high_contrast);
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW BOARD", state.large_text);
    for (index, rect) in l.difficulty.iter().enumerate() {
        mode_button(
            *rect,
            ["STD", "HARD", "EXPERT"][index],
            ColorSortDifficulty::ALL[index] == game.difficulty,
            state.large_text,
        );
    }
    let status_y = if portrait {
        385.
    } else if compact {
        355.
    } else {
        480.
    };
    crate::ui::draw_text(
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

fn draw_board(board: Rect, tubes: &[Rect], game: &ColorSort, high_contrast: bool) {
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
        let slot = (rect.h - 38.) / crate::color_sort::CAPACITY as f32;
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
            Color::new(0.92, 0.30, 0.96, 1.),
            Color::new(0.98, 0.47, 0.18, 1.),
        ]
    } else {
        [
            Color::new(0.94, 0.35, 0.42, 1.),
            Color::new(0.98, 0.72, 0.28, 1.),
            Color::new(0.35, 0.82, 0.58, 1.),
            Color::new(0.32, 0.64, 0.95, 1.),
            Color::new(0.68, 0.45, 0.90, 1.),
            Color::new(0.95, 0.48, 0.34, 1.),
        ]
    };
    palette[color as usize % palette.len()]
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
        if selected { crate::theme::MOSS_DARK } else { crate::theme::SURFACE },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 2. } else { 1. },
        accent(),
    );
    center_text(label, rect, accessibility::text_size(10., large_text), WHITE);
}
fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    crate::ui::draw_text(
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
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}
