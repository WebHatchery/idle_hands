use super::*;

impl Game {
    pub(super) fn apply_shell_group_3(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::Save => self.flush_autosave(),
            ui::UiAction::Load => {
                self.load_autosave();
                self.analytics.sync_progress(&self.state);
            }
            ui::UiAction::Game2048Hint => {
                self.state.card_hint = Some(card_hints::game_2048(&self.state));
            }
            ui::UiAction::Game2048Size(board_size) => {
                self.state.games.game = crate::state::Game2048::new_with_size(
                    self.state.games.game.seed.wrapping_add(1),
                    board_size,
                );
            }
            ui::UiAction::Move(direction) => self.try_move(direction),
            ui::UiAction::MineReveal(index) => {
                self.state.games.minesweeper.reveal(index);
            }
            ui::UiAction::MineFlag(index) => {
                self.state.games.minesweeper.toggle_flag(index);
            }
            ui::UiAction::MineFlagMode => {
                self.state.mine_flag_mode = !self.state.mine_flag_mode;
            }
            ui::UiAction::MineHint => {
                self.state.card_hint = Some(card_hints::minesweeper(&self.state));
            }
            ui::UiAction::MinePreset(preset) => {
                let seed = self.state.games.minesweeper.seed.wrapping_add(1);
                self.state.games.minesweeper = if preset == crate::minesweeper::MinePreset::Custom {
                    crate::minesweeper::Minesweeper::custom(12, 12, 20, seed)
                } else {
                    crate::minesweeper::Minesweeper::new(preset, seed)
                };
                self.state.mine_flag_mode = false;
            }
            ui::UiAction::SudokuCell(index) => {
                self.state.games.sudoku.select(index);
            }
            ui::UiAction::SudokuNumber(value) => {
                if let Some(index) = self.state.games.sudoku.selected {
                    if self.state.sudoku_note_mode {
                        self.state.games.sudoku.toggle_note(index, value);
                    } else {
                        self.state.games.sudoku.place(index, value);
                    }
                }
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
