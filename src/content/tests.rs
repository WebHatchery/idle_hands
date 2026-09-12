//! Regression coverage for the authored content contract.

use super::*;

#[test]
fn embedded_catalog_covers_every_runtime_surface() {
    let content = crate::data::GameData::load().unwrap().content;

    assert_eq!(content.games.len(), GameId::ALL.len());
    assert_eq!(content.tutorials.len(), GameId::ALL.len());
    assert_eq!(content.variants.len(), GameId::ALL.len());
    assert_eq!(content.hints.len(), GameId::ALL.len());
    assert_eq!(content.achievements.len(), GameId::ALL.len() + 2);
    assert_eq!(content.words.word_search.len(), 3);
    assert_eq!(content.words.riddles.len(), 5);
}

#[test]
fn achievement_order_is_semantically_validated() {
    let mut content = crate::data::GameData::load().unwrap().content;
    content.achievements.swap(1, 2);

    let error = content.validate().unwrap_err();
    assert!(error.contains("cabinet order"));
}

#[test]
fn variant_entries_reject_duplicate_ids() {
    let mut content = crate::data::GameData::load().unwrap().content;
    content.variants.get_mut("solitaire").unwrap()[1].id = "default".into();

    let error = content.validate().unwrap_err();
    assert!(error.contains("invalid entries"));
}
