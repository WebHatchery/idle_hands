//! Responsive touch presentation for Dungeon Sweeper.

use crate::{
    accessibility,
    dungeon_sweeper::{DungeonCell, DungeonDifficulty, DungeonStatus},
    state::AppState,
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    flag: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
    difficulties: [Rect; 3],
}
fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(250., 66., 560., 280.),
            flag: Rect::new(18., 170., 100., 44.),
            hint: Rect::new(18., 326., 100., 44.),
            undo: Rect::new(18., 222., 100., 44.),
            new_game: Rect::new(18., 274., 120., 44.),
            difficulties: [
                Rect::new(18., 66., 68., 44.),
                Rect::new(92., 66., 68., 44.),
                Rect::new(166., 66., 68., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(20., 155., 320., 320.),
            flag: Rect::new(20., 500., 145., 44.),
            hint: Rect::new(20., 620., 145., 44.),
            undo: Rect::new(20., 560., 145., 44.),
            new_game: Rect::new(185., 560., 165., 44.),
            difficulties: [
                Rect::new(20., 105., 100., 44.),
                Rect::new(130., 105., 100., 44.),
                Rect::new(240., 105., 100., 44.),
            ],
        }
    } else {
        Layout {
            board: Rect::new(360., 120., 420., 420.),
            flag: Rect::new(820., 390., 150., 46.),
            hint: Rect::new(820., 510., 120., 44.),
            undo: Rect::new(820., 450., 120., 44.),
            new_game: Rect::new(950., 450., 150., 44.),
            difficulties: [
                Rect::new(820., 120., 100., 44.),
                Rect::new(930., 120., 100., 44.),
                Rect::new(1040., 120., 100., 44.),
            ],
        }
    }
}
pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(Rect::new(0., 0., 110., 42.), point) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(l.flag, point) {
        return vec![UiAction::DungeonToggleFlag];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::DungeonUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::DungeonHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::DungeonNew];
    }
    for (rect, difficulty) in l.difficulties.iter().zip(DungeonDifficulty::ALL) {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::DungeonDifficulty(difficulty)];
        }
    }
    if let Some(index) = crate::grid::GridLayout::new(l.board, 8, 8).index_at(point) {
        return vec![UiAction::DungeonCell(index)];
    }
    let _ = state;
    vec![]
}
pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.dungeon_sweeper;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let x = if compact {
        430.
    } else if portrait {
        16.
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
    text("‹ CABINET", 8., 30., 13., muted());
    text(
        "DUNGEON SWEEPER",
        x,
        y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let run_status = if compact {
        format!(
            "{}  •  H{}  •  K{}/{}  •  {} moves",
            game.difficulty.label(),
            game.hearts,
            game.relics_found(),
            game.relic_total(),
            game.moves
        )
    } else {
        format!(
            "{}  •  H{}  •  Relics {}/{}  •  {}",
            game.difficulty.label(),
            game.hearts,
            game.relics_found(),
            game.relic_total(),
            status_text(game.status, game.moves)
        )
    };
    text(
        &run_status,
        if compact { 430. } else { x },
        if compact { 52. } else { y + 25. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    let grid = GridLayout::new(l.board, 8, 8);
    for index in 0..64 {
        let rect = grid.cell_rect(index).unwrap();
        let cell = game.cells[index];
        let revealed = matches!(cell, DungeonCell::Revealed(_));
        let fill = if revealed {
            if state.high_contrast {
                accessibility::mine_cell(true, true)
            } else {
                Color::new(0.20, 0.16, 0.29, 1.)
            }
        } else {
            if state.high_contrast {
                accessibility::mine_cell(false, true)
            } else {
                Color::new(0.11, 0.08, 0.18, 1.)
            }
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            accessibility::grid_line(state.high_contrast),
        );
        let show_trap = matches!(game.status, DungeonStatus::Lost)
            && matches!(cell, DungeonCell::Trap | DungeonCell::FlaggedTrap);
        if show_trap || matches!(cell, DungeonCell::Revealed(9)) {
            text(
                "×",
                rect.x + rect.w * 0.34,
                rect.y + rect.h * 0.67,
                cell_size(state.large_text),
                Color::new(0.95, 0.35, 0.45, 1.),
            );
        } else if index == game.exit && revealed {
            text(
                if game.relics_found() >= game.relic_total() {
                    "E"
                } else {
                    "LOCK"
                },
                rect.x
                    + if game.relics_found() >= game.relic_total() {
                        rect.w * 0.34
                    } else {
                        rect.w * 0.12
                    },
                rect.y + rect.h * 0.67,
                if game.relics_found() >= game.relic_total() {
                    cell_size(state.large_text)
                } else {
                    accessibility::text_size(8., state.large_text)
                },
                accent(),
            );
        } else if revealed && game.relics.contains(&index) {
            text(
                "K",
                rect.x + rect.w * 0.34,
                rect.y + rect.h * 0.67,
                cell_size(state.large_text),
                Color::new(0.30, 0.92, 0.88, 1.),
            );
            if let DungeonCell::Revealed(number) = cell {
                text(
                    &number.to_string(),
                    rect.right() - rect.w * 0.25,
                    rect.y + rect.h * 0.26,
                    accessibility::text_size(8., state.large_text),
                    WHITE,
                );
            }
        } else if let DungeonCell::Revealed(number) = cell {
            text(
                &number.to_string(),
                rect.x + rect.w * 0.40,
                rect.y + rect.h * 0.66,
                cell_size(state.large_text),
                WHITE,
            );
        } else if matches!(cell, DungeonCell::Flagged | DungeonCell::FlaggedTrap) {
            let pole_x = rect.x + rect.w * 0.40;
            let top = rect.y + rect.h * 0.25;
            let bottom = rect.y + rect.h * 0.74;
            draw_line(pole_x, top, pole_x, bottom, 2., accent());
            draw_triangle(
                vec2(pole_x, top),
                vec2(rect.x + rect.w * 0.72, rect.y + rect.h * 0.38),
                vec2(pole_x, rect.y + rect.h * 0.50),
                accent(),
            );
            draw_line(
                rect.x + rect.w * 0.28,
                bottom,
                rect.x + rect.w * 0.58,
                bottom,
                2.,
                accent(),
            );
        }
    }
    text("EXIT", l.board.right() - 38., l.board.y - 8., 10., accent());
    text(
        &format!(
            "Traps flagged {} / {}  •  {}",
            game.flagged_count(),
            game.traps,
            state
                .card_hint
                .as_deref()
                .unwrap_or("Find relics, then EXIT")
        ),
        if compact { 18. } else { x },
        if portrait {
            485.
        } else if compact {
            155.
        } else {
            565.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    button(
        l.flag,
        if state.mine_flag_mode {
            "FLAG MODE ON"
        } else {
            "FLAG MODE"
        },
        state.large_text,
    );
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW DUNGEON", state.large_text);
    for (rect, difficulty) in l.difficulties.iter().zip(DungeonDifficulty::ALL) {
        difficulty_button(
            *rect,
            difficulty,
            difficulty == game.difficulty,
            state.large_text,
        );
    }
}
use crate::grid::GridLayout;
fn status_text(status: DungeonStatus, moves: u16) -> String {
    match status {
        DungeonStatus::Ready => "Tap a room to enter".into(),
        DungeonStatus::Playing => format!("Find EXIT • tap clues to chord • {} moves", moves),
        DungeonStatus::Won => "The exit is found".into(),
        DungeonStatus::Lost => "A trap closed the path".into(),
    }
}
fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    text(
        label,
        rect.x + 12.,
        rect.y + 29.,
        accessibility::text_size(11., large_text),
        WHITE,
    );
}
fn difficulty_button(rect: Rect, difficulty: DungeonDifficulty, selected: bool, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::new(0.30, 0.23, 0.12, 1.)
        } else {
            crate::theme::SURFACE
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected { 3. } else { 1. },
        accent(),
    );
    text(
        difficulty.label(),
        rect.x + 6.,
        rect.y + 28.,
        accessibility::text_size(
            if crate::ui::is_compact_landscape() {
                7.
            } else {
                8.
            },
            large_text,
        ),
        if selected { accent() } else { WHITE },
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        22.
    } else {
        28.
    }
}
fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        11.
    } else {
        13.
    }
}
fn cell_size(large_text: bool) -> f32 {
    if crate::ui::is_portrait() {
        accessibility::text_size(20., large_text).min(24.)
    } else {
        accessibility::text_size(28., large_text).min(32.)
    }
}
fn accent() -> Color {
    crate::theme::BRASS
}
fn muted() -> Color {
    crate::theme::SECONDARY
}
