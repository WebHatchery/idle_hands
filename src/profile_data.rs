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

#[cfg(test)]
mod tests;
