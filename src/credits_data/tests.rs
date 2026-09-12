//! Regression coverage for the tests module.

use super::*;

fn content() -> crate::content::GameContent {
    crate::data::GameData::load().unwrap().content
}

#[test]
fn credits_copy_covers_identity_craft_touch_and_privacy() {
    let content = content();
    let paragraphs = paragraphs(&content);
    assert_eq!(title(&content), "IDLE HANDS");
    assert_eq!(paragraphs.len(), 7);
    assert!(paragraphs[1].contains("Rust"));
    assert!(paragraphs[2].contains("touch"));
    assert!(paragraphs[3].contains("provenance"));
    assert!(paragraphs[4].contains("analytics are currently disabled"));
    assert!(paragraphs[5].contains("browser or Windows profile"));
    assert!(paragraphs
        .iter()
        .all(|paragraph| !paragraph.trim().is_empty()));
}
