//! Deterministic capture scenarios for the cabinet game suite.

use idle_hands::testing::GameId;

pub const GAME: GameId = GameId::Hanoi;

#[test]
fn desktop_long_game_is_stable() {
    super::desktop_long_game::run(GAME);
}
