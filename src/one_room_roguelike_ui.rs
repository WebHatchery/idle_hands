//! Responsive touch presentation for One Room Roguelike.

use crate::domain::Direction;
use crate::{
    accessibility,
    one_room_roguelike::{EnemyKind, HeroClass, OneRoomRoguelike, RoomPhase},
    state::AppState,
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
    classes: [Rect; 3],
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
            classes: [
                Rect::new(90., 55., 62., 44.),
                Rect::new(157., 55., 62., 44.),
                Rect::new(224., 55., 62., 44.),
            ],
        }
    } else if crate::ui::is_portrait() {
        Layout {
            board: Rect::new(53., 145., 294., 294.),
            directions: [
                Rect::new(40., 465., 68., 44.),
                Rect::new(124., 465., 68., 44.),
                Rect::new(208., 465., 68., 44.),
                Rect::new(292., 465., 68., 44.),
            ],
            strike: Rect::new(40., 520., 145., 44.),
            potion: Rect::new(195., 520., 165., 44.),
            hint: Rect::new(40., 630., 145., 44.),
            undo: Rect::new(40., 575., 145., 44.),
            new_game: Rect::new(195., 575., 165., 44.),
            classes: [
                Rect::new(20., 96., 110., 44.),
                Rect::new(135., 96., 110., 44.),
                Rect::new(250., 96., 110., 44.),
            ],
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
            classes: [
                Rect::new(810., 82., 90., 44.),
                Rect::new(910., 82., 90., 44.),
                Rect::new(1010., 82., 100., 44.),
            ],
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
    for (rect, hero_class) in l.classes.iter().zip(HeroClass::ALL) {
        if crate::ui::hit(*rect, point) {
            return vec![UiAction::RogueClass(hero_class)];
        }
    }
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.one_room_roguelike;
    let compact = crate::ui::is_compact_landscape();
    let portrait = crate::ui::is_portrait();
    let (compact_title_x, compact_status_x) = compact_header_positions();
    let title_x = if compact {
        compact_title_x
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
        if compact || portrait {
            "ROOM ROGUE"
        } else {
            "ONE ROOM ROGUELIKE"
        },
        title_x,
        title_y,
        accessibility::text_size(title_size(), state.large_text),
        accent(),
    );
    let run_status = if compact {
        format!(
            "R {}/{}  HP {}/{}  P {}  S {}",
            game.room,
            OneRoomRoguelike::target_room(),
            game.health,
            game.max_health(),
            game.potions,
            game.score
        )
    } else if portrait {
        format!(
            "{}  •  HP {}/{}  •  P {}",
            game.hero_class.label(),
            game.health,
            game.max_health(),
            game.potions
        )
    } else {
        format!(
            "{}  •  Room {} / {}  •  Health {} / {}  •  Potions {}  •  Score {}",
            game.hero_class.label(),
            game.room,
            OneRoomRoguelike::target_room(),
            game.health,
            game.max_health(),
            game.potions,
            game.score
        )
    };
    text(
        &run_status,
        if compact { compact_status_x } else { title_x },
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
            center_text(
                if game.phase == RoomPhase::Stairs {
                    "STAIRS"
                } else {
                    "EXIT"
                },
                rect,
                small_size(state.large_text),
                accent(),
            );
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
            let label = format!("{}{}", enemy_label(enemy.kind), enemy.health);
            let enemy_color = enemy_color(enemy.kind, state.high_contrast);
            if enemy.kind == EnemyKind::Brute {
                draw_circle_lines(
                    rect.x + rect.w * 0.5,
                    rect.y + rect.h * 0.5,
                    rect.w.min(rect.h) * 0.34,
                    3.,
                    Color::new(1., 0.65, 0.18, 1.),
                );
            }
            draw_circle(
                rect.x + rect.w * 0.5,
                rect.y + rect.h * 0.5,
                rect.w.min(rect.h) * 0.27,
                enemy_color,
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
        center_text(
            hero_label(game.hero_class),
            rect,
            cell_size(state.large_text),
            WHITE,
        );
    }
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or(&status_text(game.phase, game.turns)),
        if compact { 300. } else { title_x },
        if portrait {
            448.
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
    button(l.new_game, "NEW RUN", state.large_text);
    for (rect, hero_class) in l.classes.iter().zip(HeroClass::ALL) {
        class_button(
            *rect,
            hero_class,
            hero_class == game.hero_class,
            state.large_text,
        );
    }
}

fn hero_label(hero_class: HeroClass) -> &'static str {
    match hero_class {
        HeroClass::Blade => "B",
        HeroClass::Warden => "W",
        HeroClass::Alchemist => "A",
    }
}

fn enemy_label(kind: EnemyKind) -> &'static str {
    match kind {
        EnemyKind::Guard => "G",
        EnemyKind::Stalker => "S",
        EnemyKind::Brute => "B",
    }
}

fn enemy_color(kind: EnemyKind, high_contrast: bool) -> Color {
    match (kind, high_contrast) {
        (EnemyKind::Guard, false) => Color::new(0.62, 0.22, 0.35, 1.),
        (EnemyKind::Stalker, false) => Color::new(0.28, 0.45, 0.72, 1.),
        (EnemyKind::Brute, false) => Color::new(0.72, 0.31, 0.16, 1.),
        (EnemyKind::Guard, true) => Color::new(1., 0.12, 0.20, 1.),
        (EnemyKind::Stalker, true) => Color::new(0.10, 0.65, 1., 1.),
        (EnemyKind::Brute, true) => Color::new(1., 0.42, 0.05, 1.),
    }
}

fn cell_fill(index: usize, game: &OneRoomRoguelike, high_contrast: bool) -> Color {
    if index == game.exit {
        if game.phase == RoomPhase::Stairs && high_contrast {
            Color::new(0.80, 0.52, 0.05, 1.)
        } else if game.phase == RoomPhase::Stairs {
            Color::new(0.48, 0.29, 0.16, 1.)
        } else if high_contrast {
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
        RoomPhase::Exploring => format!("G guard  •  S stalker  •  B brute  •  {} turns", turns),
        RoomPhase::Stairs => format!("The room is clear  •  Move to STAIRS  •  {} turns", turns),
        RoomPhase::Won => format!("The run is complete  •  {} turns", turns),
        RoomPhase::Lost => format!("The run claims you  •  {} turns", turns),
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

fn class_button(rect: Rect, hero_class: HeroClass, selected: bool, large_text: bool) {
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
    center_text(
        hero_class.label(),
        rect,
        accessibility::text_size(if crate::ui::is_portrait() { 8. } else { 9. }, large_text),
        if selected { accent() } else { WHITE },
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
    if crate::ui::is_compact_landscape() || crate::ui::is_portrait() {
        20.
    } else {
        28.
    }
}

fn compact_header_positions() -> (f32, f32) {
    (82., 280.)
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
    crate::theme::BRASS
}

fn muted() -> Color {
    crate::theme::SECONDARY
}

fn line_color(high_contrast: bool) -> Color {
    accessibility::grid_line(high_contrast)
}

#[cfg(test)]
mod tests;
