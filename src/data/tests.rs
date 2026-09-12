//! Regression coverage for the tests module.

use super::*;

#[test]
fn embedded_data_loads() {
    let data = GameData::load().unwrap();
    let state = crate::state::AppState::new(&data);

    assert!(!data.config.game_name.is_empty());
    assert!(!data.config.display_name.is_empty());
    assert_eq!(
        state.games.snake.win_score,
        data.content.balance.arcade.snake_target_score
    );
    assert_eq!(
        state.games.breakout.target_level,
        data.content.balance.arcade.breakout_target_level
    );
    assert_eq!(
        state.games.hangman.max_wrong,
        data.content.balance.word_games.hangman_classic_wrong
    );
    assert_eq!(
        state.games.riddle_room.round_target,
        data.content.balance.misc_games.riddle_rounds
    );
}
