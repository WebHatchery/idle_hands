use super::support;
use crate::state::GameId;

pub const GAME: GameId = GameId::MunchMaze;

#[test]
fn host_contract_is_complete() {
    support::assert_game_contract(GAME);
}
