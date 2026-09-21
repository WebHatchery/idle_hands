//! Regression coverage for the tests module.

use idle_hands::testing::modules::rules_data::*;

#[test]
fn filtered_rule_rows_cover_each_category_without_crossovers() {
    let all = rows(0);
    assert_eq!(all.len(), GameId::ALL.len());
    for filter in FILTERS.into_iter().skip(1) {
        let filtered = rows(filter);
        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|row| row.category == filter));
    }
}

#[test]
fn rule_page_labels_clamp_empty_and_end_windows() {
    assert_eq!(page_label(0, 0, 8), "PAGE 1 / 1");
    assert_eq!(page_label(8, 15, 8), "PAGE 2 / 2");
    assert_eq!(page_label(99, 15, 8), "PAGE 2 / 2");
}

#[test]
fn page_rows_keep_the_canonical_game_identity() {
    let page = page_rows(6, 1, 2);

    assert_eq!(page.len(), 2);
    assert_eq!(page[0].game, GameId::WordSearch);
    assert_eq!(page[1].game, GameId::Hangman);
}
