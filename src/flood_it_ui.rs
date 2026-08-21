//! Responsive touch presentation for Flood It.

use crate::{
    accessibility,
    flood_it::{FloodDifficulty, FloodIt, FloodPhase},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone)]
struct Layout {
    board: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    surge: Rect,
    colors: Vec<Rect>,
    difficulty: [Rect; 3],
}

fn layout(color_count: u8) -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(270., 44., 300., 300.),
            hint: Rect::new(620., 220., 105., 44.),
            undo: Rect::new(620., 110., 105., 44.),
            new_game: Rect::new(620., 165., 140., 44.),
            surge: Rect::new(735., 220., 100., 44.),
            colors: color_rects(620., 270., 42., 42., color_count as usize, 4),
            difficulty: [
                Rect::new(610., 52., 68., 38.),
                Rect::new(682., 52., 68., 38.),
                Rect::new(754., 52., 76., 38.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 95., 300., 300.),
            hint: Rect::new(15., 450., 145., 44.),
            undo: Rect::new(15., 505., 145., 44.),
            new_game: Rect::new(170., 505., 145., 44.),
            surge: Rect::new(170., 450., 145., 44.),
            colors: color_rects(15., 560., 48., 42., color_count as usize, 4),
            difficulty: [
                Rect::new(15., 675., 90., 38.),
                Rect::new(115., 675., 90., 38.),
                Rect::new(215., 675., 100., 38.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            hint: Rect::new(810., 235., 120., 44.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
            surge: Rect::new(950., 235., 140., 44.),
            colors: color_rects(810., 300., 46., 46., color_count as usize, 4),
            difficulty: [
                Rect::new(810., 90., 85., 38.),
                Rect::new(900., 90., 85., 38.),
                Rect::new(990., 90., 95., 38.),
            ],
        }
    }
}

fn color_rects(x: f32, y: f32, width: f32, height: f32, count: usize, columns: usize) -> Vec<Rect> {
    (0..count)
        .map(|index| {
            Rect::new(
                x + (index % columns) as f32 * (width + 7.),
                y + (index / columns) as f32 * (height + 7.),
                width,
                height,
            )
        })
        .collect()
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout(_state.games.flood_it.color_count());
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.difficulty.iter().enumerate() {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::FloodDifficulty(FloodDifficulty::ALL[index])];
        }
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::FloodUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::FloodHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::FloodNew];
    }
    if crate::ui::hit(l.surge, point) {
        return vec![UiAction::FloodSurge];
    }
    for (color, rect) in l.colors.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::FloodColor(color as u8)];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout(state.games.flood_it.color_count());
    let game = &state.games.flood_it;
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
        "FLOOD IT",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let flooded = game.region_size() * 100 / game.cells.len();
    let scoreline = if compact || portrait {
        format!(
            "{}% • M{}/{} • C{}",
            flooded,
            game.moves,
            game.move_limit(),
            game.combo
        )
    } else if portrait {
        format!(
            "{}% • M{}/{} • P{} • C{} • S{}",
            flooded,
            game.moves,
            game.move_limit(),
            game.points,
            game.combo,
            game.surges
        )
    } else {
        format!(
            "{}% flooded  •  Moves {}/{}  •  Points {}  •  Chain {}  •  {}",
            flooded,
            game.moves,
            game.move_limit(),
            game.points,
            game.combo,
            game.difficulty.label()
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
    for (color, rect) in l.colors.iter().enumerate() {
        draw_color(
            *rect,
            color as u8,
            game.active_color,
            game.preview_gain(color as u8),
            state.high_contrast,
        );
    }
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW FIELD", state.large_text);
    button(l.surge, &format!("SURGE {}", game.surges), state.large_text);
    for (index, rect) in l.difficulty.iter().enumerate() {
        mode_button(
            *rect,
            ["STD", "HARD", "EXPERT"][index],
            FloodDifficulty::ALL[index] == game.difficulty,
            state.large_text,
        );
    }
    let status_y = if portrait {
        420.
    } else if compact {
        365.
    } else {
        545.
    };
    crate::ui::draw_text(
        state.card_hint.as_deref().unwrap_or(status_text(game)),
        if compact { 270. } else { title_x },
        status_y,
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
}

fn draw_board(board: Rect, game: &FloodIt, high_contrast: bool) {
    let side = game.side();
    let cell = board.w / side as f32;
    let region = game.region_mask();
    for (index, &in_region) in region.iter().enumerate().take(side * side) {
        let rect = Rect::new(
            board.x + (index % side) as f32 * cell,
            board.y + (index / side) as f32 * cell,
            cell,
            cell,
        );
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            palette(game.cells[index], high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if in_region { 2. } else { 1. },
            if in_region {
                WHITE
            } else {
                accessibility::grid_line(high_contrast)
            },
        );
    }
}

fn draw_color(rect: Rect, color: u8, active: u8, gain: usize, high_contrast: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        palette(color, high_contrast),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if color == active { 3. } else { 1. },
        if color == active {
            WHITE
        } else {
            line_color(high_contrast)
        },
    );
    center_text(
        if color == active {
            "NOW".into()
        } else {
            format!("+{}", gain)
        }
        .as_str(),
        rect,
        10.,
        crate::theme::BACKGROUND,
    );
}

fn status_text(game: &FloodIt) -> &'static str {
    match game.phase {
        FloodPhase::Playing if game.surges > 0 => "Tap SURGE for the best forecast without a move",
        FloodPhase::Playing => "Button numbers forecast how many cells will join",
        FloodPhase::Won => "The field is one color",
        FloodPhase::Lost => "Move limit reached • Tap UNDO or NEW FIELD",
    }
}

fn palette(color: u8, high_contrast: bool) -> Color {
    let palette = if high_contrast {
        [
            Color::new(1., 0.18, 0.22, 1.),
            Color::new(1., 0.82, 0.05, 1.),
            Color::new(0.05, 0.95, 0.30, 1.),
            Color::new(0.05, 0.60, 1., 1.),
            Color::new(0.95, 0.20, 1., 1.),
            Color::new(1., 0.35, 0.75, 1.),
            Color::new(0.25, 1., 0.90, 1.),
            Color::new(0.98, 0.40, 0.08, 1.),
        ]
    } else {
        [
            Color::new(0.94, 0.35, 0.42, 1.),
            Color::new(0.98, 0.72, 0.28, 1.),
            Color::new(0.35, 0.82, 0.58, 1.),
            Color::new(0.32, 0.64, 0.95, 1.),
            Color::new(0.68, 0.45, 0.90, 1.),
            Color::new(0.95, 0.48, 0.72, 1.),
            Color::new(0.28, 0.78, 0.76, 1.),
            Color::new(0.90, 0.42, 0.24, 1.),
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
    center_text(
        label,
        rect,
        accessibility::text_size(10., large_text),
        WHITE,
    );
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
