//! Regression coverage for the tests module.

use super::*;
use crate::state::AppState;

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

#[test]
fn layout_page_sizes_match_the_visible_shelves() {
    assert_eq!(page_size_for_layout(false, false), DESKTOP_PAGE_SIZE);
    assert_eq!(page_size_for_layout(true, false), PORTRAIT_PAGE_SIZE);
    assert_eq!(
        page_size_for_layout(false, true),
        COMPACT_LANDSCAPE_PAGE_SIZE
    );
}

#[test]
fn favorite_count_tracks_only_marked_games() {
    assert_eq!(favorite_count(&AppState::default()), 0);

    let mut state = AppState::default();
    state.favorites[0] = true;
    state.favorites[3] = true;
    assert_eq!(favorite_count(&state), 2);
}

#[test]
fn category_initials_cover_known_drawers_and_unknown_filters() {
    assert_eq!(category_initial(3), "C");
    assert_eq!(category_initial(4), "L");
    assert_eq!(category_initial(5), "B");
    assert_eq!(category_initial(6), "W");
    assert_eq!(category_initial(7), "A");
    assert_eq!(category_initial(8), "M");
    assert_eq!(category_initial(9), "?");
}
