use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_3(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::MahjongSolitaireNew => {
                let seed = self.state.games.mahjong_solitaire.seed.wrapping_add(1);
                self.state.games.mahjong_solitaire.reset(seed);
            }
            UiAction::HigherLowerGuess(guess) => {
                self.state.games.higher_lower.guess(*guess);
            }
            UiAction::HigherLowerHint => {
                self.state.card_hint = Some(crate::card_hints::higher_lower(&self.state));
                return true;
            }
            UiAction::HigherLowerUndo => {
                self.state.games.higher_lower.undo();
            }
            UiAction::HigherLowerNew => {
                let seed = self.state.games.higher_lower.seed.wrapping_add(1);
                self.state.games.higher_lower.reset(seed);
            }
            UiAction::HigherLowerCashOut => {
                self.state.games.higher_lower.cash_out();
            }
            UiAction::HigherLowerRule(rule) => {
                let seed = self.state.games.higher_lower.seed.wrapping_add(1);
                self.state.games.higher_lower.set_rule(*rule, seed);
            }
            UiAction::KlondikeGolfColumn(column) => {
                self.state.games.klondike_golf.tap_column(*column);
            }
            UiAction::KlondikeGolfStock => {
                self.state.games.klondike_golf.draw_stock();
            }
            UiAction::KlondikeGolfHint => {
                self.state.card_hint = Some(crate::card_hints::klondike_golf(&self.state));
            }
            UiAction::KlondikeGolfUndo => {
                self.state.games.klondike_golf.undo();
            }
            UiAction::KlondikeGolfNew => {
                let seed = self.state.games.klondike_golf.seed.wrapping_add(1);
                self.state.games.klondike_golf.reset(seed);
            }
            UiAction::BlackjackHit => {
                self.state.games.blackjack.hit();
            }
            UiAction::BlackjackStand => {
                self.state.games.blackjack.stand();
            }
            _ => return false,
        }
        true
    }
}
