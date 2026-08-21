use super::*;

#[test]
fn portrait_solitaire_routes_every_primary_tap_target() {
    let state = AppState::default();
    assert!(matches!(
        solitaire_clicks(&state, vec2(10., 120.)).as_slice(),
        [UiAction::SolitaireStock]
    ));
    assert!(matches!(
        solitaire_clicks(&state, vec2(60., 120.)).as_slice(),
        [UiAction::SolitaireWaste]
    ));
    assert!(matches!(
        solitaire_clicks(&state, vec2(160., 120.)).as_slice(),
        [UiAction::SolitaireFoundation(0)]
    ));
    assert!(matches!(
        solitaire_clicks(&state, vec2(6., 205.)).as_slice(),
        [UiAction::SolitaireTableau(0, 0)]
    ));
    assert!(matches!(
        solitaire_clicks(&state, vec2(10., 665.)).as_slice(),
        [UiAction::SolitaireHint]
    ));
    assert!(matches!(
        solitaire_clicks(&state, vec2(125., 665.)).as_slice(),
        [UiAction::SolitaireUndo]
    ));
    assert!(matches!(
        solitaire_clicks(&state, vec2(240., 665.)).as_slice(),
        [UiAction::SolitaireNew]
    ));
}

#[test]
fn portrait_freecell_routes_every_primary_tap_target() {
    let state = AppState::default();
    assert!(matches!(
        freecell_clicks(&state, vec2(10., 665.)).as_slice(),
        [UiAction::FreeCellHint]
    ));
    assert!(matches!(
        freecell_clicks(&state, vec2(125., 665.)).as_slice(),
        [UiAction::FreeCellUndo]
    ));
    assert!(matches!(
        freecell_clicks(&state, vec2(240., 665.)).as_slice(),
        [UiAction::FreeCellNew]
    ));
    assert!(matches!(
        freecell_clicks(&state, vec2(6., 120.)).as_slice(),
        [UiAction::FreeCellCell(0)]
    ));
    assert!(matches!(
        freecell_clicks(&state, vec2(195., 120.)).as_slice(),
        [UiAction::FreeCellFoundation(0)]
    ));
    assert!(matches!(
        freecell_clicks(&state, vec2(6., 205.)).as_slice(),
        [UiAction::FreeCellCascade(0, 0)]
    ));
}

#[test]
fn reversi_portrait_subtitle_is_short_enough_for_the_header_lane() {
    crate::ui::with_portrait_layout(|| {
        assert_eq!(reversi_subtitle(), "Tap a glowing square");
    });
    crate::ui::with_desktop_layout(|| {
        assert_eq!(
            reversi_subtitle(),
            "Turn the board, one careful move at a time"
        );
    });
}
