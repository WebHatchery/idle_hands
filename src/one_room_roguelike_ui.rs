//! Responsive touch presentation for One Room Roguelike.

use crate::{
    accessibility,
    one_room_roguelike::{OneRoomRoguelike, RoomPhase},
    state::{AppState, Direction},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Layout {
    board: Rect,
    directions: [Rect; 4],
    strike: Rect,
    potion: Rect,
    hint: Rect,
    undo: Rect,
    new_game: Rect,
}

fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        Layout {
            board: Rect::new(300., 55., 280., 280.),
            directions: [
                Rect::new(18., 88., 62., 44.),
                Rect::new(18., 133., 62., 44.),
                Rect::new(18., 178., 62., 44.),
                Rect::new(18., 223., 62., 44.),
            ],
            strike: Rect::new(105., 125., 145., 44.),
            potion: Rect::new(105., 180., 145., 44.),
            hint: Rect::new(610., 230., 110., 44.),
            undo: Rect::new(610., 120., 110., 44.),
            new_game: Rect::new(610., 175., 145., 44.),
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(40., 130., 320., 320.),
            directions: [
                Rect::new(40., 470., 68., 44.),
                Rect::new(124., 470., 68., 44.),
                Rect::new(208., 470., 68., 44.),
                Rect::new(292., 470., 68., 44.),
            ],
            strike: Rect::new(40., 525., 145., 44.),
            potion: Rect::new(195., 525., 165., 44.),
            hint: Rect::new(40., 635., 145., 42.),
            undo: Rect::new(40., 580., 145., 44.),
            new_game: Rect::new(195., 580., 165., 44.),
        }
    } else {
        Layout {
            board: Rect::new(350., 110., 420., 420.),
            directions: [
                Rect::new(810., 145., 62., 44.),
                Rect::new(882., 145., 62., 44.),
                Rect::new(954., 145., 62., 44.),
                Rect::new(1026., 145., 62., 44.),
            ],
            strike: Rect::new(810., 220., 125., 44.),
            potion: Rect::new(955., 220., 135., 44.),
            hint: Rect::new(810., 350., 120., 44.),
            undo: Rect::new(810., 285., 120., 44.),
            new_game: Rect::new(950., 285., 140., 44.),
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
            return vec![UiAction::RogueMove(
                [
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                ][index],
            )];
        }
    }
    if crate::ui::hit(l.strike, point) {
        return vec![UiAction::RogueStrike];
    }
    if crate::ui::hit(l.potion, point) {
        return vec![UiAction::RoguePotion];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::RogueUndo];
    }
    if crate::ui::hit(l.hint, point) {
        return vec![UiAction::RogueHint];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::RogueNew];
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.one_room_roguelike;
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
        65.
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
            "ROOM ROGUE"
        } else {
            "ONE ROOM ROGUELIKE"
        },
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    text(
        &format!(
            "Health {} / 10  •  Potions {}  •  Score {}",
            game.health, game.potions, game.score
        ),
        if compact { 430. } else { title_x },
        if compact { 28. } else { title_y + 24. },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    let grid =
        crate::grid::GridLayout::new(l.board, OneRoomRoguelike::size(), OneRoomRoguelike::size());
    for index in 0..OneRoomRoguelike::size().pow(2) {
        let rect = grid.cell_rect(index).unwrap();
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            cell_fill(index, game, state.high_contrast),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            1.,
            line_color(state.high_contrast),
        );
        if index == game.exit {
            center_text("EXIT", rect, small_size(state.large_text), accent());
        } else if index == game.treasure {
            center_text(
                "C",
                rect,
                cell_size(state.large_text),
                Color::new(0.35, 1., 1., 1.),
            );
        }
    }
    for enemy in &game.enemies {
        if let Some(rect) = grid.cell_rect(enemy.position) {
            let label = format!("E{}", enemy.health);
            draw_circle(
                rect.x + rect.w * 0.5,
                rect.y + rect.h * 0.5,
                rect.w.min(rect.h) * 0.27,
                if state.high_contrast {
                    Color::new(1., 0.12, 0.20, 1.)
                } else {
                    Color::new(0.62, 0.22, 0.35, 1.)
                },
            );
            center_text(&label, rect, small_size(state.large_text), WHITE);
        }
    }
    if let Some(rect) = grid.cell_rect(game.player) {
        draw_circle(
            rect.x + rect.w * 0.5,
            rect.y + rect.h * 0.5,
            rect.w.min(rect.h) * 0.27,
            if state.high_contrast {
                Color::new(0.05, 0.90, 0.30, 1.)
            } else {
                Color::new(0.26, 0.60, 0.48, 1.)
            },
        );
        center_text("@", rect, cell_size(state.large_text), WHITE);
    }
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or(&status_text(game.phase, game.turns)),
        if compact { 300. } else { title_x },
        if portrait {
            460.
        } else if compact {
            355.
        } else {
            558.
        },
        accessibility::text_size(body_size(), state.large_text),
        muted(),
    );
    for (rect, label) in l.directions.iter().zip(["UP", "LEFT", "DOWN", "RIGHT"]) {
        button(*rect, label, state.large_text);
    }
    button(l.strike, "STRIKE", state.large_text);
    button(l.potion, "POTION", state.large_text);
    button(l.hint, "HINT", state.large_text);
    button(l.undo, "UNDO", state.large_text);
    button(l.new_game, "NEW ROOM", state.large_text);
}

fn cell_fill(index: usize, game: &OneRoomRoguelike, high_contrast: bool) -> Color {
    if index == game.exit {
        if high_contrast {
            Color::new(0.50, 0.38, 0.05, 1.)
        } else {
            Color::new(0.32, 0.23, 0.17, 1.)
        }
    } else if index == game.player {
        if high_contrast {
            Color::new(0.05, 0.30, 0.30, 1.)
        } else {
            Color::new(0.15, 0.25, 0.25, 1.)
        }
    } else {
        accessibility::board_fill(high_contrast)
    }
}

fn status_text(phase: RoomPhase, turns: u16) -> String {
    match phase {
        RoomPhase::Exploring => format!("Clear the room  •  {} turns", turns),
        RoomPhase::Won => format!("The room is quiet  •  {} turns", turns),
        RoomPhase::Lost => format!("The room claims you  •  {} turns", turns),
    }
}

fn button(rect: Rect, label: &str, large_text: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.20, 0.13, 0.30, 1.),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    center_text(
        label,
        rect,
        accessibility::text_size(11., large_text),
        WHITE,
    );
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
        22.
    } else {
        28.
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
        accessibility::text_size(8., large_text).min(10.)
    } else {
        accessibility::text_size(10., large_text).min(12.)
    }
}

fn accent() -> Color {
    Color::new(0.98, 0.83, 0.45, 1.)
}

fn muted() -> Color {
    Color::new(0.70, 0.64, 0.78, 1.)
}

fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}
