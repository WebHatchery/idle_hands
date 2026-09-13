use super::super::{game_board_actions::fresh_seed, Game};
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_10(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::FloodHint => {
                self.state.card_hint = Some(crate::card_hints::flood_it(&self.state));
                return true;
            }
            UiAction::FloodUndo => {
                self.state.games.flood_it.undo();
            }
            UiAction::FloodNew => {
                let seed = self.state.games.flood_it.seed.wrapping_add(1);
                self.state.games.flood_it.reset(seed);
            }
            UiAction::FloodSurge => {
                self.state.games.flood_it.use_surge();
            }
            UiAction::FloodDifficulty(difficulty) => {
                let seed = self.state.games.flood_it.seed.wrapping_add(1);
                self.state.games.flood_it = crate::flood_it::FloodIt::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.flood_it,
                );
            }
            UiAction::ColorSortTap(tube) => {
                self.state.games.color_sort.tap_tube(*tube);
            }
            UiAction::ColorSortHint => {
                self.state.card_hint = Some(crate::card_hints::color_sort(&self.state));
                return true;
            }
            UiAction::ColorSortUndo => {
                self.state.games.color_sort.undo();
            }
            UiAction::ColorSortNew => {
                let seed = fresh_seed(self.state.games.color_sort.seed);
                self.state.games.color_sort.reset(seed);
            }
            UiAction::ColorSortDifficulty(difficulty) => {
                let seed = fresh_seed(self.state.games.color_sort.seed);
                self.state.games.color_sort = crate::color_sort::ColorSort::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.color_sort,
                );
            }
            UiAction::BattleshipFire(cell) => {
                self.state.games.battleship.fire(*cell);
            }
            UiAction::BattleshipSonar => {
                self.state.games.battleship.toggle_sonar();
            }
            UiAction::BattleshipHint => {
                self.state.card_hint = Some(crate::card_hints::battleship(&self.state));
                return true;
            }
            UiAction::BattleshipUndo => {
                self.state.games.battleship.undo();
            }
            _ => return false,
        }
        true
    }
}
