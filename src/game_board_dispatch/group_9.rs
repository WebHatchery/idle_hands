use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_9(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::MancalaLevel(level) => self.state.games.mancala.set_ai_level(*level),
            UiAction::MancalaVariant(variant) => {
                let seed = self.state.games.mancala.seed.wrapping_add(1);
                self.state.games.mancala.set_variant(*variant, seed);
            }
            UiAction::HanoiPeg(peg) => {
                self.state.games.hanoi.tap_peg(*peg);
            }
            UiAction::HanoiHint => {
                self.state.card_hint = Some(crate::card_hints::hanoi(&self.state));
                return true;
            }
            UiAction::HanoiUndo => {
                self.state.games.hanoi.undo();
            }
            UiAction::HanoiNew => {
                let seed = self.state.games.hanoi.seed.wrapping_add(1);
                self.state.games.hanoi.reset(seed);
            }
            UiAction::HanoiDisks(disks) => {
                let seed = self.state.games.hanoi.seed.wrapping_add(1);
                self.state.games.hanoi.set_disks(*disks, seed);
            }
            UiAction::NumberMatchTap(index) => {
                self.state.games.number_match.tap(*index);
            }
            UiAction::NumberMatchHint => {
                self.state.card_hint = Some(crate::card_hints::number_match(&self.state));
                return true;
            }
            UiAction::NumberMatchUndo => {
                self.state.games.number_match.undo();
            }
            UiAction::NumberMatchNew => {
                let seed = self.state.games.number_match.seed.wrapping_add(1);
                self.state.games.number_match.reset(seed);
            }
            UiAction::NumberMatchRemix => {
                self.state.games.number_match.remix();
            }
            UiAction::NumberMatchRule(rule) => {
                let seed = self.state.games.number_match.seed.wrapping_add(1);
                self.state.games.number_match.set_rule(*rule, seed);
            }
            UiAction::FloodColor(color) => {
                self.state.games.flood_it.choose(*color);
            }
            _ => return false,
        }
        true
    }
}
