use super::*;

impl Game {
    pub(super) fn apply_shell_group_9(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::SpiderDeal => {
                self.state.games.spider.deal_stock();
            }
            ui::UiAction::SpiderHint => {
                self.state.card_hint = Some(card_hints::spider(&self.state));
            }
            ui::UiAction::SpiderUndo => {
                self.state.games.spider.undo();
            }
            ui::UiAction::SpiderNew => {
                let seed = self.state.games.spider.seed.wrapping_add(1);
                self.state.games.spider.reset(seed);
            }
            ui::UiAction::WordSearchCell(index) => {
                self.state.games.word_search.select(index);
            }
            ui::UiAction::WordSearchClear => {
                self.state.games.word_search.clear();
            }
            ui::UiAction::WordSearchHint => {
                self.state.card_hint = Some(card_hints::word_search(&self.state));
            }
            ui::UiAction::WordSearchNew => {
                let seed = self.state.games.word_search.seed.wrapping_add(1);
                self.state.games.word_search.reset(seed);
            }
            ui::UiAction::MineChord(index) => {
                self.state.games.minesweeper.chord(index);
            }
            ui::UiAction::MineRestart => {
                self.state.games.minesweeper = crate::minesweeper::Minesweeper::beginner(
                    self.state.games.minesweeper.seed.wrapping_add(1),
                );
            }
            ui::UiAction::Undo => {
                if self.state.games.game.undo() {
                    self.notifications.info("One move undone");
                }
            }
            ui::UiAction::Restart => self.state.confirm_restart = true,
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
