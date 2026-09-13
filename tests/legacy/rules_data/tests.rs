//! Regression coverage for the tests module.

use super::*;

#[test]
fn rule_filters_cycle_through_the_six_cabinet_categories() {
    assert_eq!(FILTERS, [0, 3, 4, 5, 6, 7, 8]);
    assert_eq!(next_filter(0), 3);
    assert_eq!(next_filter(8), 0);
    assert_eq!(filter_label(6), "Word");
    assert_eq!(normalize_filter(99), 0);
}

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
fn rule_summary_reports_the_active_drawer_count() {
    assert_eq!(
        summary_label(0),
        if cfg!(feature = "demo") {
            "60 DRAWERS · 30 OPEN · 30 FULL"
        } else {
            "60 DRAWERS"
        }
    );
    assert_eq!(
        summary_label(3),
        if cfg!(feature = "demo") {
            "11 DRAWERS · 5 OPEN · 6 FULL"
        } else {
            "11 DRAWERS"
        }
    );
}

#[test]
fn demo_rule_summary_counts_follow_the_curated_shelf() {
    assert_eq!(
        summary_label_for_build(3, true),
        "11 DRAWERS · 5 OPEN · 6 FULL"
    );
    assert_eq!(summary_label_for_build(3, false), "11 DRAWERS");
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
