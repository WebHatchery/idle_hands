//! Settings screen and destructive reset confirmation.

use crate::{state::AppState, ui::UiAction};
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
        Rect::new(240., 100., 800., 500.),
        Color::new(0.08, 0.06, 0.14, 1.),
    );
    text(
        "SETTINGS",
        290.,
        170.,
        42.,
        Color::new(0.98, 0.83, 0.45, 1.),
    );
    text(
        &format!("Profile: {}", state.profile_name),
        290.,
        235.,
        22.,
        WHITE,
    );
    text(
        &format!("Sound: {}", if state.sound { "On" } else { "Off" }),
        290.,
        295.,
        20.,
        WHITE,
    );
    text(
        &format!(
            "Reduced motion: {}",
            if state.reduced_motion { "On" } else { "Off" }
        ),
        290.,
        355.,
        20.,
        WHITE,
    );
    text(
        "Settings are saved per profile in the collection shell.",
        290.,
        430.,
        17.,
        Color::new(0.68, 0.63, 0.78, 1.),
    );
    panel(
        Rect::new(290., 490., 150., 48.),
        Color::new(0.25, 0.16, 0.32, 1.),
    );
    text("BACK", 340., 521., 17., WHITE);
    panel(
        Rect::new(470., 490., 150., 48.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("SAVE NOW", 500., 521., 16., WHITE);
    panel(
        Rect::new(650., 490., 150., 48.),
        Color::new(0.22, 0.18, 0.35, 1.),
    );
    text("LOAD", 699., 521., 16., WHITE);
    panel(
        Rect::new(830., 490., 170., 48.),
        Color::new(0.36, 0.16, 0.22, 1.),
    );
    text("RESET DATA", 852., 521., 15., WHITE);
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
        if Rect::new(430., 375., 150., 44.).contains(p) {
            return vec![UiAction::CancelResetData];
        }
        if Rect::new(650., 375., 150., 44.).contains(p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    let mut actions = Vec::new();
    if Rect::new(290., 490., 150., 48.).contains(p) {
        actions.push(UiAction::Cabinet);
    }
    if Rect::new(470., 490., 150., 48.).contains(p) {
        actions.push(UiAction::Save);
    }
    if Rect::new(650., 490., 150., 48.).contains(p) {
        actions.push(UiAction::Load);
    }
    if Rect::new(290., 270., 250., 45.).contains(p) {
        actions.push(UiAction::ToggleSound);
    }
    if Rect::new(290., 330., 300., 45.).contains(p) {
        actions.push(UiAction::ToggleMotion);
    }
    if Rect::new(830., 490., 170., 48.).contains(p) {
        actions.push(UiAction::ResetData);
    }
    actions
}
