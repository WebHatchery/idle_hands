use super::*;
use crate::state::AppState;

#[test]
fn favorite_shelf_uses_a_touch_safe_capacity_at_each_size() {
    let mut state = AppState::default();
    state.favorites.fill(true);

    crate::ui::with_desktop_layout(|| {
        assert_eq!(visible_games(&state).len(), DESKTOP_VISIBLE_GAMES);
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert_eq!(visible_games(&state).len(), 10);
    });
    crate::ui::with_portrait_layout(|| {
        assert_eq!(visible_games(&state).len(), 8);
    });
}
