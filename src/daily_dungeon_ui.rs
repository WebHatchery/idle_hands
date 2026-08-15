//! Responsive touch presentation for Daily Dungeon.

use crate::{
    daily_dungeon::{DailyDungeon, DailyPhase, DailyTile},
    state::{AppState, Direction},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    directions: [Rect; 4],
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(280., 52., 300., 300.),
            directions: [
                Rect::new(18., 88., 62., 38.),
                Rect::new(18., 133., 62., 38.),
                Rect::new(18., 178., 62., 38.),
                Rect::new(18., 223., 62., 38.),
            ],
            undo: Rect::new(610., 125., 110., 40.),
            new_game: Rect::new(610., 180., 145., 40.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(40., 125., 320., 320.),
            directions: [
                Rect::new(40., 465., 68., 40.),
                Rect::new(124., 465., 68., 40.),
                Rect::new(208., 465., 68., 40.),
                Rect::new(292., 465., 68., 40.),
            ],
            undo: Rect::new(40., 525., 145., 42.),
            new_game: Rect::new(195., 525., 165., 42.),
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
            undo: Rect::new(810., 220., 120., 44.),
            new_game: Rect::new(950., 220., 140., 44.),
        }
    }
}

pub fn clicks(_state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if Rect::new(0., 0., 110., 42.).contains(point) {
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
    if l.undo.contains(point) {
        return vec![UiAction::DailyUndo];
    }
    if l.new_game.contains(point) {
        return vec![UiAction::DailyNew];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let dungeon = &state.daily_dungeon;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let title_x = if compact {
        70.
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
    text("‹ CABINET", 8., 30., 13., muted());
    text(
        if compact {
            "DAILY RUN"
        } else {
            "DAILY DUNGEON"
        },
        title_x,
        title_y,
        title_size(),
        accent(),
    );
    text(
        &format!(
            "Day {:04}  •  Hearts {}  •  Runes {} / {}",
            dungeon.challenge,
            dungeon.hearts,
            dungeon.runes_found,
            DailyDungeon::rune_total()
        ),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        body_size(),
        muted(),
    );
    let grid = crate::grid::GridLayout::new(l.board, DailyDungeon::size(), DailyDungeon::size());
    for index in 0..DailyDungeon::size().pow(2) {
        let rect = grid.cell_rect(index).unwrap();
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, cell_fill(index, dungeon));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., line_color());
        if index == dungeon.player {
            center_text("@", rect, cell_size(), WHITE);
        } else if index == dungeon.tiles.len() - 1 {
            center_text("EXIT", rect, small_size(), accent());
        } else if !dungeon.revealed[index] {
            center_text("?", rect, cell_size(), muted());
        } else {
            match dungeon.tiles[index] {
                DailyTile::Rune => {
                    center_text("R", rect, cell_size(), Color::new(0.68, 0.92, 1., 1.))
                }
                DailyTile::Trap => {
                    center_text("!", rect, cell_size(), Color::new(0.95, 0.35, 0.45, 1.))
                }
                DailyTile::Exit => center_text("EXIT", rect, small_size(), accent()),
                DailyTile::Floor => {}
            }
        }
    }
    text(
        &status_text(dungeon.phase, dungeon.moves, dungeon.score),
        if compact { 280. } else { title_x },
        if portrait {
            455.
        } else if compact {
            370.
        } else {
            555.
        },
        body_size(),
        muted(),
    );
    for (rect, label) in l.directions.iter().zip(["UP", "LEFT", "DOWN", "RIGHT"]) {
        button(*rect, label);
    }
    button(l.undo, "UNDO");
    button(l.new_game, "NEW DAY");
}

fn cell_fill(index: usize, dungeon: &DailyDungeon) -> Color {
    if index == dungeon.player {
        Color::new(0.15, 0.25, 0.25, 1.)
    } else if index == dungeon.tiles.len() - 1 {
        Color::new(0.32, 0.23, 0.17, 1.)
    } else if !dungeon.revealed[index] {
        Color::new(0.18, 0.13, 0.27, 1.)
    } else {
        Color::new(0.10, 0.08, 0.17, 1.)
    }
}

fn status_text(phase: DailyPhase, moves: u16, score: u32) -> String {
    match phase {
        DailyPhase::Exploring => {
            format!("Reveal the route  •  {} moves  •  {} score", moves, score)
        }
        DailyPhase::Won => format!("The daily route is clear  •  {} score", score),
        DailyPhase::Lost => format!("The traps closed in  •  {} score", score),
    }
}

fn button(rect: Rect, label: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(label, rect, 11., WHITE);
}

fn center_text(label: &str, rect: Rect, size: f32, color: Color) {
    let measured = measure_text(label, None, size as u16, 1.);
    draw_text(
        label,
        rect.x + (rect.w - measured.width) * 0.5,
        rect.y + rect.h * 0.63,
        size,
        color,
    );
}

fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
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

fn cell_size() -> f32 {
    if crate::ui::is_portrait() {
        18.
    } else {
        25.
    }
}

fn small_size() -> f32 {
    if crate::ui::is_portrait() {
        7.
    } else {
        9.
    }
}

fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}

fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}

fn line_color() -> Color {
    Color::new(0.45, 0.38, 0.65, 0.8)
}
