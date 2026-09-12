//! Touch-selectable profile names loaded from the authored content catalog.

use crate::{content::GameContent, state::AppState};

pub fn names(content: &GameContent) -> &[String] {
    &content.labels.profile_names
}

pub fn name(content: &GameContent, index: u8) -> &str {
    names(content)
        .get(index as usize)
        .map(String::as_str)
        .unwrap_or_else(|| {
            names(content)
                .first()
                .map_or("Cabinet Guest", String::as_str)
        })
}

pub fn current_index(state: &AppState) -> Option<usize> {
    names(&state.content)
        .iter()
        .position(|name| *name == state.profile_name)
}

pub fn display_name(name: &str) -> String {
    let clean = name
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>();
    let clean = clean.trim();
    if clean.is_empty() {
        return "Cabinet Guest".to_owned();
    }
    let mut display = clean.chars().take(23).collect::<String>();
    if clean.chars().count() > 23 {
        display.push('…');
    }
    display
}

#[cfg(test)]
mod tests;
