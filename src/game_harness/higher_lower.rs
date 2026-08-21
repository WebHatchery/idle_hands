use super::support;
use crate::state::GameId;

pub const GAME: GameId = GameId::HigherLower;

#[test]
fn host_contract_is_complete() {
    support::assert_game_contract(GAME);
}
