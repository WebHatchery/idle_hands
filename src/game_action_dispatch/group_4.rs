use super::*;

impl Game {
    pub(super) fn apply_shell_group_4(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::SudokuFocus => {
                if self.state.games.sudoku.selected.is_none() {
                    self.state.games.sudoku.selected = Some(40);
                }
                self.state.sudoku_focus_open = !self.state.sudoku_focus_open;
            }
            ui::UiAction::SudokuErase => {
                if let Some(index) = self.state.games.sudoku.selected {
                    self.state.games.sudoku.erase(index);
                }
            }
            ui::UiAction::SudokuNoteMode => {
                self.state.sudoku_note_mode = !self.state.sudoku_note_mode;
            }
            ui::UiAction::SudokuDifficulty(difficulty) => {
                self.state.games.sudoku = crate::sudoku::Sudoku::with_difficulty(difficulty);
                self.state.sudoku_note_mode = false;
            }
            ui::UiAction::SudokuUndo => {
                self.state.games.sudoku.undo();
            }
            ui::UiAction::SudokuHint => {
                self.state.card_hint = Some(card_hints::sudoku(&self.state));
            }
            ui::UiAction::NonogramCell(index) => {
                self.state.games.nonogram.select(index);
                self.state.games.nonogram.toggle(index);
            }
            ui::UiAction::NonogramMode => {
                self.state.games.nonogram.toggle_mode();
            }
            ui::UiAction::NonogramHint => {
                self.state.card_hint = Some(card_hints::nonogram(&self.state));
            }
            ui::UiAction::NonogramFocusMove(delta_row, delta_col) => {
                let size = self.state.games.nonogram.size;
                let selected = self
                    .state
                    .games
                    .nonogram
                    .selected
                    .unwrap_or((size / 2) * size + size / 2);
                let row = (selected / size) as isize;
                let col = (selected % size) as isize;
                let next_row = (row + delta_row as isize).clamp(0, size as isize - 1) as usize;
                let next_col = (col + delta_col as isize).clamp(0, size as isize - 1) as usize;
                self.state.games.nonogram.selected = Some(next_row * size + next_col);
            }
            ui::UiAction::NonogramFocus => {
                if self.state.games.nonogram.selected.is_none() {
                    let center = self.state.games.nonogram.size / 2;
                    self.state.games.nonogram.selected =
                        Some(center * self.state.games.nonogram.size + center);
                }
                self.state.nonogram_focus_open = !self.state.nonogram_focus_open;
            }
            ui::UiAction::NonogramUndo => {
                self.state.games.nonogram.undo();
            }
            ui::UiAction::NonogramPreset(preset) => {
                let variant = self.state.games.nonogram.variant.wrapping_add(1);
                self.state.games.nonogram =
                    crate::nonogram::Nonogram::new_with_variant(preset, variant);
                self.state.games.nonogram_zoomed = false;
                self.state.games.nonogram_focus = (0, 0);
            }
            ui::UiAction::NonogramZoom => {
                self.state.games.nonogram_zoomed = !self.state.games.nonogram_zoomed;
                self.state.games.nonogram_focus = crate::nonogram::focus_origin(
                    self.state.games.nonogram.size,
                    self.state.games.nonogram_zoomed,
                    self.state.games.nonogram_focus,
                );
            }
            ui::UiAction::NonogramPan(dx, dy) => {
                let (x, y) = self.state.games.nonogram_focus;
                let next = (
                    x.saturating_add_signed(dx as isize),
                    y.saturating_add_signed(dy as isize),
                );
                self.state.games.nonogram_focus = crate::nonogram::focus_origin(
                    self.state.games.nonogram.size,
                    self.state.games.nonogram_zoomed,
                    next,
                );
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
