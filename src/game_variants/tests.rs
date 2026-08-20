use super::*;
use crate::state::Screen;

#[test]
fn every_drawer_has_a_real_rotatable_rule_card() {
    let data = GameData::load().expect("embedded game data should load");
    for game in GameId::ALL {
        let mut state = AppState::default();
        state.screen = Screen::Game(game);
        let before = label(&state, game);
        assert!(!before.is_empty(), "missing label for {game:?}");
        cycle(&mut state, &data, game);
        let after = label(&state, game);
        assert_ne!(before, after, "variant did not advance for {game:?}");
    }
}
