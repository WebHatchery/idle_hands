use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_8(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::DotsEdge(edge) => {
                self.state.games.dots_boxes.play(*edge);
            }
            UiAction::DotsHint => {
                self.state.card_hint = Some(crate::card_hints::dots_boxes(&self.state));
                return true;
            }
            UiAction::DotsUndo => {
                self.state.games.dots_boxes.undo();
            }
            UiAction::DotsNew => {
                let seed = self.state.games.dots_boxes.seed.wrapping_add(1);
                self.state.games.dots_boxes.reset(seed);
            }
            UiAction::DotsDifficulty(difficulty) => {
                let seed = self.state.games.dots_boxes.seed.wrapping_add(1);
                self.state.games.dots_boxes = crate::dots_boxes::DotsBoxes::new_with_config(
                    seed,
                    *difficulty,
                    &self.data.puzzles.dots_boxes,
                );
            }
            UiAction::SokobanMove(direction) => {
                self.state.games.sokoban.move_in(*direction);
            }
            UiAction::SokobanHint => {
                self.state.card_hint = Some(crate::card_hints::sokoban(&self.state));
                return true;
            }
            UiAction::SokobanUndo => {
                self.state.games.sokoban.undo();
            }
            UiAction::SokobanRestart => {
                let seed = self.state.games.sokoban.seed;
                self.state.games.sokoban.reset(seed);
            }
            UiAction::SokobanNew => {
                let seed = self.state.games.sokoban.seed.wrapping_add(1);
                self.state.games.sokoban.reset_next(seed);
            }
            UiAction::MancalaPit(pit) => {
                self.state.games.mancala.play(*pit);
            }
            UiAction::MancalaHint => {
                self.state.card_hint = Some(crate::card_hints::mancala(&self.state));
                return true;
            }
            UiAction::MancalaUndo => {
                self.state.games.mancala.undo();
            }
            UiAction::MancalaNew => {
                let seed = self.state.games.mancala.seed.wrapping_add(1);
                self.state.games.mancala.reset(seed);
            }
            _ => return false,
        }
        true
    }
}
