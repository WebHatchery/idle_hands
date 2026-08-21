//! Responsive presentation and touch routing for Potion 2048.

use crate::domain::Direction;
use crate::{accessibility, potion_2048::PotionDifficulty, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    arrows: [Rect; 4],
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    difficulty: [Rect; 3],
}
fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(300., 65., 280., 280.),
            arrows: [
                Rect::new(610., 70., 52., 44.),
                Rect::new(610., 120., 52., 44.),
                Rect::new(610., 170., 52., 44.),
                Rect::new(610., 220., 52., 44.),
            ],
            hint: Rect::new(680., 275., 105., 44.),
            undo: Rect::new(680., 155., 105., 44.),
            new_game: Rect::new(680., 210., 130., 44.),
            difficulty: [
                Rect::new(80., 55., 65., 44.),
                Rect::new(150., 55., 65., 44.),
                Rect::new(220., 55., 75., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(50., 150., 300., 300.),
            arrows: [
                Rect::new(20., 480., 74., 44.),
                Rect::new(102., 480., 74., 44.),
                Rect::new(184., 480., 74., 44.),
                Rect::new(266., 480., 74., 44.),
            ],
            hint: Rect::new(20., 615., 145., 44.),
            undo: Rect::new(20., 555., 145., 44.),
            new_game: Rect::new(185., 555., 155., 44.),
            difficulty: [
                Rect::new(20., 100., 100., 44.),
                Rect::new(130., 100., 100., 44.),
                Rect::new(240., 100., 100., 44.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(360., 115., 420., 420.),
            arrows: [
                Rect::new(840., 150., 62., 46.),
                Rect::new(910., 150., 62., 46.),
                Rect::new(980., 150., 62., 46.),
                Rect::new(1050., 150., 62., 46.),
            ],
            hint: Rect::new(840., 310., 120., 44.),
            undo: Rect::new(840., 250., 120., 44.),
            new_game: Rect::new(980., 250., 150., 44.),
            difficulty: [
                Rect::new(840., 95., 85., 44.),
                Rect::new(930., 95., 85., 44.),
                Rect::new(1020., 95., 95., 44.),
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
            return vec![UiAction::PotionDifficulty(PotionDifficulty::ALL[index])];
        }
    }
    for (index, rect) in l.arrows.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::PotionMove(
                [
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                ][index],
            )];
        }
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::PotionUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::PotionHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::PotionNew];
    }
    vec![]
}
pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.potion_2048;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let x = if compact {
        80.
    } else if portrait {
        10.
    } else {
        400.
    };
    let y = if compact {
        30.
    } else if portrait {
        68.
    } else {
        60.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "POTION 2048",
        x,
        y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let brew_status = if let Some(hint) = state.card_hint.as_deref() {
        hint.to_string()
    } else if compact {
        format!(
            "S{}  •  G{}  •  Chain {}/{}  •  C{}",
            game.score,
            game.target(),
            game.combo,
            game.difficulty.catalyst_chain(),
            game.catalysts_brewed
        )
    } else if portrait {
        portrait_brew_status(
            game.score,
            game.best,
            game.target() as u32,
            game.catalysts_brewed as u32,
        )
    } else {
        format!(
            "Score {}  •  Best {}  •  Goal {}  •  {}  •  Chain {}/{}  •  Catalysts {}",
            game.score,
            game.best,
            game.target(),
            game.difficulty.label(),
            game.combo,
            game.difficulty.catalyst_chain(),
            game.catalysts_brewed
        )
    };
    text(
        &brew_status,
        if compact { 80. } else { x },
        if compact { 120. } else { y + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    let grid = crate::grid::GridLayout::new(l.board, game.side(), game.side());
    for index in 0..game.cells.len() {
        let rect = grid.cell_rect(index).unwrap();
        let value = game.cells[index];
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            tile_color(value, state.high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.,
            accessibility::grid_line(state.high_contrast),
        );
        if value == 1 {
            let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
            let radius = rect.w.min(rect.h) * 0.28;
            draw_triangle(
                vec2(center.x, center.y - radius),
                vec2(center.x + radius, center.y),
                vec2(center.x, center.y + radius),
                accent(),
            );
            draw_triangle(
                vec2(center.x, center.y - radius),
                vec2(center.x - radius, center.y),
                vec2(center.x, center.y + radius),
                accent(),
            );
            let size = accessibility::text_size(12., state.large_text);
            text(
                "C",
                center.x - crate::ui::measure_text("C", None, size as u16, 1.).width * 0.5,
                center.y + size * 0.35,
                size,
                crate::theme::INK,
            );
        } else if value > 0 {
            let label = value.to_string();
            let size =
                accessibility::text_size(if value < 100 { 29. } else { 21. }, state.large_text);
            text(
                &label,
                rect.x + rect.w * 0.5
                    - crate::ui::measure_text(&label, None, size as u16, 1.).width * 0.5,
                rect.y + rect.h * 0.60,
                size,
                WHITE,
            );
        }
    }
    for (index, rect) in l.arrows.iter().enumerate() {
        button(
            *rect,
            ["UP", "LEFT", "DOWN", "RIGHT"][index],
            state.large_text,
        );
    }
    button(l.undo, "UNDO", state.large_text);
    button(l.hint, "HINT", state.large_text);
    button(l.new_game, "NEW BREW", state.large_text);
    for (index, rect) in l.difficulty.iter().enumerate() {
        mode_button(
            *rect,
            ["STD", "HARD", "EXPERT"][index],
            PotionDifficulty::ALL[index] == game.difficulty,
            state.large_text,
        );
    }
}

fn portrait_brew_status(score: u32, best: u32, target: u32, catalysts: u32) -> String {
    format!(
        "S{}  •  B{}  •  G{}  •  C{}",
        score, best, target, catalysts
    )
}
fn tile_color(value: u16, high_contrast: bool) -> Color {
    if high_contrast {
        return match value {
            0 => accessibility::board_fill(true),
            1 => Color::new(1., 0.72, 0.08, 1.),
            2 => Color::new(0.05, 0.42, 0.80, 1.),
            4 => Color::new(0.05, 0.75, 0.65, 1.),
            8 => Color::new(0.10, 0.85, 0.25, 1.),
            16 => Color::new(0.95, 0.65, 0.05, 1.),
            32 => Color::new(1., 0.15, 0.20, 1.),
            _ => Color::new(0.85, 0.20, 1., 1.),
        };
    }
    match value {
        0 => Color::new(0.10, 0.07, 0.16, 1.),
        1 => Color::new(0.22, 0.16, 0.30, 1.),
        2 => Color::new(0.22, 0.30, 0.35, 1.),
        4 => Color::new(0.25, 0.40, 0.38, 1.),
        8 => Color::new(0.34, 0.45, 0.25, 1.),
        16 => Color::new(0.48, 0.40, 0.20, 1.),
        32 => Color::new(0.50, 0.25, 0.25, 1.),
        _ => Color::new(0.36, 0.22, 0.48, 1.),
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(11., large_text);
    text(
        label,
        rect.x + rect.w * 0.5 - crate::ui::measure_text(label, None, size as u16, 1.).width * 0.5,
        rect.y + rect.h * 0.63,
        size,
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
    text(
        label,
        rect.x + rect.w * 0.5 - crate::ui::measure_text(label, None, 10, 1.).width * 0.5,
        rect.y + rect.h * 0.63,
        accessibility::text_size(10., large_text),
        WHITE,
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        23.
    } else {
        29.
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
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
