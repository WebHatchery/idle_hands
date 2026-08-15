//! Responsive presentation and touch routing for Lights Out.

use crate::{lights_out::LightsOutStatus, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    reset: Rect,
    undo: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(24., 48., 300., 300.),
            cell: 60.,
            reset: Rect::new(370., 125., 150., 48.),
            undo: Rect::new(370., 185., 150., 48.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(20., 145., 320., 320.),
            cell: 64.,
            reset: Rect::new(20., 510., 155., 48.),
            undo: Rect::new(185., 510., 155., 48.),
        }
    } else {
        Layout {
            board: Rect::new(390., 130., 500., 500.),
            cell: 100.,
            reset: Rect::new(440., 650., 180., 48.),
            undo: Rect::new(650., 650., 180., 48.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if back_rect().contains(point) {
        return vec![UiAction::Cabinet];
    }
    if layout.reset.contains(point) {
        return vec![UiAction::LightsOutNew];
    }
    if layout.undo.contains(point) {
        return vec![UiAction::LightsOutUndo];
    }
    if layout.board.contains(point) {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < crate::lights_out::SIZE && column < crate::lights_out::SIZE {
            let index = row * crate::lights_out::SIZE + column;
            if state.lights_out.status != LightsOutStatus::Won {
                return vec![UiAction::LightsOutPress(index)];
            }
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.lights_out;
    let header_y = header_y();
    let header_x = header_x(layout);
    let body_x = if crate::ui::is_compact_landscape() {
        layout.reset.x
    } else {
        header_x
    };
    let body_y = if crate::ui::is_compact_landscape() {
        layout.reset.y - 34.
    } else {
        header_y + 28.
    };
    text(
        "‹ CABINET",
        back_rect().x,
        back_rect().y + 20.,
        14.,
        muted(),
    );
    text("LIGHTS OUT", header_x, header_y, title_size(), accent());
    text(
        if game.status == LightsOutStatus::Won {
            "The cabinet is quiet. Start another board or play it again."
        } else {
            "Tap a light to toggle it and its four neighbors."
        },
        body_x,
        body_y,
        body_size(),
        muted(),
    );
    for index in 0..game.cells.len() {
        let row = index / crate::lights_out::SIZE;
        let column = index % crate::lights_out::SIZE;
        let rect = Rect::new(
            layout.board.x + column as f32 * layout.cell,
            layout.board.y + row as f32 * layout.cell,
            layout.cell - 3.,
            layout.cell - 3.,
        );
        let fill = if game.cells[index] {
            Color::new(0.92, 0.64, 0.28, 1.)
        } else {
            Color::new(0.12, 0.08, 0.20, 1.)
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., accent());
        if game.cells[index] {
            draw_circle(rect.center().x, rect.center().y, layout.cell * 0.15, WHITE);
        }
    }
    text(
        &format!("MOVES  {}", game.moves),
        layout.board.x,
        layout.board.bottom() + 28.,
        body_size(),
        muted(),
    );
    button(layout.reset, "NEW BOARD");
    button(layout.undo, "UNDO");
}

fn back_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(10., 8., 100., 30.)
    } else if crate::ui::is_portrait() {
        Rect::new(10., 52., 110., 30.)
    } else {
        Rect::new(48., 28., 130., 36.)
    }
}

fn header_y() -> f32 {
    if crate::ui::is_compact_landscape() {
        35.
    } else if crate::ui::is_portrait() {
        87.
    } else {
        72.
    }
}

fn header_x(layout: Layout) -> f32 {
    if crate::ui::is_compact_landscape() {
        layout.board.x + 108.
    } else {
        layout.board.x
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.14, 0.31, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., accent());
    text(
        label,
        rect.x + 16.,
        rect.y + rect.h * 0.64,
        body_size(),
        WHITE,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}

fn muted() -> Color {
    Color::new(0.76, 0.70, 0.86, 1.)
}

fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        28.
    } else {
        34.
    }
}

fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        13.
    } else {
        16.
    }
}
