//! Responsive presentation and touch routing for Terrain Cannon.

use crate::{
    accessibility,
    state::AppState,
    terrain_cannon::{CannonStatus, TerrainCannon, HEIGHT, WIDTH},
    ui::UiAction,
};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Layout {
    pub board: Rect,
    pub cell: f32,
    pub angle_down: Rect,
    pub angle_up: Rect,
    pub power_down: Rect,
    pub power_up: Rect,
    pub fire: Rect,
    pub pause: Rect,
    pub undo: Rect,
    pub new_game: Rect,
}

pub fn layout() -> Layout {
    if crate::ui::is_compact_landscape() {
        let cell = 14.;
        Layout {
            board: Rect::new(10., 42., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            angle_down: Rect::new(465., 50., 48., 38.),
            angle_up: Rect::new(518., 50., 48., 38.),
            power_down: Rect::new(465., 95., 48., 38.),
            power_up: Rect::new(518., 95., 48., 38.),
            fire: Rect::new(465., 140., 101., 42.),
            pause: Rect::new(465., 190., 101., 36.),
            undo: Rect::new(465., 234., 48., 36.),
            new_game: Rect::new(518., 234., 48., 36.),
        }
    } else if crate::ui::is_portrait() {
        let cell = 9.;
        Layout {
            board: Rect::new(10., 104., f32::from(WIDTH) * cell, f32::from(HEIGHT) * cell),
            cell,
            angle_down: Rect::new(20., 285., 70., 40.),
            angle_up: Rect::new(95., 285., 70., 40.),
            power_down: Rect::new(170., 285., 70., 40.),
            power_up: Rect::new(245., 285., 85., 40.),
            fire: Rect::new(20., 335., 145., 42.),
            pause: Rect::new(185., 335., 145., 42.),
            undo: Rect::new(20., 390., 145., 42.),
            new_game: Rect::new(185., 390., 145., 42.),
        }
    } else {
        let cell = 24.;
        Layout {
            board: Rect::new(
                180.,
                100.,
                f32::from(WIDTH) * cell,
                f32::from(HEIGHT) * cell,
            ),
            cell,
            angle_down: Rect::new(970., 135., 86., 42.),
            angle_up: Rect::new(1062., 135., 86., 42.),
            power_down: Rect::new(970., 185., 86., 42.),
            power_up: Rect::new(1062., 185., 86., 42.),
            fire: Rect::new(970., 235., 178., 44.),
            pause: Rect::new(970., 290., 86., 42.),
            undo: Rect::new(1062., 290., 86., 42.),
            new_game: Rect::new(970., 345., 178., 42.),
        }
    }
}

pub fn clicks(state: &AppState, point: Vec2) -> Vec<UiAction> {
    let l = layout();
    if crate::ui::hit(back_rect(), point) {
        return vec![UiAction::Cabinet];
    }
    for (rect, action) in [
        (l.angle_down, UiAction::CannonAngle(-5)),
        (l.angle_up, UiAction::CannonAngle(5)),
        (l.power_down, UiAction::CannonPower(-5)),
        (l.power_up, UiAction::CannonPower(5)),
    ] {
        if crate::ui::hit(rect, point) {
            return vec![action];
        }
    }
    if crate::ui::hit(l.fire, point) {
        return vec![UiAction::CannonFire];
    }
    if crate::ui::hit(l.pause, point) {
        return vec![UiAction::CannonPause];
    }
    if crate::ui::hit(l.undo, point) {
        return vec![UiAction::CannonUndo];
    }
    if crate::ui::hit(l.new_game, point) {
        return vec![UiAction::CannonNew];
    }
    let _ = state;
    vec![]
}

pub fn draw(state: &AppState) {
    let l = layout();
    let game = &state.games.terrain_cannon;
    let compact = crate::ui::is_compact_landscape();
    let title_x = if compact {
        80.
    } else if crate::ui::is_portrait() {
        10.
    } else {
        180.
    };
    let title_y = if crate::ui::is_compact_landscape() {
        27.
    } else if crate::ui::is_portrait() {
        68.
    } else {
        60.
    };
    label("‹ CABINET", 8., 30., 13., muted());
    label("TERRAIN CANNON", title_x, title_y, title_size(), accent());
    let score_y = if compact { 39. } else { title_y + 25. };
    label(
        &format!(
            "Angle {}°  •  Power {}  •  Target {}",
            game.angle, game.power, game.target_health
        ),
        title_x,
        score_y,
        body_size(),
        muted(),
    );
    draw_rectangle(
        l.board.x,
        l.board.y,
        l.board.w,
        l.board.h,
        accessibility::board_fill(state.high_contrast),
    );
    for column in 0..WIDTH {
        let x = l.board.x + f32::from(column) * l.cell;
        let terrain_y = l.board.y + f32::from(game.terrain[usize::from(column)]) * l.cell;
        draw_rectangle(
            x,
            terrain_y,
            l.cell + 1.,
            l.board.bottom() - terrain_y,
            Color::new(0.30, 0.50, 0.30, 1.),
        );
        draw_line(
            x,
            terrain_y,
            x + l.cell,
            terrain_y,
            2.,
            Color::new(0.68, 0.82, 0.38, 1.),
        );
    }
    let cannon_x = l.board.x + 4.5 * l.cell;
    let cannon_y = l.board.y + f32::from(game.terrain[4]) * l.cell;
    draw_circle(
        cannon_x,
        cannon_y,
        l.cell * 0.55,
        Color::new(0.35, 0.85, 0.90, 1.),
    );
    let radians = f32::from(game.angle).to_radians();
    draw_line(
        cannon_x,
        cannon_y,
        cannon_x + radians.cos() * l.cell * 3.,
        cannon_y - radians.sin() * l.cell * 3.,
        3.,
        crate::theme::CREAM,
    );
    let target_x = l.board.x + (f32::from(game.target_x) + 0.5) * l.cell;
    let target_y = l.board.y + f32::from(game.terrain[usize::from(game.target_x)]) * l.cell;
    draw_rectangle(
        target_x - l.cell * 0.45,
        target_y - l.cell * 1.1,
        l.cell * 0.9,
        l.cell * 1.1,
        Color::new(0.96, 0.33, 0.30, 1.),
    );
    if let Some(shot) = game.shot {
        draw_circle(
            l.board.x + shot.x * l.cell,
            l.board.y + shot.y * l.cell,
            l.cell * 0.22,
            Color::new(1., 0.80, 0.20, 1.),
        );
    }
    let status = status_text(game);
    label(
        state.card_hint.as_deref().unwrap_or(&status),
        title_x,
        if crate::ui::is_portrait() {
            275.
        } else if crate::ui::is_compact_landscape() {
            310.
        } else {
            575.
        },
        body_size(),
        muted(),
    );
    button(
        l.angle_down,
        if compact { "A −" } else { "ANGLE −" },
        state.large_text,
    );
    button(
        l.angle_up,
        if compact { "A +" } else { "ANGLE +" },
        state.large_text,
    );
    button(
        l.power_down,
        if compact { "P −" } else { "POWER −" },
        state.large_text,
    );
    button(
        l.power_up,
        if compact { "P +" } else { "POWER +" },
        state.large_text,
    );
    button(l.fire, "FIRE", state.large_text);
    button(
        l.pause,
        if game.paused { "RESUME" } else { "PAUSE" },
        state.large_text,
    );
    button(l.undo, if compact { "U" } else { "UNDO" }, state.large_text);
    button(
        l.new_game,
        if compact { "NEW" } else { "NEW HILLS" },
        state.large_text,
    );
}

pub fn status_text(game: &TerrainCannon) -> String {
    match game.status {
        CannonStatus::Playing => {
            "Adjust the angle and power, then carve a path to the target".into()
        }
        CannonStatus::Won => "Target silenced — every impact reshaped the hill".into(),
        CannonStatus::Lost => "The shot cycle ended — set up a new hillside".into(),
    }
}
pub fn button(rect: Rect, value: &str, large_text: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, crate::theme::SURFACE);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent());
    let size = accessibility::text_size(10., large_text);
    crate::ui::draw_text(
        value,
        rect.x + (rect.w - crate::ui::measure_text(value, None, size as u16, 1.).width) * 0.5,
        rect.y + rect.h * 0.64,
        size,
        WHITE,
    );
}
pub fn label(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}
pub fn title_size() -> f32 {
    if crate::ui::is_portrait() {
        22.
    } else {
        29.
    }
}
pub fn body_size() -> f32 {
    if crate::ui::is_portrait() {
        9.
    } else {
        12.
    }
}
pub fn accent() -> Color {
    crate::theme::BRASS
}
pub fn muted() -> Color {
    crate::theme::SECONDARY
}
pub fn back_rect() -> Rect {
    Rect::new(0., 0., 115., 42.)
}
