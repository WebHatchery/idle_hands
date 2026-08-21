use super::support;
use crate::state::GameId;

pub const GAME: GameId = GameId::OneRoomRoguelike;

#[test]
fn host_contract_is_complete() {
    support::assert_game_contract(GAME);
}

#[test]
fn desktop_long_game_is_stable() {
    super::desktop_long_game::run(GAME);
}

#[test]
fn desktop_ui_exposes_a_tap_target() {
    super::desktop_ui::run(GAME);
}
