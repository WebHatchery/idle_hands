//! Touch-first presentation for the five Misc cabinet games.

use crate::{
    accessibility,
    misc_games::{MiscGame, MiscKind},
    state::{AppState, GameId},
    ui::UiAction,
};
use macroquad::prelude::*;

const COMPACT_METRICS_X: f32 = 310.;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
struct Layout {
    panel: Rect,
    hint: Rect,
    undo: Rect,
    clear: Rect,
    submit: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            panel: Rect::new(270., 42., 300., 300.),
            hint: Rect::new(620., 105., 105., 44.),
            undo: Rect::new(620., 160., 105., 44.),
            clear: Rect::new(620., 215., 105., 44.),
            submit: Rect::new(735., 215., 105., 44.),
            new_game: Rect::new(620., 270., 140., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            panel: Rect::new(15., 100., 300., 310.),
            hint: Rect::new(15., 470., 90., 44.),
            undo: Rect::new(112., 470., 90., 44.),
            clear: Rect::new(209., 470., 90., 44.),
            submit: Rect::new(15., 525., 137., 44.),
            new_game: Rect::new(162., 525., 137., 44.),
        }
    } else {
        Layout {
            panel: Rect::new(320., 105., 520., 310.),
            hint: Rect::new(875., 125., 120., 44.),
            undo: Rect::new(875., 180., 120., 44.),
            clear: Rect::new(875., 235., 120., 44.),
            submit: Rect::new(1005., 235., 120., 44.),
            new_game: Rect::new(875., 290., 160., 44.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    let game = game(state);
    if crate::ui::hit(Rect::new(0., 0., 120., 44.), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::MiscHint];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::MiscUndo];
    }
    if crate::ui::hit(l.clear, point) {
        return vec![UiAction::MiscClear];
    }
    if crate::ui::hit(l.submit, point) {
        return vec![UiAction::MiscSubmit];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::MiscNew];
    }
    if !l.panel.contains(point) {
        return Vec::new();
    }
    match game.kind {
        MiscKind::RiddleRoom | MiscKind::PatternVault => choice_rects(l.panel)
            .iter()
            .enumerate()
            .find_map(|(index, rect)| {
                crate::ui::hit(*rect, point).then_some(vec![UiAction::MiscTap(index)])
            })
            .unwrap_or_default(),
        MiscKind::SumCircuit => {
            let tile = Rect::new(l.panel.x + 30., l.panel.y + 120., l.panel.w - 60., 132.);
            let cell_w = tile.w / 4.;
            let cell_h = tile.h / 3.;
            let col = ((point.x - tile.x) / cell_w) as usize;
            let row = ((point.y - tile.y) / cell_h) as usize;
            if row < 3 && col < 4 {
                vec![UiAction::MiscTap(row * 4 + col)]
            } else {
                Vec::new()
            }
        }
        MiscKind::OrbitOrder => {
            let rects = orbit_rects(l.panel);
            rects
                .iter()
                .enumerate()
                .find_map(|(index, rect)| {
                    crate::ui::hit(*rect, point).then_some(vec![UiAction::MiscTap(index)])
                })
                .unwrap_or_default()
        }
        MiscKind::WordForge => letter_rects(l.panel, game.board.len())
            .iter()
            .enumerate()
            .find_map(|(index, rect)| {
                crate::ui::hit(*rect, point).then_some(vec![UiAction::MiscTap(index)])
            })
            .unwrap_or_default(),
    }
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = game(state);
    let title_x = if crate::ui::is_compact_landscape() {
        crate::ui::COMPACT_HEADER_TITLE_X
    } else if crate::ui::is_portrait() {
        25.
    } else {
        370.
    };
    let title_y = if crate::ui::is_compact_landscape() {
        28.
    } else if crate::ui::is_portrait() {
        62.
    } else {
        58.
    };
    text(
        "‹ CABINET",
        8.,
        30.,
        13.,
        crate::theme::SECONDARY,
        state.large_text,
    );
    text(
        game.title(),
        title_x,
        title_y,
        title_size(),
        crate::theme::BRASS,
        state.large_text,
    );
    text(
        game.subtitle(),
        title_x,
        title_y + 24.,
        12.,
        crate::theme::SECONDARY,
        state.large_text,
    );
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let metrics = metrics_text(game, compact || portrait);
    text(
        &metrics,
        if compact { COMPACT_METRICS_X } else { title_x },
        if compact {
            28.
        } else if portrait {
            99.
        } else {
            title_y + 45.
        },
        11.,
        crate::theme::CREAM,
        state.large_text,
    );
    draw_panel(l.panel, game, state);
    let status = state
        .card_hint
        .clone()
        .unwrap_or_else(|| game.status_text());
    text(
        &status,
        if crate::ui::is_portrait() {
            20.
        } else if crate::ui::is_compact_landscape() {
            270.
        } else {
            320.
        },
        if crate::ui::is_portrait() {
            435.
        } else if crate::ui::is_compact_landscape() {
            365.
        } else {
            455.
        },
        12.,
        crate::theme::SECONDARY,
        state.large_text,
    );
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.clear, "CLEAR", state.large_text);
    button(l.submit, "SUBMIT", state.large_text);
    button(l.new_game, "NEW ROUND", state.large_text);
}

