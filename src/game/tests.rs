use super::*;

#[test]
fn portrait_card_drag_dispatches_source_and_destination_actions() {
    let solitaire = crate::state::AppState {
        screen: Screen::Game(GameId::Solitaire),
        ..Default::default()
    };
    let actions = card_drag_actions(&solitaire, vec2(6., 205.), vec2(55., 205.), true, false);
    assert!(matches!(
        actions.as_slice(),
        [
            ui::UiAction::SolitaireTableau(0, 0),
            ui::UiAction::SolitaireTableau(1, 0),
        ]
    ));

    let freecell = crate::state::AppState {
        screen: Screen::Game(GameId::FreeCell),
        ..Default::default()
    };
    let actions = card_drag_actions(&freecell, vec2(6., 205.), vec2(55., 205.), true, false);
    assert!(matches!(
        actions.as_slice(),
        [
            ui::UiAction::FreeCellCascade(0, 0),
            ui::UiAction::FreeCellCascade(1, 0),
        ]
    ));
}

#[test]
fn card_drag_dispatch_ignores_non_card_screens() {
    let state = crate::state::AppState::default();
    assert!(card_drag_actions(&state, vec2(6., 205.), vec2(55., 205.), true, false).is_empty());
}
