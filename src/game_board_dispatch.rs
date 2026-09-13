//! Category-specific action routing for the cabinet game drawers.

use super::{game_arcade_actions, Game};
use crate::ui::UiAction;

#[path = "game_board_dispatch/group_1.rs"]
mod group_1;
#[path = "game_board_dispatch/group_10.rs"]
mod group_10;
#[path = "game_board_dispatch/group_11.rs"]
mod group_11;
#[path = "game_board_dispatch/group_12.rs"]
mod group_12;
#[path = "game_board_dispatch/group_13.rs"]
mod group_13;
#[path = "game_board_dispatch/group_2.rs"]
mod group_2;
#[path = "game_board_dispatch/group_3.rs"]
mod group_3;
#[path = "game_board_dispatch/group_4.rs"]
mod group_4;
#[path = "game_board_dispatch/group_5.rs"]
mod group_5;
#[path = "game_board_dispatch/group_6.rs"]
mod group_6;
#[path = "game_board_dispatch/group_7.rs"]
mod group_7;
#[path = "game_board_dispatch/group_8.rs"]
mod group_8;
#[path = "game_board_dispatch/group_9.rs"]
mod group_9;

impl Game {
    pub(super) fn apply_game_action(&mut self, action: &UiAction) -> bool {
        if game_arcade_actions::apply(self, action) {
            return true;
        }

        self.apply_game_action_group_1(action)
            || self.apply_game_action_group_2(action)
            || self.apply_game_action_group_3(action)
            || self.apply_game_action_group_4(action)
            || self.apply_game_action_group_5(action)
            || self.apply_game_action_group_6(action)
            || self.apply_game_action_group_7(action)
            || self.apply_game_action_group_8(action)
            || self.apply_game_action_group_9(action)
            || self.apply_game_action_group_10(action)
            || self.apply_game_action_group_11(action)
            || self.apply_game_action_group_12(action)
            || self.apply_game_action_group_13(action)
    }
}
