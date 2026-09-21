//! Regression coverage for the tests module.

use idle_hands::testing::modules::cabinet_data::*;
use idle_hands::testing::state::AppState;

#[test]
fn page_start_clamps_to_the_last_full_view() {
    assert_eq!(page_start(60, 0, 44), 0);
    assert_eq!(page_start(60, 44, 44), 16);
    assert_eq!(page_start(60, usize::MAX, 44), 16);
    assert_eq!(page_start(8, 7, 12), 0);
}

#[test]
fn range_label_describes_empty_and_partial_pages() {
    let state = AppState {
        cabinet_filter: 9,
        ..AppState::default()
    };
    let page = page(&state, 12);
    assert_eq!(page.start, 0);
    assert_eq!(page.end, 12.min(page.total));
    assert_eq!(
        range_label(&page),
        format!("1-{} OF {}", page.end, page.total)
    );
    assert!(!page.has_previous());
    assert_eq!(page.has_next(), page.total > page.end);

    let empty = CabinetPage {
        games: Vec::new(),
        start: 0,
        end: 0,
        total: 0,
    };
    assert_eq!(range_label(&empty), "0-0 OF 0");
    assert!(!empty.has_previous());
    assert!(!empty.has_next());
}
