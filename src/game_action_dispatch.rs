//! Central action dispatcher for the cabinet runtime.

use super::{game_restart, Game};
use crate::card_hints;
use crate::{
    state::{AppState, Screen},
    ui,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShellActionResult {
    Continue,
    Stop,
    Unhandled,
}

#[path = "game_action_dispatch/group_1.rs"]
mod group_1;
#[path = "game_action_dispatch/group_10.rs"]
mod group_10;
#[path = "game_action_dispatch/group_2.rs"]
mod group_2;
#[path = "game_action_dispatch/group_3.rs"]
mod group_3;
#[path = "game_action_dispatch/group_4.rs"]
mod group_4;
#[path = "game_action_dispatch/group_5.rs"]
mod group_5;
#[path = "game_action_dispatch/group_6.rs"]
mod group_6;
#[path = "game_action_dispatch/group_7.rs"]
mod group_7;
#[path = "game_action_dispatch/group_8.rs"]
mod group_8;
#[path = "game_action_dispatch/group_9.rs"]
mod group_9;

impl Game {
    pub(super) fn apply(&mut self, action: ui::UiAction) {
        self.state.games.solitaire_peek = None;
        self.state.games.spider_solitaire_peek = None;
        let previous_screen = self.state.screen;
        if !self.confirmation_bypass
            && game_restart::requires_new_confirmation(action)
            && self.state.pending_restart.is_none()
        {
            self.state.pending_restart = Some(action);
            self.state.confirm_restart = true;
            return;
        }
        if !card_hints::is_hint(action) {
            self.state.card_hint = None;
        }
        if !crate::game_actions::is_shell(action) && self.apply_game_action(&action) {
            self.finish_action(previous_screen, action);
            return;
        }
        if self.apply_browse_action(action) {
            self.finish_action(previous_screen, action);
            return;
        }
        match self.apply_shell_action(action) {
            ShellActionResult::Continue => self.finish_action(previous_screen, action),
            ShellActionResult::Stop => {}
            ShellActionResult::Unhandled => unreachable!("shell action was not handled"),
        }
    }

    fn apply_shell_action(&mut self, action: ui::UiAction) -> ShellActionResult {
        match self.apply_shell_group_1(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_2(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_3(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_4(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_5(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_6(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_7(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_8(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_9(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        match self.apply_shell_group_10(action) {
            ShellActionResult::Unhandled => {}
            result => return result,
        }
        ShellActionResult::Unhandled
    }
}
