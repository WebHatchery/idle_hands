use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_1(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::CycleGameVariant => {
                if let crate::state::Screen::Game(game) = self.state.screen {
                    crate::game_variants::cycle(&mut self.state, &self.data, game);
                }
            }
            UiAction::HangmanGuess(letter) => {
                self.state.games.hangman.guess(*letter);
            }
            UiAction::HangmanHint => {
                self.state.card_hint = Some(crate::card_hints::hangman(&self.state));
            }
            UiAction::HangmanNew => {
                let seed = self.state.games.hangman.seed.wrapping_add(1);
                self.state.games.hangman.reset(seed);
            }
            UiAction::HangmanReveal => {
                self.state.games.hangman.reveal();
            }
            UiAction::HangmanUndo => {
                self.state.games.hangman.undo();
            }
            UiAction::HangmanCategory(category) => {
                let seed = self.state.games.hangman.seed.wrapping_add(1);
                self.state
                    .games
                    .hangman
                    .set_category_with_config_and_balance(
                        *category,
                        seed,
                        &self.data.content.words.hangman,
                        &self.data.content.balance.word_games,
                    );
            }
            UiAction::HangmanRule(rule) => {
                let seed = self.state.games.hangman.seed.wrapping_add(1);
                self.state.games.hangman.set_rule_with_balance(
                    *rule,
                    seed,
                    &self.data.content.balance.word_games,
                );
            }
            UiAction::LightsOutGuide => {
                self.state.games.lights_out.toggle_guide();
            }
            UiAction::LightsOutDifficulty(difficulty) => {
                let seed = self.state.games.lights_out.seed.wrapping_add(1);
                self.state
                    .games
                    .lights_out
                    .set_difficulty(*difficulty, seed);
            }
            UiAction::MemoryPairsPeek => {
                self.state.games.memory_pairs.peek();
            }
            UiAction::ConnectFourDrop(column) => self.apply_connect_four_drop(*column),
            UiAction::ConnectFourHint => {
                self.state.card_hint = Some(crate::card_hints::connect_four(&self.state));
                return true;
            }
            UiAction::ConnectFourUndo => self.apply_connect_four_undo(),
            _ => return false,
        }
        true
    }
}
