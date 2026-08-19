//! Responsive touch presentation for Number Match.

use crate::{
    accessibility,
    number_match::{LinkRule, NumberMatch, NumberMatchPhase, SIDE},
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
    remix: Rect,
    rules: [Rect; 3],
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(270., 44., 300., 300.),
            hint: Rect::new(620., 220., 105., 44.),
            undo: Rect::new(620., 110., 105., 44.),
            new_game: Rect::new(620., 165., 140., 44.),
            remix: Rect::new(620., 275., 140., 44.),
            rules: [
                Rect::new(610., 50., 70., 44.),
                Rect::new(685., 50., 70., 44.),
                Rect::new(760., 50., 78., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(15., 105., 300., 300.),
            hint: Rect::new(15., 530., 145., 44.),
            undo: Rect::new(15., 475., 145., 44.),
            new_game: Rect::new(170., 475., 145., 44.),
            remix: Rect::new(170., 530., 145., 44.),
            rules: [
                Rect::new(15., 420., 90., 44.),
                Rect::new(112., 420., 90., 44.),
                Rect::new(209., 420., 90., 44.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(350., 90., 420., 420.),
            hint: Rect::new(810., 245., 120., 44.),
            undo: Rect::new(810., 180., 120., 44.),
            new_game: Rect::new(950., 180., 140., 44.),
            remix: Rect::new(950., 245., 140., 44.),
            rules: [
                Rect::new(810., 115., 90., 44.),
                Rect::new(910., 115., 90., 44.),
                Rect::new(1010., 115., 100., 44.),
            ],
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    let cell = l.board.w / SIDE as f32;
    if l.board.contains(point) {
        let col = ((point.x - l.board.x) / cell) as usize;
        let row = ((point.y - l.board.y) / cell) as usize;
        if row < SIDE && col < SIDE {
            return vec![UiAction::NumberMatchTap(row * SIDE + col)];
        }
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::NumberMatchHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::NumberMatchUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::NumberMatchNew];
    }
    if crate::ui::hit(l.remix, point) {
        return vec![UiAction::NumberMatchRemix];
    }
    for (index, rule) in [LinkRule::Neighbors, LinkRule::Lines, LinkRule::Diagonals]
        .into_iter()
        .enumerate()
    {
        if crate::ui::hit(l.rules[index], point) {
            return vec![UiAction::NumberMatchRule(rule)];
        }
    }
    Vec::new()
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.number_match;
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
    text(
        "‹ CABINET",
        8.,
        30.,
        accessibility::text_size(13., state.large_text),
        muted(),
    );
    text(
        "NUMBER MATCH",
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let scoreline = if compact {
        format!(
            "P{} • Pts{} • C{} • {}",
            game.score,
            game.points,
            game.combo,
            rule_label(game.rule)
        )
    } else {
        format!(
            "Pairs {}  •  Points {}  •  Chain {}  •  {}",
            game.score,
            game.points,
            game.combo,
            if game.won() {
                "GRID CLEAR"
            } else if game.phase == NumberMatchPhase::Stuck {
                "NO LINKS"
            } else {
                "PAIR THE NUMBERS"
            }
        )
    };
    text(
        &scoreline,
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    draw_board(l.board, game, state.high_contrast, state.large_text);
    text(
        state.card_hint.as_deref().unwrap_or(status_text(game)),
        if compact { 270. } else { title_x },
        if portrait {
            600.
        } else if compact {
            365.
        } else {
            545.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW BOARD", state.large_text);
    button(
        l.remix,
        &format!("REMIX {}", game.remixes_left),
        state.large_text,
    );
    for (rect, (label, rule)) in l.rules.iter().zip([
        ("NEAR", LinkRule::Neighbors),
        ("LINES", LinkRule::Lines),
        ("DIAGONAL", LinkRule::Diagonals),
    ]) {
        button_selected(*rect, label, game.rule == rule, state.large_text);
    }
}

fn draw_board(board: Rect, game: &NumberMatch, high_contrast: bool, large_text: bool) {
    let cell = board.w / SIDE as f32;
    for row in 0..SIDE {
        for col in 0..SIDE {
            let index = row * SIDE + col;
            let rect = Rect::new(
                board.x + col as f32 * cell,
                board.y + row as f32 * cell,
                cell,
                cell,
            );
            let selected = game.selected == Some(index);
            let linked = game
                .selected
                .is_some_and(|first| game.can_pair(first, index));
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if game.cells[index] == 0 {
                    accessibility::board_fill(high_contrast)
                } else if selected {
                    if high_contrast {
                        Color::new(0.45, 0.35, 0.55, 1.)
                    } else {
                        Color::new(0.35, 0.25, 0.45, 1.)
                    }
                } else {
                    if high_contrast {
                        Color::new(0.12, 0.10, 0.16, 1.)
                    } else {
                        Color::new(0.18, 0.13, 0.27, 1.)
                    }
                },
            );
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if linked { 3. } else { 1. },
                if selected {
                    accent()
                } else if linked {
                    Color::new(0.45, 0.90, 0.58, 1.)
                } else {
                    line_color(high_contrast)
                },
            );
            if game.cells[index] != 0 {
                center_text(
                    &game.cells[index].to_string(),
                    rect,
                    accessibility::text_size(if portrait_size() { 16. } else { 23. }, large_text),
                    WHITE,
                );
            }
        }
    }
}

fn status_text(game: &NumberMatch) -> &'static str {
    match game.phase {
        NumberMatchPhase::Playing => match game.rule {
            LinkRule::Neighbors => "Tap neighboring equal or sum-to-ten numbers",
            LinkRule::Lines => "Pair through clear rows or columns",
            LinkRule::Diagonals => "Pair through clear rows, columns, or diagonals",
        },
        NumberMatchPhase::Won => "Every number has found its pair",
        NumberMatchPhase::Stuck => "No links remain • Tap UNDO or REMIX",
    }
}

fn rule_label(rule: LinkRule) -> &'static str {
    match rule {
        LinkRule::Neighbors => "NEAR",
        LinkRule::Lines => "LINES",
        LinkRule::Diagonals => "DIAGONAL",
    }
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
fn button_selected(rect: Rect, label: &str, selected: bool, large_text: bool) {
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
    center_text(label, rect, accessibility::text_size(9., large_text), WHITE);
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
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn portrait_size() -> bool {
    crate::ui::is_portrait()
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
