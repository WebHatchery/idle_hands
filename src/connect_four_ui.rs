//! Responsive presentation and touch routing for Connect Four.

use crate::{
    connect_four::{ConnectFourStatus, Disc},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    drops: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(10., 48., 350., 300.),
            cell: 50.,
            drops: Rect::new(10., 350., 350., 34.),
            undo: Rect::new(430., 210., 120., 42.),
            new_game: Rect::new(570., 210., 150., 42.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 120., 340., 292.),
            cell: 48.5714,
            drops: Rect::new(10., 420., 340., 38.),
            undo: Rect::new(20., 520., 145., 42.),
            new_game: Rect::new(185., 520., 165., 42.),
        }
    } else {
        Layout {
            board: Rect::new(350., 105., 560., 480.),
            cell: 80.,
            drops: Rect::new(350., 600., 560., 42.),
            undo: Rect::new(950., 600., 120., 44.),
            new_game: Rect::new(1090., 600., 150., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.undo.contains(point) {
        return vec![UiAction::ConnectFourUndo];
    }
    if layout.new_game.contains(point) {
        return vec![UiAction::ConnectFourNew];
    }
    if layout.drops.contains(point) || layout.board.contains(point) {
        let column = ((point.x - layout.board.x) / layout.cell).clamp(0., 6.99) as usize;
        if column < 7 && state.connect_four.status == ConnectFourStatus::Playing {
            return vec![UiAction::ConnectFourDrop(column)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.connect_four;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let header_x = if compact {
        120.
    } else if portrait {
        10.
    } else {
        350.
    };
    let header_y = if compact {
        30.
    } else if portrait {
        78.
    } else {
        72.
    };
    text("‹ CABINET", 8., 30., 13., muted());
    text("CONNECT FOUR", header_x, header_y, title_size(), accent());
    text(
        status_text(game.status),
        if compact { 430. } else { header_x },
        if compact { 30. } else { header_y + 25. },
        body_size(),
        muted(),
    );
    draw_rectangle(
        layout.board.x,
        layout.board.y,
        layout.board.w,
        layout.board.h,
        Color::new(0.12, 0.16, 0.30, 1.),
    );
    for row in 0..6 {
        for column in 0..7 {
            let index = row * 7 + column;
            let center = vec2(
                layout.board.x + column as f32 * layout.cell + layout.cell / 2.,
                layout.board.y + row as f32 * layout.cell + layout.cell / 2.,
            );
            draw_circle(
                center.x,
                center.y,
                layout.cell * 0.35,
                disc_color(game.cells[index]),
            );
            draw_circle_lines(
                center.x,
                center.y,
                layout.cell * 0.35,
                1.,
                Color::new(0.45, 0.38, 0.65, 1.),
            );
        }
    }
    for column in 0..7 {
        let rect = Rect::new(
            layout.drops.x + column as f32 * layout.drops.w / 7.,
            layout.drops.y,
            layout.drops.w / 7. - 4.,
            layout.drops.h,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.20, 0.13, 0.30, 1.),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
        text(
            &format!("{}", column + 1),
            rect.x + rect.w * 0.45,
            rect.y + rect.h * 0.68,
            12.,
            WHITE,
        );
    }
    text(
        "DROP A DISC",
        layout.drops.x,
        layout.drops.y - 8.,
        10.,
        muted(),
    );
    text(
        &format!("Moves {}  •  Red is you  •  Yellow answers", game.moves),
        if compact {
            430.
        } else if portrait {
            10.
        } else {
            350.
        },
        if compact {
            150.
        } else if portrait {
            485.
        } else {
            650.
        },
        body_size(),
        muted(),
    );
    button(layout.undo, "UNDO");
    button(layout.new_game, "NEW BOARD");
}

fn disc_color(disc: Disc) -> Color {
    match disc {
        Disc::Empty => Color::new(0.04, 0.04, 0.09, 1.),
        Disc::Red => Color::new(0.90, 0.30, 0.35, 1.),
        Disc::Yellow => Color::new(0.98, 0.75, 0.30, 1.),
    }
}
fn status_text(status: ConnectFourStatus) -> &'static str {
    match status {
        ConnectFourStatus::Playing => "Drop four in a row",
        ConnectFourStatus::Won(Disc::Red) => "Red takes the row",
        ConnectFourStatus::Won(Disc::Yellow) => "Yellow takes the row",
        ConnectFourStatus::Won(Disc::Empty) => "The board is quiet",
        ConnectFourStatus::Draw => "The board is full",
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
