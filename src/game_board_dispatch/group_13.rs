use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_13(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::MatchThreeNew => {
                let seed = self.state.games.match_three.seed.wrapping_add(1);
                self.state.games.match_three.reset(seed);
            }
            UiAction::MatchThreeDifficulty(difficulty) => {
                let seed = self.state.games.match_three.seed.wrapping_add(1);
                self.state.games.match_three = crate::match_three::MatchThree::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.match_three,
                );
            }
            UiAction::MiscTap(index) => {
                if let Some(game) = self.active_misc_game_mut() {
                    game.tap(*index);
                }
            }
            UiAction::MiscSubmit => {
                if let Some(game) = self.active_misc_game_mut() {
                    game.submit();
                }
            }
            UiAction::MiscClear => {
                if let Some(game) = self.active_misc_game_mut() {
                    game.clear_selection();
                }
            }
            UiAction::MiscHint => {
                if let Some(game) = self.active_misc_game_mut() {
                    self.state.card_hint = Some(game.hint());
                }
                return true;
            }
            UiAction::MiscUndo => {
                if let Some(game) = self.active_misc_game_mut() {
                    game.undo();
                }
            }
            UiAction::MiscNew => {
                if let Some(game) = self.active_misc_game_mut() {
                    let seed = game.seed.wrapping_add(1);
                    game.reset(seed);
                }
            }
            _ => return false,
        }
        true
    }
}
