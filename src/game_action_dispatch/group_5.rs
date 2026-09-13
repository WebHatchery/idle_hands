use super::*;

impl Game {
    pub(super) fn apply_shell_group_5(&mut self, action: ui::UiAction) -> ShellActionResult {
        match action {
            ui::UiAction::SolitaireStock => {
                self.state.games.solitaire.draw_stock();
            }
            ui::UiAction::SolitaireTableau(column, depth) => {
                let had_selection = self.state.games.solitaire.selected.is_some();
                if !self.state.games.solitaire.tap_tableau(column, depth) && had_selection {
                    self.notifications
                        .warning("That tableau does not accept this card");
                }
            }
            ui::UiAction::SolitaireWaste => {
                self.state.games.solitaire.tap_waste();
            }
            ui::UiAction::SolitaireFoundation(suit) => {
                if !self.state.games.solitaire.move_to_foundation(suit) {
                    self.notifications
                        .warning("That card cannot go to this foundation yet");
                }
            }
            ui::UiAction::SolitaireUndo => {
                self.state.games.solitaire.undo();
            }
            ui::UiAction::SolitaireHint => {
                self.state.card_hint = Some(card_hints::solitaire(&self.state));
            }
            ui::UiAction::SolitaireNew => {
                self.state.games.solitaire = crate::solitaire::Solitaire::new(
                    self.state.games.solitaire.seed.wrapping_add(1),
                );
            }
            ui::UiAction::FreeCellCell(cell) => {
                self.state.games.freecell.tap_cell(cell);
            }
            ui::UiAction::FreeCellCascade(cascade, depth) => {
                let had_selection = self.state.games.freecell.selected.is_some();
                if !self.state.games.freecell.tap_cascade(cascade, depth) && had_selection {
                    self.notifications
                        .warning("That stack cannot move to this cascade");
                }
            }
            ui::UiAction::FreeCellFoundation(suit) => {
                if !self.state.games.freecell.move_selected_to_foundation(suit) {
                    self.notifications
                        .warning("That card cannot go to this foundation yet");
                }
            }
            ui::UiAction::FreeCellUndo => {
                self.state.games.freecell.undo();
            }
            ui::UiAction::FreeCellHint => {
                self.state.card_hint = Some(card_hints::freecell(&self.state));
            }
            _ => return ShellActionResult::Unhandled,
        }
        ShellActionResult::Continue
    }
}
