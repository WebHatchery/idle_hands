//! Stamp-gated cosmetic choices. Cosmetics never participate in game rules.

use macroquad::prelude::Color;

pub const CARD_BACKS: [&str; 3] = ["Plum", "Moss", "Midnight"];
pub const BOARD_THEMES: [&str; 3] = ["Plum felt", "Moss felt", "Dawn paper"];
pub const SOUND_SETS: [&str; 3] = ["Quiet room", "Rain on glass", "Late library"];
pub const CABINET_DECORATIONS: [&str; 3] = ["Brass key", "Pressed fern", "Moon card"];

const CARD_BACK_COSTS: [u16; 3] = [0, 2, 5];
const BOARD_THEME_COSTS: [u16; 3] = [0, 3, 7];
const SOUND_SET_COSTS: [u16; 3] = [0, 4, 8];
const CABINET_DECORATION_COSTS: [u16; 3] = [0, 4, 8];

pub fn card_back_name(index: u8) -> &'static str {
    CARD_BACKS[index as usize % CARD_BACKS.len()]
}
pub fn board_theme_name(index: u8) -> &'static str {
    BOARD_THEMES[index as usize % BOARD_THEMES.len()]
}
pub fn sound_set_name(index: u8) -> &'static str {
    SOUND_SETS[index as usize % SOUND_SETS.len()]
}
pub fn cabinet_decoration_name(index: u8) -> &'static str {
    CABINET_DECORATIONS[index as usize % CABINET_DECORATIONS.len()]
}

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
        1 => Color::new(0.135, 0.165, 0.105, 1.),
        2 => Color::new(0.255, 0.205, 0.155, 1.),
        _ => crate::theme::BACKGROUND,
    }
}

pub fn cabinet_accent(decoration: u8) -> Color {
    match decoration as usize % CABINET_DECORATIONS.len() {
        1 => crate::theme::MOSS,
        2 => Color::new(0.66, 0.57, 0.38, 1.),
        _ => crate::theme::BRASS,
    }
}

pub fn card_back_colors(back: u8) -> (Color, Color) {
    match back as usize % CARD_BACKS.len() {
        1 => (
            Color::new(0.16, 0.34, 0.24, 1.),
            Color::new(0.58, 0.88, 0.55, 1.),
        ),
        2 => (
            Color::new(0.07, 0.10, 0.20, 1.),
            Color::new(0.50, 0.72, 0.95, 1.),
        ),
        _ => (
            Color::new(0.20, 0.13, 0.30, 1.),
            Color::new(0.75, 0.55, 0.90, 1.),
        ),
    }
}

#[cfg(test)]
mod tests;
