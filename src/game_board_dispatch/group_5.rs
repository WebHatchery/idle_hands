use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_5(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::TriPeaksTap(index) => {
                self.state.games.tri_peaks.tap(*index);
            }
            UiAction::TriPeaksStock => {
                self.state.games.tri_peaks.draw_stock();
            }
            UiAction::TriPeaksHint => {
                self.state.card_hint = Some(crate::card_hints::tri_peaks(&self.state));
            }
            UiAction::TriPeaksUndo => {
                self.state.games.tri_peaks.undo();
            }
            UiAction::TriPeaksNew => {
                let seed = self.state.games.tri_peaks.seed.wrapping_add(1);
                self.state.games.tri_peaks.reset(seed);
            }
            UiAction::TriPeaksRule(rule) => {
                let seed = self.state.games.tri_peaks.seed.wrapping_add(1);
                self.state.games.tri_peaks.set_rule(*rule, seed);
            }
            UiAction::TriPeaksBridge => {
                self.state.games.tri_peaks.toggle_bridge();
            }
            UiAction::NimSelect(heap) => {
                self.state.games.nim.select_heap(*heap);
            }
            UiAction::NimTake(amount) => {
                self.state.games.nim.take(*amount);
            }
            UiAction::NimHint => {
                self.state.card_hint = Some(crate::card_hints::nim(&self.state));
            }
            UiAction::NimUndo => {
                self.state.games.nim.undo();
            }
            UiAction::NimNew => {
                let seed = self.state.games.nim.seed.wrapping_add(1);
                self.state.games.nim.reset(seed);
            }
            UiAction::NimRule(rule) => {
                let seed = self.state.games.nim.seed.wrapping_add(1);
                self.state.games.nim.set_rule(*rule, seed);
            }
            UiAction::DungeonCell(index) => {
                if self.state.mine_flag_mode {
                    self.state.games.dungeon_sweeper.toggle_flag(*index);
                } else {
                    self.state.games.dungeon_sweeper.reveal(*index);
                }
            }
            _ => return false,
        }
        true
    }
}
