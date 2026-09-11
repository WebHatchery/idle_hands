use super::*;

#[test]
fn credits_copy_covers_identity_craft_touch_and_privacy() {
    assert_eq!(TITLE, "IDLE HANDS");
    assert_eq!(PARAGRAPHS.len(), 7);
    assert!(PARAGRAPHS[1].contains("Rust"));
    assert!(PARAGRAPHS[2].contains("touch"));
    assert!(PARAGRAPHS[3].contains("provenance"));
    assert!(PARAGRAPHS[4].contains("analytics are currently disabled"));
    assert!(PARAGRAPHS[5].contains("browser or Windows profile"));
    assert!(PARAGRAPHS
        .iter()
        .all(|paragraph| !paragraph.trim().is_empty()));
}
