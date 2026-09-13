use super::{panel, text};
use crate::{cosmetics, state::AppState, ui::UiAction};
use macroquad::prelude::*;

pub fn draw_settings(state: &AppState) {
    let labels = crate::settings_data::accessibility_labels(state);
    let summary = crate::collection_summary::from_state(state);
    panel(
        Rect::new(20., 10., 804., 370.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("SETTINGS", 40., 45., 25., crate::theme::BRASS);
    text(
        &format!(
            "Profile: {}  -  Cosmetics: {}/{}",
            crate::profile_data::display_name(&state.profile_name),
            cosmetics::total_unlocked(summary.stamps),
            cosmetics::total_options()
        ),
        220.,
        43.,
        13.,
        WHITE,
    );
    text(
        &summary.progress_label(),
        220.,
        59.,
        10.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(450., 268., 160., 44.), crate::theme::SURFACE_DARK);
    text("EDIT NAME", 500., 296., 11., WHITE);
    for (index, row) in crate::settings_data::cosmetic_rows(state)
        .into_iter()
        .enumerate()
    {
        let rect = Rect::new(40., 68. + index as f32 * 48., 370., 44.);
        panel(rect, Color::new(0.16, 0.11, 0.24, 1.));
        text(row.kind.label(), rect.x + 12., rect.y + 20., 11., WHITE);
        text(
            row.option.name,
            rect.x + 190.,
            rect.y + 20.,
            11.,
            crate::theme::BRASS,
        );
        text(
            &format!(
                "{} / {} OPEN  ·  {}",
                row.unlocked,
                row.total,
                crate::settings_data::next_label(row)
            ),
            rect.x + 190.,
            rect.y + 36.,
            8.,
            crate::theme::SECONDARY,
        );
    }
    panel(Rect::new(450., 68., 160., 44.), crate::theme::SURFACE);
    text(
        &format!("SOUND {}", labels.sound.to_uppercase()),
        495.,
        96.,
        11.,
        WHITE,
    );
    panel(Rect::new(630., 68., 160., 44.), crate::theme::SURFACE);
    text(
        &format!("MOTION {}", labels.motion.to_uppercase()),
        670.,
        96.,
        11.,
        WHITE,
    );
    panel(
        Rect::new(450., 120., 160., 44.),
        Color::new(0.18, 0.26, 0.34, 1.),
    );
    text("SAVE NOW", 500., 148., 11., WHITE);
    panel(Rect::new(630., 120., 160., 44.), crate::theme::SURFACE);
    text("LOAD", 690., 148., 11., WHITE);
    panel(
        Rect::new(450., 172., 160., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("CONTRAST {}", labels.contrast.to_uppercase()),
        475.,
        200.,
        10.,
        WHITE,
    );
    panel(
        Rect::new(630., 172., 160., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("LARGE TEXT {}", labels.text.to_uppercase()),
        650.,
        200.,
        10.,
        WHITE,
    );
    panel(Rect::new(450., 220., 340., 44.), crate::theme::SURFACE);
    text(
        &format!(
            "VOLUME {} {}",
            labels.volume.to_uppercase(),
            crate::settings_data::volume_meter(state)
        ),
        570.,
        248.,
        11.,
        WHITE,
    );
    panel(Rect::new(40., 268., 160., 44.), crate::theme::MOSS_DARK);
    text("BACK", 98., 297., 12., WHITE);
    panel(
        Rect::new(220., 268., 160., 44.),
        Color::new(0.36, 0.16, 0.22, 1.),
    );
    text("RESET DATA", 267., 297., 11., WHITE);
    if state.confirm_reset {
        panel(
            Rect::new(250., 150., 350., 150.),
            Color::new(0.16, 0.08, 0.16, 0.99),
        );
        text("Reset the cabinet?", 330., 190., 20., WHITE);
        text(
            "This removes saves and records.",
            300.,
            220.,
            13.,
            crate::theme::CREAM,
        );
        panel(
            Rect::new(285., 242., 110., 44.),
            Color::new(0.22, 0.18, 0.35, 1.),
        );
        text("CANCEL", 315., 270., 11., WHITE);
        panel(
            Rect::new(455., 242., 110., 44.),
            Color::new(0.45, 0.20, 0.24, 1.),
        );
        text("RESET", 490., 270., 11., WHITE);
    }
}
pub fn settings_clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if state.confirm_reset {
        if crate::ui::hit(Rect::new(285., 242., 110., 44.), p) {
            return vec![UiAction::CancelResetData];
        }
        if crate::ui::hit(Rect::new(455., 242., 110., 44.), p) {
            return vec![UiAction::ConfirmResetData];
        }
        return vec![];
    }
    for (index, rect) in [
        Rect::new(40., 68., 370., 44.),
        Rect::new(40., 116., 370., 44.),
        Rect::new(40., 164., 370., 44.),
        Rect::new(40., 212., 370., 44.),
    ]
    .into_iter()
    .enumerate()
    {
        if crate::ui::hit(rect, p) {
            if let Some(action) = crate::settings_data::cosmetic_action(index) {
                return vec![action];
            }
        }
    }
    if crate::ui::hit(Rect::new(450., 68., 160., 44.), p) {
        return vec![UiAction::ToggleSound];
    }
    if crate::ui::hit(Rect::new(630., 68., 160., 44.), p) {
        return vec![UiAction::ToggleMotion];
    }
    if crate::ui::hit(Rect::new(450., 220., 340., 44.), p) {
        return vec![UiAction::CycleSoundVolume];
    }
    if crate::ui::hit(Rect::new(450., 172., 160., 44.), p) {
        return vec![UiAction::ToggleHighContrast];
    }
    if crate::ui::hit(Rect::new(630., 172., 160., 44.), p) {
        return vec![UiAction::ToggleLargeText];
    }
    if crate::ui::hit(Rect::new(450., 120., 160., 44.), p) {
        return vec![UiAction::Save];
    }
    if crate::ui::hit(Rect::new(630., 120., 160., 44.), p) {
        return vec![UiAction::Load];
    }
    if crate::ui::hit(Rect::new(40., 268., 160., 44.), p) {
        return vec![UiAction::Cabinet];
    }
    if crate::ui::hit(Rect::new(220., 268., 160., 44.), p) {
        return vec![UiAction::ResetData];
    }
    if crate::ui::hit(Rect::new(450., 268., 160., 44.), p) {
        return vec![UiAction::Profile];
    }
    vec![]
}
