use super::*;

#[test]
fn rule_filters_cycle_through_the_six_cabinet_categories() {
    assert_eq!(FILTERS, [0, 3, 4, 5, 6, 7, 8]);
    assert_eq!(next_filter(0), 3);
    assert_eq!(next_filter(8), 0);
    assert_eq!(filter_label(6), "Word");
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
