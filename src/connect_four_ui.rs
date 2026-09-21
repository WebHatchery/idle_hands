//! Responsive presentation and touch routing for Connect Four.

use crate::{
    accessibility,
    connect_four::{AiLevel, ConnectFourStatus, Disc},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Layout {
    pub board: Rect,
    pub cell: f32,
    pub drops: Rect,
    pub hint: Rect,
    pub undo: Rect,
    pub new_game: Rect,
    pub levels: [Rect; 3],
}

pub fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(10., 48., 350., 300.),
            cell: 50.,
            drops: Rect::new(10., 350., 350., 34.),
            hint: Rect::new(430., 265., 290., 42.),
            undo: Rect::new(430., 210., 120., 42.),
            new_game: Rect::new(570., 210., 150., 42.),
            levels: [
                Rect::new(430., 65., 90., 42.),
                Rect::new(530., 65., 90., 42.),
                Rect::new(630., 65., 90., 42.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(10., 120., 340., 292.),
            cell: 48.5714,
            drops: Rect::new(10., 420., 340., 38.),
            hint: Rect::new(20., 575., 330., 42.),
            undo: Rect::new(20., 520., 145., 42.),
            new_game: Rect::new(185., 520., 165., 42.),
            levels: [
                Rect::new(10., 468., 105., 42.),
                Rect::new(120., 468., 105., 42.),
                Rect::new(230., 468., 105., 42.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(350., 105., 560., 480.),
            cell: 80.,
            drops: Rect::new(350., 600., 560., 42.),
            hint: Rect::new(950., 540., 290., 44.),
            undo: Rect::new(950., 600., 120., 44.),
            new_game: Rect::new(1090., 600., 150., 44.),
            levels: [
                Rect::new(950., 105., 90., 42.),
                Rect::new(1050., 105., 90., 42.),
                Rect::new(1150., 105., 90., 42.),
            ],
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(layout.undo, point) {
        return vec![UiAction::ConnectFourUndo];
    }
    if crate::ui::hit(layout.new_game, point) {
        return vec![UiAction::ConnectFourNew];
    }
    if crate::ui::hit(layout.hint, point) {
        return vec![UiAction::ConnectFourHint];
    }
    for (index, level) in [AiLevel::Gentle, AiLevel::Sharp, AiLevel::Expert]
        .into_iter()
        .enumerate()
    {
        if crate::ui::hit(layout.levels[index], point) {
            return vec![UiAction::ConnectFourLevel(level)];
        }
    }
    if layout.drops.contains(point) || layout.board.contains(point) {
        let column = ((point.x - layout.board.x) / layout.cell).clamp(0., 6.99) as usize;
        if column < 7 && state.games.connect_four.status == ConnectFourStatus::Playing {
            return vec![UiAction::ConnectFourDrop(column)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.games.connect_four;
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
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "CONNECT FOUR",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let instruction = if compact {
        "Tap a column"
    } else {
        state
            .card_hint
            .as_deref()
            .unwrap_or(status_text(game.status))
    };
    let instruction_position = if compact {
        compact_instruction_position()
    } else {
        vec2(header_x, header_y + 25.)
    };
    text(
        instruction,
        instruction_position.x,
        instruction_position.y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_rectangle(
        layout.board.x,
        layout.board.y,
        layout.board.w,
        layout.board.h,
        if state.high_contrast {
            Color::new(0.04, 0.12, 0.28, 1.)
        } else {
            Color::new(0.12, 0.16, 0.30, 1.)
        },
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
                disc_color(game.cells[index], state.high_contrast),
            );
            draw_circle_lines(
                center.x,
                center.y,
                layout.cell * 0.35,
                1.,
                accessibility::grid_line(state.high_contrast),
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
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
        text(
            &format!("{}", column + 1),
            rect.x + rect.w * 0.45,
            rect.y + rect.h * 0.68,
            accessibility::text_size(12., state.large_text),
            WHITE,
        );
    }
    if show_drop_prompt(portrait, compact) {
        text(
            "DROP A DISC",
            layout.drops.x,
            layout.drops.y - 8.,
            accessibility::text_size(10., state.large_text),
            muted(),
        );
    }
    text(
        &format!("Moves {}  •  Red is you  •  Yellow answers", game.moves),
        if compact {
            430.
        } else if portrait {
            10.
        } else {
            350.
        },
        moves_summary_y(compact, portrait),
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(layout.undo, "UNDO", state.large_text);
    button(layout.new_game, "NEW BOARD", state.large_text);
    button(layout.hint, "HINT", state.large_text);
    for (rect, (label, level)) in layout.levels.iter().zip([
        ("GENTLE", AiLevel::Gentle),
        ("SHARP", AiLevel::Sharp),
        ("EXPERT", AiLevel::Expert),
    ]) {
        button_selected(*rect, label, game.ai_level == level, state.large_text);
    }
}

pub fn disc_color(disc: Disc, high_contrast: bool) -> Color {
    match disc {
        Disc::Empty => accessibility::board_fill(high_contrast),
        Disc::Red => {
            if high_contrast {
                Color::new(1., 0.12, 0.18, 1.)
            } else {
                Color::new(0.90, 0.30, 0.35, 1.)
            }
        }
        Disc::Yellow => {
            if high_contrast {
                Color::new(1., 0.85, 0.05, 1.)
            } else {
                crate::theme::BRASS
            }
        }
    }
}
pub fn status_text(status: ConnectFourStatus) -> &'static str {
    match status {
        ConnectFourStatus::Playing => "Drop four in a row",
        ConnectFourStatus::Won(Disc::Red) => "Red takes the row",
        ConnectFourStatus::Won(Disc::Yellow) => "Yellow takes the row",
        ConnectFourStatus::Won(Disc::Empty) => "The board is settled",
        ConnectFourStatus::Draw => "The board is full",
    }
}
pub fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 12.,
        rect.y + 28.,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
pub fn button_selected(rect: Rect, label: &str, selected: bool, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::new(0.45, 0.25, 0.42, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 10.,
        rect.y + 28.,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}
pub fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        22.
    } else {
        31.
    }
}
pub fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
pub fn accent() -> Color {
    crate::theme::BRASS
}
pub fn muted() -> Color {
    crate::theme::SECONDARY
}

pub fn show_drop_prompt(portrait: bool, compact: bool) -> bool {
    !portrait && !compact
}

pub fn compact_instruction_position() -> Vec2 {
    vec2(430., 45.)
}

pub fn moves_summary_y(compact: bool, portrait: bool) -> f32 {
    if compact {
        150.
    } else if portrait {
        485.
    } else {
        665.
    }
}

pub fn back_rect() -> Rect {
    Rect::new(0., 0., 110., 42.)
}
