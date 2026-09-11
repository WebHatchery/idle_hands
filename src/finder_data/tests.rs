use super::*;
use crate::state::GameId;

#[test]
fn rows_are_alphabetical_and_cover_the_collection() {
    let rows = rows(0);

    assert_eq!(rows.len(), GameId::ALL.len());
    assert!(rows
        .windows(2)
        .all(|pair| pair[0].title() <= pair[1].title()));
}

#[test]
fn alphabet_buckets_partition_titles() {
    let bucketed = (1..=4).map(|filter| rows(filter).len()).sum::<usize>();

    assert_eq!(bucketed, GameId::ALL.len());
    assert!(rows(1).iter().all(|game| matches_filter(*game, 1)));
    assert!(rows(4).iter().all(|game| matches_filter(*game, 4)));
}

#[test]
fn invalid_filters_fall_back_to_all() {
    assert_eq!(normalize_filter(99), 0);
    assert_eq!(filter_label(99), "ALL");
    assert_eq!(rows(99), rows(0));
}

#[test]
fn page_limits_are_safe_for_every_layout() {
    let state = AppState::default();
    crate::ui::with_desktop_layout(|| {
        assert_eq!(visible_count(), 12);
        assert!(page_label(&state).contains("OF 60"));
    });
    crate::ui::with_compact_landscape_layout(|| assert_eq!(visible_count(), 8));
    crate::ui::with_portrait_layout(|| assert_eq!(visible_count(), 8));
}
