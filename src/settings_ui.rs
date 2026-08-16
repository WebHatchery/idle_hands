//! Settings screen and destructive reset confirmation.

use crate::{cosmetics, state::AppState, ui::UiAction};
use macroquad::prelude::*;

fn panel(rect: Rect, fill: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.,
        Color::new(0.45, 0.38, 0.65, 0.65),
    );
}
fn text(value: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(value, x, y, size, color);
}

pub fn draw_settings(state: &AppState) {
    panel(
        Rect::new(180., 60., 920., 600.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text(
        "SETTINGS",
        230.,
        125.,
        42.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        &format!(
            "Profile: {}  •  Stamps: {}",
            state.profile_name, state.stamps
        ),
        230.,
        185.,
        22.,
        WHITE,
    );
    text(
        &format!("Card back: {}", cosmetics::card_back_name(state.card_back)),
        230.,
        245.,
        20.,
        WHITE,
    );
    text(
        &format!(
            "Board theme: {}",
            cosmetics::board_theme_name(state.board_theme)
        ),
        230.,
        295.,
        20.,
        WHITE,
    );
    text(
        &format!("Sound set: {}", cosmetics::sound_set_name(state.sound_set)),
        230.,
        345.,
        20.,
        WHITE,
    );
    text(
        &format!(
            "Cabinet decoration: {}",
            cosmetics::cabinet_decoration_name(state.cabinet_decoration)
        ),
        230.,
        395.,
        20.,
        WHITE,
    );
    text(
        &format!(
            "Sound: {}  •  Reduced motion: {}",
            if state.sound { "On" } else { "Off" },
            if state.reduced_motion { "On" } else { "Off" }
        ),
        230.,
        445.,
        18.,
        WHITE,
    );
    panel(
        Rect::new(230., 470., 350., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "High contrast: {}",
            if state.high_contrast { "On" } else { "Off" }
        ),
        250.,
        496.,
        15.,
        WHITE,
    );
    panel(
        Rect::new(580., 470., 350., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!(
            "Large text: {}",
            if state.large_text { "On" } else { "Off" }
        ),
        600.,
        496.,
        15.,
        WHITE,
    );
    text(
        "Tap a cosmetic row to cycle through the items your stamps have opened.",
        230.,
        530.,
        17.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    panel(
        Rect::new(230., 560., 150., 48.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("BACK", 280., 591., 17., WHITE);
    panel(
        Rect::new(410., 560., 150., 48.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("SAVE NOW", 440., 591., 16., WHITE);
    panel(
        Rect::new(590., 560., 150., 48.),
        Color::new(0.22, 0.18, 0.35, 1.),
    );
    text("LOAD", 639., 591., 16., WHITE);
    panel(
        Rect::new(770., 560., 210., 48.),
        Color::new(0.36, 0.16, 0.22, 1.),
    );
    text("RESET DATA", 805., 591., 15., WHITE);
    if state.confirm_reset {
        panel(
            Rect::new(360., 260., 560., 190.),
            Color::new(0.16, 0.08, 0.16, 0.99),
        );
        text("Reset the cabinet?", 425., 315., 27., WHITE);
        text(
            "This removes saves, records, and tutorial progress.",
            425.,
            350.,
            17.,
            Color::new(0.78, 0.73, 0.86, 1.),
        );
        panel(
            Rect::new(430., 375., 150., 44.),
            Color::new(0.22, 0.18, 0.35, 1.),
        );
        text("CANCEL", 472., 404., 16., WHITE);
        panel(
            Rect::new(650., 375., 150., 44.),
            Color::new(0.45, 0.20, 0.24, 1.),
        );
        text("RESET", 698., 404., 16., WHITE);
    }
}

pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        if crate::ui::hit(Rect::new(430., 375., 150., 44.), p) {
            return vec![UiAction::CancelResetData];
        }
        if crate::ui::hit(Rect::new(650., 375., 150., 44.), p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    let mut actions = Vec::new();
    if crate::ui::hit(Rect::new(230., 560., 150., 48.), p) {
        actions.push(UiAction::Cabinet);
    }
    if crate::ui::hit(Rect::new(410., 560., 150., 48.), p) {
        actions.push(UiAction::Save);
    }
    if crate::ui::hit(Rect::new(590., 560., 150., 48.), p) {
        actions.push(UiAction::Load);
    }
    if crate::ui::hit(Rect::new(230., 215., 700., 45.), p) {
        actions.push(UiAction::CycleCardBack);
    }
    if crate::ui::hit(Rect::new(230., 265., 700., 45.), p) {
        actions.push(UiAction::CycleBoardTheme);
    }
    if crate::ui::hit(Rect::new(230., 315., 700., 45.), p) {
        actions.push(UiAction::CycleSoundSet);
    }
    if crate::ui::hit(Rect::new(230., 365., 700., 45.), p) {
        actions.push(UiAction::CycleCabinetDecoration);
    }
    if crate::ui::hit(Rect::new(230., 415., 350., 45.), p) {
        actions.push(UiAction::ToggleSound);
    }
    if crate::ui::hit(Rect::new(580., 415., 350., 45.), p) {
        actions.push(UiAction::ToggleMotion);
    }
    if crate::ui::hit(Rect::new(230., 470., 350., 44.), p) {
        actions.push(UiAction::ToggleHighContrast);
    }
    if crate::ui::hit(Rect::new(580., 470., 350., 44.), p) {
        actions.push(UiAction::ToggleLargeText);
    }
    if crate::ui::hit(Rect::new(770., 560., 210., 48.), p) {
        actions.push(UiAction::ResetData);
    }
    actions
}
