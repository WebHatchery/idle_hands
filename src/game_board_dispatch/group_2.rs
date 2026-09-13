use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_2(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::ConnectFourNew => self.apply_connect_four_new(),
            UiAction::ConnectFourLevel(level) => self.state.games.connect_four.set_ai_level(*level),
            UiAction::CheckersTap(square) => self.apply_checkers_tap(*square),
            UiAction::CheckersHint => {
                self.state.card_hint = Some(crate::card_hints::checkers(&self.state));
                return true;
            }
            UiAction::CheckersUndo => self.apply_checkers_undo(),
            UiAction::CheckersNew => self.apply_checkers_new(),
            UiAction::CheckersLevel(level) => self.state.games.checkers.set_ai_level(*level),
            UiAction::PegSolitaireTap(square) => {
                self.state.games.peg_solitaire.tap(*square);
            }
            UiAction::PegSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::peg_solitaire(&self.state));
                return true;
            }
            UiAction::PegSolitaireUndo => {
                self.state.games.peg_solitaire.undo();
            }
            UiAction::PegSolitaireNew => {
                let seed = self.state.games.peg_solitaire.seed.wrapping_add(1);
                self.state.games.peg_solitaire.reset(seed);
            }
            UiAction::MahjongSolitaireTap(index) => {
                self.state.games.mahjong_solitaire.tap(*index);
            }
            UiAction::MahjongSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::mahjong_solitaire(&self.state));
                return true;
            }
            UiAction::MahjongSolitaireUndo => {
                self.state.games.mahjong_solitaire.undo();
            }
            _ => return false,
        }
        true
    }
}
