use crate::state::AppState;

pub(super) fn apply(state: &mut AppState, scene: &str) {
    state.rules_filter = match scene {
        "rules_logic" => 4,
        "rules_word" => 6,
        _ => return,
    };
    state.library_scroll = 0;
}

#[cfg(test)]
#[path = "game_capture_rules/tests.rs"]
mod tests;
