//! Compact portrait cabinet and 2048 layouts.

use crate::domain::Direction;
use crate::{game_2048::Game2048Size, palette_ui, state::AppState, ui::UiAction};
use macroquad::prelude::*;

#[cfg(test)]
#[path = "../tests/legacy/responsive_ui/tests.rs"]
mod tests;

pub const WIDTH: f32 = 360.;
pub const HEIGHT: f32 = 780.;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        crate::theme::drawer_surface(fill),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

pub fn draw_2048(state: &AppState) {
    let game = &state.games.game;
    panel(Rect::new(0., 0., 120., 48.), crate::theme::SURFACE_DARK);
    text("‹ CABINET", 16., 35., 15., crate::theme::BRASS);
    text("2048", 16., 82., 38., crate::theme::BRASS);
    for (index, board_size) in Game2048Size::ALL.iter().enumerate() {
        let rect = Rect::new(170. + index as f32 * 88., 94., 80., 36.);
        panel(
            rect,
            if *board_size == game.board_size {
                crate::theme::LEATHER
            } else {
                crate::theme::GAME_PANEL
            },
        );
        text(board_size.label(), rect.x + 12., rect.y + 23., 10., WHITE);
    }
    text(
        &format!("Score {}  ·  Best {}", game.score, game.best),
        18.,
        128.,
        14.,
        WHITE,
    );
    let board = Rect::new(20., 145., 320., 320.);
    panel(board, crate::accessibility::board_fill(state.high_contrast));
    let dimension = game.board_size.dimension();
    let tile_size = if dimension == 4 { 72. } else { 54. };
    let gap = 6.;
    let grid_side = tile_size * dimension as f32 + gap * (dimension - 1) as f32;
    let origin_x = board.x + (board.w - grid_side) * 0.5;
    let origin_y = board.y + (board.h - grid_side) * 0.5;
    for index in 0..game.cells.len() {
        let rect = Rect::new(
            origin_x + (index % dimension) as f32 * (tile_size + gap),
            origin_y + (index / dimension) as f32 * (tile_size + gap),
            tile_size,
            tile_size,
        );
        let value = game.cells[index];
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            palette_ui::tile_color(value, state.board_theme),
        );
        if value > 0 {
            let label = value.to_string();
            let size = if value < 100 {
                if dimension == 4 {
                    25.
                } else {
                    20.
                }
            } else if dimension == 4 {
                19.
            } else {
                17.
            };
            let width = crate::ui::measure_text(&label, None, size as u16, 1.).width;
            text(
                &label,
                rect.x + (rect.w - width) / 2.,
                rect.y + (rect.h + size * 0.36) * 0.5,
                size,
                WHITE,
            );
        }
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(20. + index as f32 * 82., 480., 74., 46.);
        panel(rect, crate::theme::SURFACE_DARK);
        text(
            ["UP", "LEFT", "DOWN", "RIGHT"][index],
            rect.x + 27.,
            rect.y + 32.,
            24.,
            crate::theme::BRASS,
        );
        let _ = direction;
    }
    panel(Rect::new(20., 550., 150., 46.), crate::theme::SURFACE_DARK);
    text("UNDO", 70., 580., 15., WHITE);
    panel(Rect::new(190., 550., 150., 46.), crate::theme::SURFACE);
    text("NEW GAME", 220., 580., 14., WHITE);
    panel(Rect::new(20., 610., 150., 46.), crate::theme::SURFACE_DARK);
    text("HINT", 70., 640., 15., WHITE);
    text(
        state.card_hint.as_deref().unwrap_or(""),
        18.,
        685.,
        14.,
        crate::theme::SECONDARY,
    );
    if state.confirm_restart {
        panel(
            Rect::new(25., 265., 310., 145.),
            Color::new(0.16, 0.09, 0.20, 1.),
        );
        text("Start a new board?", 58., 305., 20., WHITE);
        panel(Rect::new(45., 335., 120., 44.), crate::theme::MOSS_DARK);
        text("CANCEL", 76., 364., 14., WHITE);
        panel(
            Rect::new(195., 335., 120., 44.),
            Color::new(0.45, 0.22, 0.25, 1.),
        );
        text("START", 235., 364., 14., WHITE);
    }
}

pub fn game2048_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if crate::ui::hit(Rect::new(0., 0., 120., 48.), p) {
        return vec![UiAction::Cabinet];
    }
    if state.confirm_restart {
        if crate::ui::hit(Rect::new(45., 335., 120., 44.), p) {
            return vec![UiAction::Cancel];
        }
        if crate::ui::hit(Rect::new(195., 335., 120., 44.), p) {
            return vec![UiAction::ConfirmRestart];
        }
        return vec![];
    }
    if crate::ui::hit(Rect::new(20., 550., 150., 46.), p) && state.games.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if crate::ui::hit(Rect::new(190., 550., 150., 46.), p) {
        return vec![UiAction::Restart];
    }
    if crate::ui::hit(Rect::new(20., 610., 150., 46.), p) {
        return vec![UiAction::Game2048Hint];
    }
    for (index, board_size) in Game2048Size::ALL.iter().enumerate() {
        if crate::ui::hit(Rect::new(170. + index as f32 * 88., 94., 80., 36.), p)
            && state.games.game.board_size != *board_size
        {
            return vec![UiAction::Game2048Size(*board_size)];
        }
    }
    for (index, direction) in [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .enumerate()
    {
        if crate::ui::hit(Rect::new(20. + index as f32 * 82., 480., 74., 46.), p) {
            return vec![UiAction::Move(*direction)];
        }
    }
    vec![]
}

