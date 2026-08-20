//! Responsive presentation and touch routing for Lights Out.

use crate::{
    accessibility,
    lights_out::{LightsDifficulty, LightsOutStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    cell: f32,
    reset: Rect,
    undo: Rect,
    hint: Rect,
    guide: Rect,
    difficulty: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(24., 48., 300., 300.),
            cell: 60.,
            reset: Rect::new(370., 125., 150., 48.),
            undo: Rect::new(370., 185., 150., 48.),
            hint: Rect::new(530., 185., 150., 48.),
            guide: Rect::new(530., 125., 150., 48.),
            difficulty: Rect::new(370., 245., 310., 48.),
        }
    } else if crate::ui::is_portrait() {
        let width = screen_width().min(370.);
        let height = screen_height();
        let side = (width - 20.).min(if height < 650. { 200. } else { 320. });
        let top = if height < 650. { 100. } else { 145. };
        let controls_y = top + side + 42.;
        let control_w = (width - 32.) / 3.;
        Layout {
            board: Rect::new(10., top, side, side),
            cell: side / 5.,
            reset: Rect::new(10., controls_y, control_w, 44.),
            undo: Rect::new(16. + control_w, controls_y, control_w, 44.),
            hint: Rect::new(22. + control_w * 2., controls_y, control_w, 44.),
            guide: Rect::new(10., controls_y + 52., control_w, 44.),
            difficulty: Rect::new(16. + control_w, controls_y + 52., control_w * 2. + 6., 44.),
        }
    } else {
        Layout {
            board: Rect::new(390., 130., 500., 500.),
            cell: 100.,
            reset: Rect::new(920., 180., 190., 48.),
            undo: Rect::new(920., 240., 190., 48.),
            hint: Rect::new(920., 300., 190., 48.),
            guide: Rect::new(920., 360., 190., 48.),
            difficulty: Rect::new(920., 420., 190., 48.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let layout = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(layout.reset, point) {
        return vec![UiAction::LightsOutNew];
    }
    if crate::ui::hit(layout.undo, point) {
        return vec![UiAction::LightsOutUndo];
    }
    if crate::ui::hit(layout.hint, point) {
        return vec![UiAction::LightsOutHint];
    }
    if crate::ui::hit(layout.guide, point) {
        return vec![UiAction::LightsOutGuide];
    }
    if crate::ui::hit(layout.difficulty, point) {
        return vec![UiAction::LightsOutDifficulty(
            match state.games.lights_out.difficulty {
                LightsDifficulty::Classic => LightsDifficulty::Dense,
                LightsDifficulty::Dense => LightsDifficulty::Classic,
            },
        )];
    }
    if layout.board.contains(point) {
        let column = ((point.x - layout.board.x) / layout.cell) as usize;
        let row = ((point.y - layout.board.y) / layout.cell) as usize;
        if row < crate::lights_out::SIZE && column < crate::lights_out::SIZE {
            let index = row * crate::lights_out::SIZE + column;
            if state.games.lights_out.status != LightsOutStatus::Won {
                return vec![UiAction::LightsOutPress(index)];
            }
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let layout = layout();
    let game = &state.games.lights_out;
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
        accessibility::text_size(14., state.large_text),
        muted(),
    );
    text(
        "LIGHTS OUT",
        header_x,
        header_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let instruction = if game.status == LightsOutStatus::Won {
        "The cabinet is settled. Start another board or play it again."
    } else {
        "Tap a light to toggle it and its four neighbors."
    };
    text(
        state.card_hint.as_deref().unwrap_or(instruction),
        body_x,
        body_y,
        accessibility::text_size(body_size(), state.large_text),
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
            if state.high_contrast {
                Color::new(1., 0.85, 0.05, 1.)
            } else {
                Color::new(0.92, 0.64, 0.28, 1.)
            }
        } else {
            accessibility::board_fill(state.high_contrast)
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.,
            accessibility::grid_line(state.high_contrast),
        );
        if game.optimal_contains(index) {
            draw_rectangle_lines(
                rect.x + 3.,
                rect.y + 3.,
                rect.w - 6.,
                rect.h - 6.,
                4.,
                guide_color(),
            );
        }
        if game.cells[index] {
            draw_circle(rect.center().x, rect.center().y, layout.cell * 0.15, WHITE);
        }
    }
    text(
        &format!(
            "MOVES {} • PAR {} • {} LIT • {} LEFT",
            game.moves,
            game.displayed_par(),
            game.lit_count(),
            game.minimum_solution().len()
        ),
        layout.board.x,
        layout.board.bottom() + 25.,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(layout.reset, "NEW BOARD", state.large_text);
    button(layout.undo, "UNDO", state.large_text);
    button(layout.hint, "HINT", state.large_text);
    active_button(layout.guide, "GUIDE", game.guide, state.large_text);
    button(layout.difficulty, game.difficulty.label(), state.large_text);
}

fn back_rect() -> Rect {
    if crate::ui::is_compact_landscape() {
        Rect::new(10., 8., 100., 30.)
    } else if crate::ui::is_portrait() {
        Rect::new(0., 0., 110., 42.)
    } else {
        Rect::new(48., 28., 130., 36.)
    }
}

fn header_y() -> f32 {
    if crate::ui::is_compact_landscape() {
        35.
    } else if crate::ui::is_portrait() {
        if screen_height() < 650. {
            55.
        } else {
            105.
        }
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

fn button(rect: Rect, label: &str, large_text: bool) {
    active_button(rect, label, false, large_text);
}

fn active_button(rect: Rect, label: &str, active: bool, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if active { 4. } else { 2. },
        if active { guide_color() } else { accent() },
    );
    text(
        label,
        rect.x + 16.,
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

fn guide_color() -> Color {
    Color::from_rgba(80, 224, 126, 255)
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
