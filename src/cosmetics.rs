//! Stamp-gated cosmetic choices. Cosmetics never participate in game rules.

use macroquad::prelude::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CosmeticOption {
    pub name: &'static str,
    pub cost: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmeticKind {
    CardBack,
    BoardTheme,
    SoundSet,
    CabinetDecoration,
}

impl CosmeticKind {
    pub const ALL: [Self; 4] = [
        Self::CardBack,
        Self::BoardTheme,
        Self::SoundSet,
        Self::CabinetDecoration,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::CardBack => "CARD BACK",
            Self::BoardTheme => "BOARD THEME",
            Self::SoundSet => "SOUND SET",
            Self::CabinetDecoration => "CABINET DECOR",
        }
    }

    pub fn options(self) -> &'static [CosmeticOption] {
        match self {
            Self::CardBack => &CARD_BACKS,
            Self::BoardTheme => &BOARD_THEMES,
            Self::SoundSet => &SOUND_SETS,
            Self::CabinetDecoration => &CABINET_DECORATIONS,
        }
    }

    pub fn unlocked_count(self, stamps: u16) -> usize {
        self.options()
            .iter()
            .filter(|option| option.cost <= stamps)
            .count()
    }

    pub fn next_cost(self, stamps: u16) -> Option<u16> {
        self.options()
            .iter()
            .map(|option| option.cost)
            .find(|&cost| cost > stamps)
    }
}

pub fn total_options() -> usize {
    CosmeticKind::ALL
        .into_iter()
        .map(|kind| kind.options().len())
        .sum()
}

pub fn total_unlocked(stamps: u16) -> usize {
    CosmeticKind::ALL
        .into_iter()
        .map(|kind| kind.unlocked_count(stamps))
        .sum()
}

pub const CARD_BACKS: [CosmeticOption; 3] = [
    CosmeticOption {
        name: "Plum",
        cost: 0,
    },
    CosmeticOption {
        name: "Moss",
        cost: 2,
    },
    CosmeticOption {
        name: "Midnight",
        cost: 5,
    },
];
pub const BOARD_THEMES: [CosmeticOption; 3] = [
    CosmeticOption {
        name: "Walnut felt",
        cost: 0,
    },
    CosmeticOption {
        name: "Moss felt",
        cost: 3,
    },
    CosmeticOption {
        name: "Dawn paper",
        cost: 7,
    },
];
pub const SOUND_SETS: [CosmeticOption; 3] = [
    CosmeticOption {
        name: "Soft room",
        cost: 0,
    },
    CosmeticOption {
        name: "Rain on glass",
        cost: 4,
    },
    CosmeticOption {
        name: "Late library",
        cost: 8,
    },
];
pub const CABINET_DECORATIONS: [CosmeticOption; 3] = [
    CosmeticOption {
        name: "Brass key",
        cost: 0,
    },
    CosmeticOption {
        name: "Pressed fern",
        cost: 4,
    },
    CosmeticOption {
        name: "Moon card",
        cost: 8,
    },
];

const CARD_BACK_COSTS: [u16; 3] = [0, 2, 5];
const BOARD_THEME_COSTS: [u16; 3] = [0, 3, 7];
const SOUND_SET_COSTS: [u16; 3] = [0, 4, 8];
const CABINET_DECORATION_COSTS: [u16; 3] = [0, 4, 8];

pub fn next_card_back(current: u8, stamps: u16) -> u8 {
    next_unlocked(current, stamps, &CARD_BACK_COSTS)
}
pub fn next_board_theme(current: u8, stamps: u16) -> u8 {
    next_unlocked(current, stamps, &BOARD_THEME_COSTS)
}
pub fn next_sound_set(current: u8, stamps: u16) -> u8 {
    next_unlocked(current, stamps, &SOUND_SET_COSTS)
}
pub fn next_cabinet_decoration(current: u8, stamps: u16) -> u8 {
    next_unlocked(current, stamps, &CABINET_DECORATION_COSTS)
}

fn next_unlocked(current: u8, stamps: u16, costs: &[u16]) -> u8 {
    let start = current as usize % costs.len();
    (1..=costs.len())
        .map(|offset| (start + offset) % costs.len())
        .find(|&index| costs[index] <= stamps)
        .map_or(start as u8, |index| index as u8)
}

pub fn background(theme: u8) -> Color {
    match theme as usize % BOARD_THEMES.len() {
        1 => crate::theme::MOSS_DARK,
        2 => crate::theme::WALNUT,
        _ => crate::theme::BACKGROUND,
    }
}

pub fn card_back_colors(back: u8) -> (Color, Color) {
    match back as usize % CARD_BACKS.len() {
        1 => (crate::theme::MOSS_DARK, crate::theme::MOSS),
        2 => (crate::theme::SLATE_BRONZE, crate::theme::BRASS),
        _ => (crate::theme::LEATHER, crate::theme::BRASS),
    }
}

#[cfg(test)]
mod tests;
