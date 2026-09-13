use super::*;

impl Game {
    pub(super) fn apply_shell_group_8(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::MemoryPairsNew => {
                let seed = self.state.games.memory_pairs.seed.wrapping_add(1);
                self.state.games.memory_pairs.reset(seed);
            }
            ui::UiAction::SlidingPuzzleMove(index) => {
                self.state.games.sliding_puzzle.move_tile(index);
            }
            ui::UiAction::SlidingPuzzleHint => {
                self.state.card_hint = Some(card_hints::sliding_puzzle(&self.state));
            }
            ui::UiAction::SlidingPuzzleUndo => {
                self.state.games.sliding_puzzle.undo();
            }
            ui::UiAction::SlidingPuzzleNew => {
                let seed = self.state.games.sliding_puzzle.seed.wrapping_add(1);
                self.state.games.sliding_puzzle.reset(seed);
            }
            ui::UiAction::MastermindPick(color) => {
                self.state.games.mastermind.pick(color);
            }
            ui::UiAction::MastermindHint => {
                self.state.card_hint = Some(card_hints::mastermind(&self.state));
            }
            ui::UiAction::MastermindSubmit => {
                self.state.games.mastermind.submit();
            }
            ui::UiAction::MastermindClear => {
                self.state.games.mastermind.clear();
            }
            ui::UiAction::MastermindUndo => {
                self.state.games.mastermind.undo();
            }
            ui::UiAction::MastermindNew => {
                let seed = self.state.games.mastermind.seed.wrapping_add(1);
                self.state.games.mastermind.reset(seed);
            }
            ui::UiAction::SpiderSelect(column, depth) => {
                self.state.games.spider.tap_column(column, depth);
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
