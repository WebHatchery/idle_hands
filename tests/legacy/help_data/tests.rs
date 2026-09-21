//! Regression coverage for the tests module.

use idle_hands::testing::modules::help_data::*;

fn content() -> idle_hands::testing::content::GameContent {
    idle_hands::testing::data::GameData::load().unwrap().content
}

#[test]
fn help_copy_has_a_touch_first_intro_and_recovery_guidance() {
    let content = content();
    let paragraphs = paragraphs(&content);
    let navigation = navigation(&content);
    assert_eq!(paragraphs.len(), 4);
    assert!(paragraphs[1].contains("Tap"));
    assert!(paragraphs[2].contains("visible controls"));
    assert!(paragraphs[3].contains("visible touch control"));
    assert_eq!(navigation, ["TUTORIALS", "RULES", "CREDITS", "BACK"]);
    assert!(paragraphs.iter().all(|line| {
        let lower = line.to_ascii_lowercase();
        !lower.contains("keyboard") && !lower.contains("dismiss") && !lower.contains("confirm")
    }));
}
