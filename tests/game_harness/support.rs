//! Helpers shared by the game input contract tests.

pub fn is_recovery_action(action: &idle_hands::testing::UiAction) -> bool {
    let label = format!("{action:?}");
    matches!(
        label.as_str(),
        "Undo" | "Restart" | "Cancel" | "ConfirmRestart" | "New" | "Hint"
    ) || label.ends_with("Undo")
        || label.ends_with("New")
        || label.ends_with("Hint")
        || label.ends_with("Restart")
        || label.ends_with("Pause")
}
