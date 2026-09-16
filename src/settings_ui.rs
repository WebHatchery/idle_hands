//! Settings screen and destructive reset confirmation.

use crate::{cosmetics, state::AppState, ui::UiAction};
use macroquad::prelude::*;

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

pub fn draw_settings(state: &AppState) {
    let labels = crate::settings_data::accessibility_labels(state);
    let summary = crate::collection_summary::from_state(state);
    panel(
        Rect::new(180., 60., 920., 600.),
        crate::theme::BACKGROUND_DEEP,
    );
    text("SETTINGS", 230., 125., 42., crate::theme::BRASS);
    panel(Rect::new(230., 145., 700., 55.), crate::theme::SURFACE_DARK);
    text(
        &format!(
            "Profile: {}  •  Cosmetics open: {}/{}",
            crate::profile_data::display_name(&state.profile_name),
            cosmetics::total_unlocked(summary.stamps),
            cosmetics::total_options()
        ),
        230.,
        185.,
        22.,
        WHITE,
    );
    text("EDIT NAME", 820., 185., 12., crate::theme::BRASS);
    text(
        &summary.progress_label(),
        230.,
        214.,
        16.,
        crate::theme::SECONDARY,
    );
    for (index, row) in crate::settings_data::cosmetic_rows(state)
        .into_iter()
        .enumerate()
    {
        text(
            &format!("{}: {}", row.kind.label(), row.option.name),
            230.,
            245. + index as f32 * 50.,
            20.,
            WHITE,
        );
        text(
            &format!(
                "{}/{} OPEN  ·  {}",
                row.unlocked,
                row.total,
                crate::settings_data::next_label(row)
            ),
            730.,
            245. + index as f32 * 50.,
            12.,
            crate::theme::SECONDARY,
        );
    }
    panel(Rect::new(230., 415., 160., 44.), crate::theme::SURFACE);
    text(
        &format!("SOUND {}", labels.sound.to_uppercase()),
        270.,
        443.,
        12.,
        WHITE,
    );
    panel(Rect::new(410., 415., 160., 44.), crate::theme::SURFACE);
    text(
        &format!(
            "VOLUME {} {}",
            labels.volume.to_uppercase(),
            crate::settings_data::volume_meter(state)
        ),
        430.,
        443.,
        11.,
        WHITE,
    );
    panel(Rect::new(590., 415., 340., 44.), crate::theme::SURFACE);
    text(
        &format!("REDUCED MOTION {}", labels.motion.to_uppercase()),
        620.,
        443.,
        12.,
        WHITE,
    );
    panel(
        Rect::new(230., 470., 350., 44.),
        Color::new(0.16, 0.11, 0.24, 1.),
    );
    text(
        &format!("High contrast: {}", labels.contrast),
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
        &format!("Large text: {}", labels.text),
        600.,
        496.,
        15.,
        WHITE,
    );
    text(
        "Tap a cosmetic row to cycle open items; each row shows its next stamp threshold.",
        230.,
        530.,
        17.,
        crate::theme::SECONDARY,
    );
    panel(Rect::new(230., 560., 150., 48.), crate::theme::MOSS_DARK);
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
            crate::theme::CREAM,
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
    if crate::ui::hit(Rect::new(230., 145., 700., 55.), p) {
        actions.push(UiAction::Profile);
        return actions;
    }
    if crate::ui::hit(Rect::new(230., 560., 150., 48.), p) {
        actions.push(UiAction::Cabinet);
    }
    if crate::ui::hit(Rect::new(410., 560., 150., 48.), p) {
        actions.push(UiAction::Save);
    }
    if crate::ui::hit(Rect::new(590., 560., 150., 48.), p) {
        actions.push(UiAction::Load);
    }
    for (index, rect) in [
        Rect::new(230., 215., 700., 45.),
        Rect::new(230., 265., 700., 45.),
        Rect::new(230., 315., 700., 45.),
        Rect::new(230., 365., 700., 45.),
    ]
    .into_iter()
    .enumerate()
    {
        if crate::ui::hit(rect, p) {
            if let Some(action) = crate::settings_data::cosmetic_action(index) {
                actions.push(action);
            }
        }
    }
    if crate::ui::hit(Rect::new(230., 415., 160., 44.), p) {
        actions.push(UiAction::ToggleSound);
    }
    if crate::ui::hit(Rect::new(410., 415., 160., 44.), p) {
        actions.push(UiAction::CycleSoundVolume);
    }
    if crate::ui::hit(Rect::new(590., 415., 340., 44.), p) {
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
