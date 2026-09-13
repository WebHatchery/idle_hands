use super::*;

impl Game {
    pub(super) fn apply_shell_group_6(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::FreeCellNew => {
                self.state.games.freecell =
                    crate::freecell::FreeCell::new(self.state.games.freecell.seed.wrapping_add(1));
            }
            ui::UiAction::FivefoldRoll => {
                self.state.games.fivefold.roll();
            }
            ui::UiAction::FivefoldHold(index) => {
                self.state.games.fivefold.toggle_hold(index);
            }
            ui::UiAction::FivefoldCategory(category) => {
                self.state.games.fivefold.choose_category(category);
            }
            ui::UiAction::FivefoldScorePage(delta) => {
                self.state.fivefold_score_page = self
                    .state
                    .fivefold_score_page
                    .saturating_add_signed(delta as isize)
                    .min(2);
            }
            ui::UiAction::FivefoldHint => {
                self.state.card_hint = Some(card_hints::fivefold(&self.state));
            }
            ui::UiAction::FivefoldNew => {
                self.state.games.fivefold =
                    crate::fivefold::Fivefold::new(self.state.games.fivefold.seed.wrapping_add(1));
                self.state.fivefold_score_page = 0;
            }
            ui::UiAction::ReversiPlace(index) => {
                if self.state.games.reversi.ai_level == crate::reversi::AiLevel::TwoPlayer {
                    self.state.games.reversi.place_current(index);
                } else if self.state.games.reversi.place(index) {
                    self.state.games.reversi.ai_move();
                }
            }
            ui::UiAction::ReversiPass => {
                if self.state.games.reversi.pass()
                    && self.state.games.reversi.ai_level != crate::reversi::AiLevel::TwoPlayer
                {
                    self.state.games.reversi.ai_move();
                }
            }
            ui::UiAction::ReversiHint => {
                self.state.card_hint = Some(card_hints::reversi(&self.state));
            }
            ui::UiAction::ReversiNew => {
                self.state.games.reversi = crate::reversi::Reversi::new(
                    self.state.games.reversi.seed.wrapping_add(1),
                    self.state.games.reversi.ai_level,
                );
            }
            ui::UiAction::ReversiLevel(level) => {
                self.state.games.reversi.ai_level = level;
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
