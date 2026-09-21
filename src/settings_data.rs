//! Shared preference rows used by every responsive Settings surface.

use crate::{
    cosmetics::{CosmeticKind, CosmeticOption},
    state::AppState,
    ui_action::UiAction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CosmeticRow {
    pub kind: CosmeticKind,
    pub current: u8,
    pub option: CosmeticOption,
    pub unlocked: usize,
    pub total: usize,
    pub next_cost: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessibilityLabels {
    pub sound: &'static str,
    pub volume: &'static str,
    pub motion: &'static str,
    pub contrast: &'static str,
    pub text: &'static str,
}

pub fn cosmetic_rows(state: &AppState) -> [CosmeticRow; 4] {
    CosmeticKind::ALL.map(|kind| {
        let current = current_value(state, kind);
        let options = kind.options();
        CosmeticRow {
            kind,
            current,
            option: options[current as usize % options.len()],
            unlocked: kind.unlocked_count(state.stamps),
            total: options.len(),
            next_cost: kind.next_cost(state.stamps),
        }
    })
}

pub fn current_value(state: &AppState, kind: CosmeticKind) -> u8 {
    let current = match kind {
        CosmeticKind::CardBack => state.card_back,
        CosmeticKind::BoardTheme => state.board_theme,
        CosmeticKind::SoundSet => state.sound_set,
        CosmeticKind::CabinetDecoration => state.cabinet_decoration,
    };
    kind.normalize(current, state.stamps)
}

pub fn next_label(row: CosmeticRow) -> String {
    row.next_cost.map_or_else(
        || "ALL OPEN".to_owned(),
        |cost| format!("NEXT {cost} STAMPS"),
    )
}

pub fn cosmetic_action(index: usize) -> Option<UiAction> {
    match index {
        0 => Some(UiAction::CycleCardBack),
        1 => Some(UiAction::CycleBoardTheme),
        2 => Some(UiAction::CycleSoundSet),
        3 => Some(UiAction::CycleCabinetDecoration),
        _ => None,
    }
}

pub fn accessibility_labels(state: &AppState) -> AccessibilityLabels {
    AccessibilityLabels {
        sound: if state.sound { "On" } else { "Off" },
        volume: crate::audio_settings::label(state.sound, state.sound_level),
        motion: if state.reduced_motion {
            "Reduced"
        } else {
            "Full"
        },
        contrast: if state.high_contrast { "On" } else { "Off" },
        text: if state.large_text { "Large" } else { "Normal" },
    }
}

pub fn volume_meter(state: &AppState) -> &'static str {
    crate::audio_settings::meter(state.sound, state.sound_level)
}
