//! Touch-selectable profile names for the cabinet identity plate.

use crate::state::AppState;

pub const NAMES: [&str; 8] = [
    "Cabinet Guest",
    "Quiet Solver",
    "Night Owl",
    "Puzzle Wanderer",
    "Cozy Strategist",
    "One More Round",
    "Cabinet Keeper",
    "Curious Tinkerer",
];

pub fn name(index: u8) -> &'static str {
    NAMES.get(index as usize).copied().unwrap_or(NAMES[0])
}

pub fn current_index(state: &AppState) -> Option<usize> {
    NAMES.iter().position(|name| *name == state.profile_name)
}

pub fn display_name(name: &str) -> String {
    let clean = name
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>();
    let clean = clean.trim();
    if clean.is_empty() {
        return NAMES[0].to_owned();
    }
    let mut display = clean.chars().take(23).collect::<String>();
    if clean.chars().count() > 23 {
        display.push('…');
    }
    display
}

#[cfg(test)]
mod tests;
