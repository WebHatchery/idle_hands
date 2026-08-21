//! Responsive touch presentation for Pipe Loop.

use crate::{
    accessibility,
    pipe_loop::{PipeLoop, PipePattern, PipePhase, SIDE},
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
    pattern: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(260., 44., 300., 300.),
            hint: Rect::new(620., 220., 105., 44.),
            undo: Rect::new(620., 110., 105., 44.),
            new_game: Rect::new(620., 165., 140., 44.),
            pattern: Rect::new(735., 220., 95., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 95., 300., 300.),
            hint: Rect::new(15., 440., 145., 44.),
            undo: Rect::new(15., 495., 145., 44.),
            new_game: Rect::new(170., 495., 145., 44.),
            pattern: Rect::new(170., 440., 145., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            hint: Rect::new(810., 245., 120., 44.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
            pattern: Rect::new(950., 245., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    if l.board.contains(point) {
        let cell = l.board.w / SIDE as f32;
        let col = ((point.x - l.board.x) / cell) as usize;
        let row = ((point.y - l.board.y) / cell) as usize;
        if row < SIDE && col < SIDE {
            return vec![UiAction::PipeRotate(row * SIDE + col)];
        }
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::PipeHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::PipeUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::PipeNew];
    }
    if crate::ui::hit(l.pattern, point) {
        return vec![UiAction::PipePattern(
            match _state.games.pipe_loop.pattern {
                PipePattern::Serpent => PipePattern::Trunk,
                PipePattern::Trunk => PipePattern::Serpent,
            },
        )];
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.pipe_loop;
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
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    crate::ui::draw_text(
        "PIPE LOOP",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let scoreline = if compact || crate::ui::display_width() < 360. {
        format!(
            "P{} • L{} • M{}",
            game.connected_count(),
            game.leak_count(),
            game.moves
        )
    } else {
        format!(
            "Powered {}/25  •  Leaks {}  •  Rotations {}/{}  •  {}",
            game.connected_count(),
            game.leak_count(),
            game.moves,
            game.par,
            game.pattern.label()
        )
    };
    crate::ui::draw_text(
        scoreline,
        if compact {
            crate::ui::COMPACT_HEADER_STATUS_X
        } else {
            title_x
        },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast);
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW LOOP", state.large_text);
    mode_button(l.pattern, game.pattern.label(), state.large_text);
    let status_y = if portrait {
        420.
    } else if compact {
        365.
    } else {
        545.
    };
    crate::ui::draw_text(
        state.card_hint.as_deref().unwrap_or(
            if crate::ui::display_width() < 360. && game.phase == PipePhase::Playing {
                "Close red leaks to power all 25"
            } else {
                status(game.phase)
            },
        ),
        if compact { 260. } else { title_x },
        status_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
}

fn draw_board(board: Rect, game: &PipeLoop, high_contrast: bool) {
    let cell = board.w / SIDE as f32;
    let powered = game.powered_mask();
    for (index, &is_powered) in powered.iter().enumerate().take(SIDE * SIDE) {
        let rect = Rect::new(
            board.x + (index % SIDE) as f32 * cell,
            board.y + (index / SIDE) as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            accessibility::board_fill(high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            line_color(high_contrast),
        );
        let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        draw_circle(
            center.x,
            center.y,
            cell * 0.10,
            if is_powered { accent() } else { muted() },
        );
        let mask = game.pipes[index];
        let endpoints = [
            (mask & 1 != 0, vec2(center.x, rect.y)),
            (mask & 2 != 0, vec2(rect.right(), center.y)),
            (mask & 4 != 0, vec2(center.x, rect.bottom())),
            (mask & 8 != 0, vec2(rect.x, center.y)),
        ];
        for (side, (connected, endpoint)) in endpoints.into_iter().enumerate() {
            if connected {
                draw_line(
                    center.x,
                    center.y,
                    endpoint.x,
                    endpoint.y,
                    cell * 0.12,
                    if powered[index] {
                        pipe_color(high_contrast)
                    } else {
                        muted()
                    },
                );
                let bit = [1, 2, 4, 8][side];
                if game.has_leak(index, bit) {
                    draw_circle(
                        endpoint.x.clamp(rect.x + 4., rect.right() - 4.),
                        endpoint.y.clamp(rect.y + 4., rect.bottom() - 4.),
                        cell * 0.055,
                        Color::new(0.95, 0.25, 0.24, 1.),
                    );
                }
            }
        }
    }
}

fn status(phase: PipePhase) -> &'static str {
    match phase {
        PipePhase::Playing => "Rotate tiles until all 25 are powered and every red leak closes",
        PipePhase::Won => "Network complete — tap NEW LOOP",
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
fn pipe_color(high_contrast: bool) -> Color {
    if high_contrast {
        Color::new(0.05, 1., 0.85, 1.)
    } else {
        Color::new(0.35, 0.82, 0.70, 1.)
    }
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}