fn metrics_text(game: &MiscGame, compact: bool) -> String {
    if compact {
        format!("R{} • S{} • M{}", game.round.min(5), game.score, game.moves)
    } else {
        format!(
            "ROUND {}/5  •  SCORE {}  •  MOVES {}",
            game.round.min(5),
            game.score,
            game.moves
        )
    }
}

fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        21.
    } else {
        27.
    }
}

fn draw_panel(rect: Rect, game: &MiscGame, state: &AppState) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.12, 0.08, 0.19, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BRASS);
    center(
        &game.prompt,
        Rect::new(rect.x + 12., rect.y + 15., rect.w - 24., 40.),
        18.,
        state.large_text,
    );
    center(
        &game.detail,
        Rect::new(rect.x + 12., rect.y + 55., rect.w - 24., 30.),
        11.,
        state.large_text,
    );
    match game.kind {
        MiscKind::RiddleRoom | MiscKind::PatternVault => draw_choices(rect, game, state),
        MiscKind::SumCircuit => draw_sum(rect, game, state),
        MiscKind::OrbitOrder => draw_orbit(rect, game, state),
        MiscKind::WordForge => draw_word(rect, game, state),
    }
}

fn draw_choices(rect: Rect, game: &MiscGame, state: &AppState) {
    for (index, option) in game.options.iter().enumerate() {
        let choice = choice_rects(rect)[index];
        let selected = game.selected.contains(&index);
        draw_rectangle(
            choice.x,
            choice.y,
            choice.w,
            choice.h,
            if selected {
                Color::new(0.38, 0.25, 0.46, 1.)
            } else {
                crate::theme::SURFACE
            },
        );
        draw_rectangle_lines(
            choice.x,
            choice.y,
            choice.w,
            choice.h,
            1.,
            if index == game.answer && game.hint_used {
                crate::theme::MOSS
            } else {
                crate::theme::BRASS
            },
        );
        center(
            &format!("{}  {}", (b'A' + index as u8) as char, option),
            choice,
            12.,
            state.large_text,
        );
    }
}

fn draw_sum(rect: Rect, game: &MiscGame, state: &AppState) {
    let tile = Rect::new(rect.x + 30., rect.y + 120., rect.w - 60., 132.);
    for index in 0..12 {
        let col = index % 4;
        let row = index / 4;
        let cell = Rect::new(
            tile.x + col as f32 * tile.w / 4.,
            tile.y + row as f32 * tile.h / 3.,
            tile.w / 4.,
            tile.h / 3.,
        );
        let selected = game.selected.contains(&index);
        draw_rectangle(
            cell.x + 3.,
            cell.y + 3.,
            cell.w - 6.,
            cell.h - 6.,
            if game.board[index] == 0 {
                crate::theme::BACKGROUND_DEEP
            } else if selected {
                Color::new(0.35, 0.32, 0.18, 1.)
            } else {
                crate::theme::SURFACE
            },
        );
        draw_rectangle_lines(
            cell.x + 3.,
            cell.y + 3.,
            cell.w - 6.,
            cell.h - 6.,
            1.,
            if selected {
                crate::theme::MOSS
            } else {
                crate::theme::BRASS
            },
        );
        if game.board[index] != 0 {
            center(&game.board[index].to_string(), cell, 20., state.large_text);
        }
    }
}

