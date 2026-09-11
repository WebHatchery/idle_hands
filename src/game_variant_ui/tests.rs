use super::*;
use crate::state::{AppState, GameId, Screen};

#[test]
fn rule_card_is_touchable_in_every_layout() {
    let state = AppState {
        screen: Screen::Game(GameId::Game2048),
        ..Default::default()
    };

    crate::ui::with_desktop_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
    crate::ui::with_portrait_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
    crate::ui::with_compact_landscape_layout(|| {
        assert!(clicks(&state, button_rect().center()));
    });
}
