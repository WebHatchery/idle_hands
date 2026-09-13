use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_4(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::BlackjackHint => {
                self.state.card_hint = Some(crate::card_hints::blackjack(&self.state));
                return true;
            }
            UiAction::BlackjackUndo => {
                self.state.games.blackjack.undo();
            }
            UiAction::BlackjackNew => {
                let seed = self.state.games.blackjack.seed.wrapping_add(1);
                self.state.games.blackjack.reset(seed);
            }
            UiAction::SpiderSolitaireSelect(column, depth) => {
                self.state
                    .games
                    .spider_solitaire
                    .tap_column(*column, *depth);
            }
            UiAction::SpiderSolitaireDeal => {
                self.state.games.spider_solitaire.deal_stock();
            }
            UiAction::SpiderSolitaireHint => {
                self.state.card_hint = Some(crate::card_hints::spider_solitaire(&self.state));
            }
            UiAction::SpiderSolitaireUndo => {
                self.state.games.spider_solitaire.undo();
            }
            UiAction::SpiderSolitaireNew => {
                let seed = self.state.games.spider_solitaire.seed.wrapping_add(1);
                self.state.games.spider_solitaire.reset(seed);
            }
            UiAction::PyramidTap(index) => {
                self.state.games.pyramid.tap(*index);
            }
            UiAction::PyramidStock => {
                self.state.games.pyramid.draw_stock();
            }
            UiAction::PyramidHint => {
                self.state.card_hint = Some(crate::card_hints::pyramid(&self.state));
            }
            UiAction::PyramidUndo => {
                self.state.games.pyramid.undo();
            }
            UiAction::PyramidNew => {
                let seed = self.state.games.pyramid.seed.wrapping_add(1);
                self.state.games.pyramid.reset(seed);
            }
            UiAction::PyramidDrawRule(rule) => {
                let seed = self.state.games.pyramid.seed.wrapping_add(1);
                self.state.games.pyramid.set_draw_rule(*rule, seed);
            }
            _ => return false,
        }
        true
    }
}
