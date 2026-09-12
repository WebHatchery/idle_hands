//! Responsive touch presentation for Daily Dungeon.

use crate::domain::Direction;
use crate::{
    accessibility,
    daily_dungeon::{DailyDungeon, DailyPhase, DailyTile},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    directions: [Rect; 4],
    hint: Rect,
    scout: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(280., 52., 300., 300.),
            directions: [
                Rect::new(18., 88., 62., 44.),
                Rect::new(18., 133., 62., 44.),
                Rect::new(18., 178., 62., 44.),
                Rect::new(18., 223., 62., 44.),
            ],
            hint: Rect::new(610., 235., 110., 44.),
            scout: Rect::new(105., 145., 145., 44.),
            undo: Rect::new(610., 125., 110., 44.),
            new_game: Rect::new(610., 180., 145., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(20., 125., 320., 320.),
            directions: [
                Rect::new(20., 465., 68., 44.),
                Rect::new(104., 465., 68., 44.),
                Rect::new(188., 465., 68., 44.),
                Rect::new(272., 465., 68., 44.),
            ],
            hint: Rect::new(20., 580., 145., 44.),
            scout: Rect::new(185., 580., 155., 44.),
            undo: Rect::new(20., 525., 145., 44.),
            new_game: Rect::new(185., 525., 155., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 105., 420., 420.),
            directions: [
                Rect::new(810., 145., 62., 44.),
                Rect::new(882., 145., 62., 44.),
                Rect::new(954., 145., 62., 44.),
                Rect::new(1026., 145., 62., 44.),
            ],
            hint: Rect::new(810., 285., 120., 44.),
            scout: Rect::new(950., 285., 140., 44.),
            undo: Rect::new(810., 220., 120., 44.),
            new_game: Rect::new(950., 220., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    for (index, rect) in l.directions.iter().enumerate() {
        if rect.contains(point) {
            return vec![UiAction::DailyMove(
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
        return vec![UiAction::DailyUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::DailyHint];
    }
    if crate::ui::hit(l.scout, point) {
        return vec![UiAction::DailyScout];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::DailyNew];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let dungeon = &state.games.daily_dungeon;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        crate::ui::COMPACT_HEADER_TITLE_X
    } else if portrait {
        10.
    } else {
        400.
    };
    let title_y = if compact {
        28.
    } else if portrait {
        64.
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
        if compact {
            "DAILY RUN"
        } else {
            "DAILY DUNGEON"
        },
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let run_status = if compact || portrait {
        format!(
            "{}  •  H{}  •  R{}/{}",
            crate::daily_challenge::label(dungeon.day_key, dungeon.challenge),
            dungeon.hearts,
            dungeon.runes_found,
            DailyDungeon::rune_total()
        )
    } else {
        format!(
            "{}  •  {}  •  Hearts {}  •  Runes {} / {}  •  Scouts {}  •  Best {}",
            dungeon.rule.label(),
            crate::daily_challenge::label(dungeon.day_key, dungeon.challenge),
            dungeon.hearts,
            dungeon.runes_found,
            DailyDungeon::rune_total(),
            dungeon.scouts,
            state
                .records
                .daily_score(dungeon.day_key)
                .map_or_else(|| "—".to_owned(), |score| score.to_string())
        )
    };
    text(
        &run_status,
        if compact {
            crate::ui::COMPACT_HEADER_STATUS_X
        } else {
            title_x
        },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    let grid = crate::grid::GridLayout::new(l.board, DailyDungeon::size(), DailyDungeon::size());
    for index in 0..DailyDungeon::size().pow(2) {
        let Some(rect) = grid.cell_rect(index) else {
            continue;
        };
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            cell_fill(index, dungeon, state.high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            line_color(state.high_contrast),
        );
        if dungeon.scouts > 0
            && !dungeon.revealed[index]
            && grid_distance(index, dungeon.player) == 1
        {
            draw_rectangle_lines(
                rect.x + 2.,
                rect.y + 2.,
                rect.w - 4.,
                rect.h - 4.,
                2.,
                accent(),
            );
        }
        if index == dungeon.player {
            center_text("@", rect, cell_size(state.large_text), WHITE);
        } else if index == dungeon.tiles.len() - 1 {
            center_text("EXIT", rect, small_size(state.large_text), accent());
        } else if !dungeon.revealed[index] {
            center_text("?", rect, cell_size(state.large_text), muted());
        } else {
            match dungeon.tiles[index] {
                DailyTile::Rune => center_text(
                    "R",
                    rect,
                    cell_size(state.large_text),
                    Color::new(0.35, 1., 1., 1.),
                ),
                DailyTile::Trap => center_text(
                    "!",
                    rect,
                    cell_size(state.large_text),
                    Color::new(1., 0.12, 0.20, 1.),
                ),
                DailyTile::Exit => {
                    center_text("EXIT", rect, small_size(state.large_text), accent())
                }
                DailyTile::Spring => center_text(
                    "+",
                    rect,
                    cell_size(state.large_text),
                    Color::new(0.30, 0.90, 0.55, 1.),
                ),
                DailyTile::Floor => {}
            }
        }
    }
    text(
        state.card_hint.as_deref().unwrap_or(&status_text(
            dungeon.day_key,
            dungeon.challenge,
            dungeon.phase,
            dungeon.moves,
            dungeon.score,
            state.records.daily_score(dungeon.day_key),
        )),
        if compact { 280. } else { title_x },
        if portrait {
            455.
        } else if compact {
            370.
        } else {
            555.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for (rect, label) in l.directions.iter().zip(["UP", "LEFT", "DOWN", "RIGHT"]) {
        button(*rect, label, state.large_text);
    }
    button(l.hint, "HINT", state.large_text);
    button(
        l.scout,
        &format!("SCOUT {}", dungeon.scouts),
        state.large_text,
    );
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "REPLAY DAY", state.large_text);
}

fn cell_fill(index: usize, dungeon: &DailyDungeon, high_contrast: bool) -> Color {
    if index == dungeon.player {
        if high_contrast {
            Color::new(0.05, 0.30, 0.30, 1.)
        } else {
            Color::new(0.15, 0.25, 0.25, 1.)
        }
    } else if index == dungeon.tiles.len() - 1 {
        if high_contrast {
            Color::new(0.50, 0.38, 0.05, 1.)
        } else {
            Color::new(0.32, 0.23, 0.17, 1.)
        }
    } else if !dungeon.revealed[index] {
        if high_contrast {
            Color::new(0.18, 0.14, 0.26, 1.)
        } else {
            Color::new(0.18, 0.13, 0.27, 1.)
        }
    } else {
        accessibility::board_fill(high_contrast)
    }
}

fn grid_distance(first: usize, second: usize) -> usize {
    let size = DailyDungeon::size();
    (first / size).abs_diff(second / size) + (first % size).abs_diff(second % size)
}

fn status_text(
    day: u64,
    challenge: u32,
    phase: DailyPhase,
    moves: u16,
    score: u32,
    best_score: Option<u32>,
) -> String {
    let identity = crate::daily_challenge::status_label(day, challenge, phase);
    let best = best_score.map_or_else(|| "—".to_owned(), |value| value.to_string());
    match phase {
        DailyPhase::Exploring => {
            format!(
                "{}  •  scout or risk hidden rooms  •  {} actions  •  {} score  •  best {}",
                identity, moves, score, best
            )
        }
        DailyPhase::Won => format!(
            "{}  •  the route is clear  •  {} score  •  best {}",
            identity, score, best
        ),
        DailyPhase::Lost => format!(
            "{}  •  the traps closed in  •  {} score  •  best {}",
            identity, score, best
        ),
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

fn title_size() -> f32 {
    if crate::ui::is_compact_landscape() {
        20.
    } else if crate::ui::is_portrait() {
        23.
    } else {
        29.
    }
}

fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        10.
    } else {
        12.
    }
}

fn cell_size(large_text: bool) -> f32 {
    if crate::ui::is_portrait() {
        accessibility::text_size(18., large_text).min(22.)
    } else {
        accessibility::text_size(25., large_text).min(29.)
    }
}

fn small_size(large_text: bool) -> f32 {
    if crate::ui::is_portrait() {
        accessibility::text_size(7., large_text).min(9.)
    } else {
        accessibility::text_size(9., large_text).min(11.)
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
