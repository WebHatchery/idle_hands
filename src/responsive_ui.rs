//! Compact portrait cabinet and 2048 layouts.

use crate::{
    cosmetics, palette_ui,
    state::{AppState, Direction},
    ui::UiAction,
};
use macroquad::prelude::*;

pub const WIDTH: f32 = 360.;
pub const HEIGHT: f32 = 780.;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2., crate::theme::BORDER);
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    crate::ui::draw_text(value, x, y, crate::ui::readable_text_size(size), color);
}

pub fn draw_2048(state: &AppState) {
    let game = &state.game;
    panel(Rect::new(0., 0., 120., 48.), crate::theme::SURFACE_DARK);
    text("‹ CABINET", 16., 35., 15., Color::new(0.78, 0.70, 0.92, 1.));
    text("2048", 16., 82., 38., crate::theme::BRASS);
    text(
        &format!("Score {}  -  Best {}", game.score, game.best),
        18.,
        108.,
        14.,
        WHITE,
    );
    let board = Rect::new(20., 130., 320., 320.);
    panel(board, crate::accessibility::board_fill(state.high_contrast));
    for index in 0..16 {
        let rect = Rect::new(
            board.x + 8. + (index % 4) as f32 * 78.,
            board.y + 8. + (index / 4) as f32 * 78.,
            72.,
            72.,
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
            let size = if value < 100 { 25. } else { 19. };
            let width = crate::ui::measure_text(&label, None, size as u16, 1.).width;
            text(
                &label,
                rect.x + (rect.w - width) / 2.,
                rect.y + 45.,
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
        let rect = Rect::new(20. + index as f32 * 82., 475., 74., 46.);
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
    panel(Rect::new(20., 545., 150., 46.), crate::theme::SURFACE_DARK);
    text("UNDO", 70., 575., 15., WHITE);
    panel(Rect::new(190., 545., 150., 46.), crate::theme::SURFACE);
    text("NEW GAME", 220., 575., 14., WHITE);
    panel(Rect::new(20., 600., 150., 46.), crate::theme::SURFACE_DARK);
    text("HINT", 70., 630., 15., WHITE);
    text(
        state
            .card_hint
            .as_deref()
            .unwrap_or("Swipe the board or tap an arrow."),
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
    if crate::ui::hit(Rect::new(20., 545., 150., 46.), p) && state.game.can_undo() {
        return vec![UiAction::Undo];
    }
    if crate::ui::hit(Rect::new(190., 545., 150., 46.), p) {
        return vec![UiAction::Restart];
    }
    if crate::ui::hit(Rect::new(20., 600., 150., 46.), p) {
        return vec![UiAction::Game2048Hint];
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
        if crate::ui::hit(Rect::new(20. + index as f32 * 82., 475., 74., 46.), p) {
            return vec![UiAction::Move(*direction)];
        }
    }
    vec![]
}

pub fn draw_settings(state: &AppState) {
    panel(
        Rect::new(8., 30., 344., 700.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("SETTINGS", 22., 82., 30., crate::theme::BRASS);
    text(
        &format!("Profile: {}", state.profile_name),
        22.,
        120.,
        15.,
        WHITE,
    );
    for (y, label) in [
        (
            165.,
            format!("Card back: {}", cosmetics::card_back_name(state.card_back)),
        ),
        (
            215.,
            format!(
                "Board theme: {}",
                cosmetics::board_theme_name(state.board_theme)
            ),
        ),
        (
            265.,
            format!("Sound set: {}", cosmetics::sound_set_name(state.sound_set)),
        ),
        (
            315.,
            format!(
                "Decoration: {}",
                cosmetics::cabinet_decoration_name(state.cabinet_decoration)
            ),
        ),
    ] {
        panel(
            Rect::new(22., y - 28., 316., 44.),
            Color::new(0.16, 0.11, 0.24, 1.),
        );
        text(&label, 34., y, 14., WHITE);
    }
    panel(
        Rect::new(22., 360., 150., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("Sound: {}", if state.sound { "On" } else { "Off" }),
        35.,
        387.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(186., 360., 152., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Motion: {}",
            if state.reduced_motion {
                "Reduced"
            } else {
                "Full"
            }
        ),
        198.,
        387.,
        13.,
        WHITE,
    );
    panel(
        Rect::new(22., 465., 150., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Contrast: {}",
            if state.high_contrast { "On" } else { "Off" }
        ),
        35.,
        492.,
        12.,
        WHITE,
    );
    panel(
        Rect::new(186., 465., 152., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Text: {}",
            if state.large_text { "Large" } else { "Normal" }
        ),
        200.,
        492.,
        12.,
        WHITE,
    );
    text(
        "Tap a row to cycle unlocked cosmetics.",
        22.,
        445.,
        13.,
        crate::theme::SECONDARY,
    );
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
    for (rect, action) in [
        (Rect::new(22., 137., 316., 44.), UiAction::CycleCardBack),
        (Rect::new(22., 187., 316., 44.), UiAction::CycleBoardTheme),
        (Rect::new(22., 237., 316., 44.), UiAction::CycleSoundSet),
        (
            Rect::new(22., 287., 316., 44.),
            UiAction::CycleCabinetDecoration,
        ),
        (Rect::new(22., 360., 150., 44.), UiAction::ToggleSound),
        (Rect::new(186., 360., 152., 44.), UiAction::ToggleMotion),
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
