//! Responsive presentation and touch routing for Block Stack.

use crate::{
    accessibility,
    block_stack::{BlockMove, BlockStack, BlockStatus, HEIGHT, WIDTH},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Layout {
    pub board: Rect,
    pub cell: f32,
    pub left: Rect,
    pub right: Rect,
    pub rotate: Rect,
    pub drop: Rect,
    pub pause: Rect,
    pub undo: Rect,
    pub new_game: Rect,
}

pub fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let cell = 20.;
        Layout {
            board: Rect::new(10., 42., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            left: Rect::new(230., 55., 70., 40.),
            right: Rect::new(308., 55., 70., 40.),
            rotate: Rect::new(230., 102., 148., 40.),
            drop: Rect::new(230., 149., 148., 40.),
            pause: Rect::new(230., 196., 70., 36.),
            undo: Rect::new(308., 196., 70., 36.),
            new_game: Rect::new(230., 240., 148., 36.),
        }
    } else if crate::ui::is_portrait() {
        let cell = 14.;
        Layout {
            board: Rect::new(50., 100., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            left: Rect::new(20., 405., 70., 40.),
            right: Rect::new(95., 405., 70., 40.),
            rotate: Rect::new(170., 405., 70., 40.),
            drop: Rect::new(245., 405., 85., 40.),
            pause: Rect::new(20., 455., 145., 42.),
            undo: Rect::new(185., 455., 145., 42.),
            new_game: Rect::new(20., 510., 310., 42.),
        }
    } else {
        let cell = 26.;
        Layout {
            board: Rect::new(
                330.,
                100.,
                f32::from(WIDTH) * cell,
                f32::from(HEIGHT) * cell,
            ),
            cell,
            left: Rect::new(620., 135., 92., 44.),
            right: Rect::new(720., 135., 92., 44.),
            rotate: Rect::new(620., 190., 192., 44.),
            drop: Rect::new(620., 245., 192., 44.),
            pause: Rect::new(620., 305., 92., 42.),
            undo: Rect::new(720., 305., 92., 42.),
            new_game: Rect::new(620., 360., 192., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    for (rect, movement) in [
        (l.left, BlockMove::Left),
        (l.right, BlockMove::Right),
        (l.rotate, BlockMove::Rotate),
        (l.drop, BlockMove::Drop),
    ] {
        if crate::ui::hit(rect, point) {
            return vec![UiAction::BlockMove(movement)];
        }
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::BlockPause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::BlockUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::BlockNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.block_stack;
    let title_x = if crate::ui::is_compact_landscape() {
        80.
    } else if crate::ui::is_portrait() {
        10.
    } else {
        330.
    };
    let title_y = if crate::ui::is_compact_landscape() {
        27.
    } else if crate::ui::is_portrait() {
        68.
    } else {
        60.
    };
    label("‹ CABINET", 8., 30., 13., muted());
    label("BLOCK STACK", title_x, title_y, title_size(), accent());
    let score_y = if crate::ui::is_compact_landscape() {
        39.
    } else {
        title_y + 25.
    };
    label(
        &format!(
            "Lines {} / {}  •  Score {}",
            game.lines, game.target_lines, game.score
        ),
        title_x,
        score_y,
        body_size(),
        muted(),
    );
    draw_rectangle(
        l.board.x,
        l.board.y,
        l.board.w,
        l.board.h,
        accessibility::board_fill(state.high_contrast),
    );
    for row in 0..HEIGHT {
        for column in 0..WIDTH {
            let x = l.board.x + f32::from(column) * l.cell;
            let y = l.board.y + f32::from(row) * l.cell;
            draw_rectangle_lines(x, y, l.cell, l.cell, 1., Color::new(0.25, 0.22, 0.40, 0.5));
            let value = game.board[usize::from(row) * usize::from(WIDTH) + usize::from(column)];
            if value != 0 {
                draw_block(x, y, l.cell, value - 1, state.high_contrast);
            }
        }
    }
    if game.status == BlockStatus::Playing {
        for (x, y) in game.active_cells() {
            let board_x = game.piece_x + x;
            let board_y = game.piece_y + y;
            if board_x >= 0 && board_y >= 0 && board_x < WIDTH as i8 && board_y < HEIGHT as i8 {
                draw_block(
                    l.board.x + f32::from(board_x) * l.cell,
                    l.board.y + f32::from(board_y) * l.cell,
                    l.cell,
                    game.piece,
                    state.high_contrast,
                );
            }
        }
    }
    let status = status_text(game);
    label(
        state.card_hint.as_deref().unwrap_or(&status),
        title_x,
        if crate::ui::is_portrait() { 385. } else { 640. },
        body_size(),
        muted(),
    );
    button(l.left, "LEFT", state.large_text);
    button(l.right, "RIGHT", state.large_text);
    button(l.rotate, "ROTATE", state.large_text);
    button(l.drop, "DROP", state.large_text);
    button(
        l.pause,
        if game.paused { "RESUME" } else { "PAUSE" },
        state.large_text,
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW STACK", state.large_text);
}

pub fn draw_block(x: f32, y: f32, cell: f32, kind: u8, high_contrast: bool) {
    let colors = [
        Color::new(0.27, 0.85, 1., 1.),
        Color::new(1., 0.78, 0.24, 1.),
        Color::new(0.76, 0.42, 1., 1.),
        Color::new(1., 0.45, 0.24, 1.),
        Color::new(0.35, 0.92, 0.52, 1.),
        Color::new(0.95, 0.35, 0.54, 1.),
        Color::new(0.46, 0.55, 1., 1.),
    ];
    let color = if high_contrast {
        WHITE
    } else {
        colors[usize::from(kind % 7)]
    };
    draw_rectangle(x + 1., y + 1., cell - 2., cell - 2., color);
    draw_rectangle_lines(
        x + 2.,
        y + 2.,
        cell - 4.,
        cell - 4.,
        1.,
        crate::theme::SURFACE_DARK,
    );
}

pub fn status_text(game: &BlockStack) -> String {
    match game.status {
        BlockStatus::Playing => "Tap LEFT / RIGHT, ROTATE, or DROP to place the next block".into(),
        BlockStatus::Won => "Twenty lines cleared — the stack stands tall".into(),
        BlockStatus::Lost => "The stack reached the ceiling — start a new run".into(),
    }
}
pub fn button(rect: Rect, value: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(11., large_text);
    crate::ui::draw_text(
        value,
        rect.x + (rect.w - crate::ui::measure_text(value, None, size as u16, 1.).width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}
pub fn label(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        24.
    } else {
        30.
    }
}
pub fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}
pub fn accent() -> Color {
    crate::theme::BRASS
}
pub fn muted() -> Color {
    crate::theme::SECONDARY
}
pub fn back_rect() -> Rect {
    Rect::new(0., 0., 115., 42.)
}