fn draw_orbit(rect: Rect, game: &MiscGame, state: &AppState) {
    for (index, planet) in game.board.iter().enumerate() {
        let card = orbit_rects(rect)[index];
        draw_rectangle(
            card.x,
            card.y,
            card.w,
            card.h,
            if game.selected.contains(&index) {
                Color::new(0.38, 0.25, 0.46, 1.)
            } else {
                crate::theme::SURFACE
            },
        );
        draw_rectangle_lines(card.x, card.y, card.w, card.h, 2., crate::theme::BRASS);
        center(
            &planet.to_string(),
            Rect::new(card.x, card.y + 4., card.w, 36.),
            24.,
            state.large_text,
        );
        center(
            &format!("POS {}", index + 1),
            Rect::new(card.x, card.y + 47., card.w, 18.),
            9.,
            state.large_text,
        );
    }
}

fn draw_word(rect: Rect, game: &MiscGame, state: &AppState) {
    let chosen: String = game
        .selected
        .iter()
        .map(|index| game.board[*index] as char)
        .collect();
    center(
        &format!("_ {}", chosen),
        Rect::new(rect.x + 20., rect.y + 92., rect.w - 40., 28.),
        17.,
        state.large_text,
    );
    for (index, letter) in game.board.iter().enumerate() {
        let cell = letter_rects(rect, game.board.len())[index];
        let selected = game.selected.contains(&index);
        draw_rectangle(
            cell.x,
            cell.y,
            cell.w,
            cell.h,
            if selected {
                Color::new(0.35, 0.32, 0.18, 1.)
            } else {
                crate::theme::SURFACE
            },
        );
        draw_rectangle_lines(
            cell.x,
            cell.y,
            cell.w,
            cell.h,
            1.,
            if selected {
                crate::theme::MOSS
            } else {
                crate::theme::BRASS
            },
        );
        center(&(*letter as char).to_string(), cell, 19., state.large_text);
    }
}

fn choice_rects(panel: Rect) -> [Rect; 4] {
    [
        Rect::new(panel.x + 24., panel.y + 105., panel.w - 48., 42.),
        Rect::new(panel.x + 24., panel.y + 153., panel.w - 48., 42.),
        Rect::new(panel.x + 24., panel.y + 201., panel.w - 48., 42.),
        Rect::new(panel.x + 24., panel.y + 249., panel.w - 48., 42.),
    ]
}

fn orbit_rects(panel: Rect) -> [Rect; 5] {
    let width = (panel.w - 48.) / 5.;
    std::array::from_fn(|index| {
        Rect::new(
            panel.x + 16. + index as f32 * width,
            panel.y + 135.,
            width - 5.,
            78.,
        )
    })
}

fn letter_rects(panel: Rect, count: usize) -> Vec<Rect> {
    let width = (panel.w - 48.) / count.max(1) as f32;
    (0..count)
        .map(|index| {
            Rect::new(
                panel.x + 24. + index as f32 * width,
                panel.y + 180.,
                width - 5.,
                54.,
            )
        })
        .collect()
}

fn game(state: &AppState) -> &MiscGame {
    match state.screen {
        crate::state::Screen::Game(GameId::RiddleRoom) => &state.games.riddle_room,
        crate::state::Screen::Game(GameId::PatternVault) => &state.games.pattern_vault,
        crate::state::Screen::Game(GameId::SumCircuit) => &state.games.sum_circuit,
        crate::state::Screen::Game(GameId::OrbitOrder) => &state.games.orbit_order,
        crate::state::Screen::Game(GameId::WordForge) => &state.games.word_forge,
        _ => &state.games.riddle_room,
    }
}

fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., crate::theme::BRASS);
    center(label, rect, 11., large_text);
}

fn center(label: &str, rect: Rect, size: f32, large_text: bool) {
    let size = accessibility::text_size(size, large_text);
    let measured = crate::ui::measure_text(label, None, size as u16, 1.);
    crate::ui::draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.65,
        size,
        crate::theme::CREAM,
    );
}

fn text(label: &str, x: f32, y: f32, size: f32, color: Color, large_text: bool) {
    crate::ui::draw_text(
        label,
        x,
        y,
        accessibility::text_size(size, large_text),
        color,
    );
}
