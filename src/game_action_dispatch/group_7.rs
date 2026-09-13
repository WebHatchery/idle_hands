use super::*;

impl Game {
    pub(super) fn apply_shell_group_7(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::LightsOutPress(index) => {
                self.state.games.lights_out.press(index);
            }
            ui::UiAction::LightsOutHint => {
                self.state.card_hint = Some(card_hints::lights_out(&self.state));
            }
            ui::UiAction::LightsOutUndo => {
                self.state.games.lights_out.undo();
            }
            ui::UiAction::LightsOutNew => {
                let seed = self.state.games.lights_out.seed.wrapping_add(1);
                self.state.games.lights_out.reset(seed);
            }
            ui::UiAction::TicTacToePress(index) => {
                self.state.games.tic_tac_toe.place(index);
            }
            ui::UiAction::TicTacToeHint => {
                self.state.card_hint = Some(card_hints::tic_tac_toe(&self.state));
            }
            ui::UiAction::TicTacToeUndo => {
                self.state.games.tic_tac_toe.undo();
            }
            ui::UiAction::TicTacToeNew => {
                let seed = self.state.games.tic_tac_toe.seed.wrapping_add(1);
                self.state.games.tic_tac_toe.reset(seed);
            }
            ui::UiAction::TicTacToeLevel(level) => {
                self.state.games.tic_tac_toe.set_ai_level(level);
            }
            ui::UiAction::MemoryPairsSelect(index) => {
                self.state.games.memory_pairs.select(index);
            }
            ui::UiAction::MemoryPairsHint => {
                self.state.card_hint = Some(card_hints::memory_pairs(&self.state));
            }
            ui::UiAction::MemoryPairsUndo => {
                self.state.games.memory_pairs.undo();
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
