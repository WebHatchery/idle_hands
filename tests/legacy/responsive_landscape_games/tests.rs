//! Regression coverage for the tests module.

use idle_hands::testing::modules::responsive_landscape_games::*;

#[test]
fn landscape_nonogram_focus_maps_visible_cells_to_the_large_board() {
    let state = AppState {
        games: idle_hands::testing::game_store::GameStore {
            nonogram: idle_hands::testing::nonogram::Nonogram::new(
                idle_hands::testing::nonogram::NonogramPreset::Large,
            ),
            nonogram_zoomed: true,
            nonogram_focus: (6, 6),
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(matches!(
        nonogram_clicks(&state, vec2(401., 191.)).as_slice(),
        [UiAction::NonogramZoom]
    ));
    assert!(matches!(
        nonogram_clicks(&state, vec2(71., 89.)).as_slice(),
        [UiAction::NonogramCell(96)]
    ));
    assert!(matches!(
        nonogram_clicks(&state, vec2(591., 191.)).as_slice(),
        [UiAction::NonogramPan(-1, 0)]
    ));
    let actions = nonogram_drag_actions(&state, vec2(71., 89.), vec2(102., 89.));
    assert!(matches!(
        actions.as_slice(),
        [UiAction::NonogramCell(96), UiAction::NonogramCell(97)]
    ));
}
