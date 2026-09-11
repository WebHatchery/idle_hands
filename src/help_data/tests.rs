use super::*;

#[test]
fn help_copy_has_a_touch_first_intro_and_recovery_guidance() {
    assert_eq!(PARAGRAPHS.len(), 4);
    assert!(PARAGRAPHS[1].contains("Tap"));
    assert!(PARAGRAPHS[2].contains("visible controls"));
    assert!(PARAGRAPHS[3].contains("visible touch control"));
    assert_eq!(NAV_LABELS, ["TUTORIALS", "RULES", "CREDITS", "BACK"]);
    assert!(PARAGRAPHS.iter().all(|line| {
        let lower = line.to_ascii_lowercase();
        !lower.contains("keyboard") && !lower.contains("dismiss") && !lower.contains("confirm")
    }));
}
