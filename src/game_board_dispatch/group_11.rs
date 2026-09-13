use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_11(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::BattleshipNew => {
                let seed = self.state.games.battleship.seed.wrapping_add(1);
                self.state.games.battleship.reset(seed);
            }
            UiAction::WordGridLetter(letter) => {
                self.state.games.word_grid.tap_letter(*letter);
            }
            UiAction::WordGridBackspace => {
                self.state.games.word_grid.backspace();
            }
            UiAction::WordGridSubmit => {
                self.state.games.word_grid.submit();
            }
            UiAction::WordGridHint => {
                self.state.card_hint = Some(crate::card_hints::word_grid(&self.state));
                return true;
            }
            UiAction::WordGridUndo => {
                self.state.games.word_grid.undo();
            }
            UiAction::WordGridNew => {
                let seed = self.state.games.word_grid.seed.wrapping_add(1);
                self.state.games.word_grid.reset(seed);
            }
            UiAction::WordGridMode(mode) => {
                let seed = self.state.games.word_grid.seed.wrapping_add(1);
                self.state.games.word_grid.set_mode(*mode, seed);
            }
            UiAction::WordLadderLetter(letter) => {
                self.state.games.word_ladder.tap_letter(*letter);
            }
            UiAction::WordLadderBackspace => {
                self.state.games.word_ladder.backspace();
            }
            UiAction::WordLadderSubmit => {
                self.state.games.word_ladder.submit();
            }
            UiAction::WordLadderHint => {
                self.state.card_hint = Some(crate::card_hints::word_ladder(&self.state));
                return true;
            }
            UiAction::WordLadderUndo => {
                self.state.games.word_ladder.undo();
            }
            UiAction::WordLadderNew => {
                let seed = self.state.games.word_ladder.seed.wrapping_add(1);
                self.state.games.word_ladder.reset(seed);
            }
            _ => return false,
        }
        true
    }
}