pub fn draw_settings(state: &AppState) {
    let labels = crate::settings_data::accessibility_labels(state);
    panel(
        Rect::new(8., 30., 344., 700.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("SETTINGS", 22., 82., 30., crate::theme::BRASS);
    text(
        &format!(
            "Profile: {}",
            crate::profile_data::display_name(&state.profile_name)
        ),
        22.,
        120.,
        15.,
        WHITE,
    );
    panel(Rect::new(186., 92., 152., 42.), crate::theme::SURFACE_DARK);
    text("EDIT NAME", 218., 118., 10., WHITE);
    text(
        &crate::collection_summary::from_state(state).progress_label(),
        22.,
        139.,
        8.,
        crate::theme::SECONDARY,
    );
    for (index, row) in crate::settings_data::cosmetic_rows(state)
        .into_iter()
        .enumerate()
    {
        let y = 175. + index as f32 * 50.;
        panel(
            Rect::new(22., y - 28., 316., 44.),
            Color::new(0.16, 0.11, 0.24, 1.),
        );
        text(
            &format!("{}: {}", row.kind.label(), row.option.name),
            34.,
            y,
            14.,
            WHITE,
        );
        text(
            &format!(
                "{} / {} OPEN  ·  {}",
                row.unlocked,
                row.total,
                crate::settings_data::next_label(row)
            ),
            34.,
            y + 15.,
            9.,
            crate::theme::SECONDARY,
        );
    }
    panel(
        Rect::new(22., 360., 150., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(&format!("Sound: {}", labels.sound), 35., 387., 13., WHITE);
    panel(
        Rect::new(186., 360., 152., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("Motion: {}", labels.motion),
        198.,
        387.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(22., 412., 316., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Volume: {} {}",
            labels.volume,
            crate::settings_data::volume_meter(state)
        ),
        35.,
        439.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(22., 465., 150., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("Contrast: {}", labels.contrast),
        35.,
        492.,
        12.,
        WHITE,
    );
    panel(
        Rect::new(186., 465., 152., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(&format!("Text: {}", labels.text), 200., 492., 12., WHITE);
    for (rect, label) in [
        (Rect::new(22., 665., 76., 44.), "BACK"),
        (Rect::new(108., 665., 76., 44.), "SAVE"),
        (Rect::new(194., 665., 76., 44.), "LOAD"),
        (Rect::new(280., 665., 58., 44.), "RESET"),
    ] {
        panel(rect, crate::theme::SURFACE);
        let width = crate::ui::measure_text(label, None, 11, 1.).width;
        text(
            label,
            rect.x + (rect.w - width) / 2.,
            rect.y + 27.,
            11.,
            WHITE,
        );
    }
    if state.confirm_reset {
        panel(
            Rect::new(20., 485., 320., 145.),
            Color::new(0.16, 0.08, 0.16, 0.99),
        );
        text("Reset the cabinet?", 42., 520., 20., WHITE);
        text(
            "This removes saves and records.",
            42.,
            548.,
            13.,
            crate::theme::CREAM,
        );
        panel(
            Rect::new(42., 568., 120., 44.),
            Color::new(0.22, 0.18, 0.35, 1.),
        );
        text("CANCEL", 75., 596., 12., WHITE);
        panel(
            Rect::new(198., 568., 120., 44.),
            Color::new(0.45, 0.20, 0.24, 1.),
        );
        text("RESET", 238., 596., 12., WHITE);
    }
}

pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        if crate::ui::hit(Rect::new(42., 568., 120., 44.), p) {
            return vec![UiAction::CancelResetData];
        }
        if crate::ui::hit(Rect::new(198., 568., 120., 44.), p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    if crate::ui::hit(Rect::new(22., 665., 76., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(108., 665., 76., 44.), p) {
        return vec![UiAction::Save];
    }
    if crate::ui::hit(Rect::new(194., 665., 76., 44.), p) {
        return vec![UiAction::Load];
    }
    if crate::ui::hit(Rect::new(280., 665., 58., 44.), p) {
        return vec![UiAction::ResetData];
    }
    if crate::ui::hit(Rect::new(186., 92., 152., 42.), p) {
        return vec![UiAction::Profile];
    }
    for (index, rect) in [
        Rect::new(22., 137., 316., 44.),
        Rect::new(22., 187., 316., 44.),
        Rect::new(22., 237., 316., 44.),
        Rect::new(22., 287., 316., 44.),
    ]
    .into_iter()
    .enumerate()
    {
        if rect.contains(p) {
            if let Some(action) = crate::settings_data::cosmetic_action(index) {
                return vec![action];
            }
        }
    }
    for (rect, action) in [
        (Rect::new(22., 360., 150., 44.), UiAction::ToggleSound),
        (Rect::new(186., 360., 152., 44.), UiAction::ToggleMotion),
        (Rect::new(22., 412., 316., 44.), UiAction::CycleSoundVolume),
        (
            Rect::new(22., 465., 150., 44.),
            UiAction::ToggleHighContrast,
        ),
        (Rect::new(186., 465., 152., 44.), UiAction::ToggleLargeText),
    ] {
        if rect.contains(p) {
            return vec![action];
        }
    }
    vec![]
}
