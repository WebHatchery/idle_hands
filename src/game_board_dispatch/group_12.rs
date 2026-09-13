use super::super::Game;
use crate::ui::UiAction;

impl Game {
    pub(super) fn apply_game_action_group_12(&mut self, action: &UiAction) -> bool {
        match action {
            UiAction::WordLadderMode(mode) => {
                let seed = self.state.games.word_ladder.seed.wrapping_add(1);
                self.state.games.word_ladder.set_mode(*mode, seed);
            }
            UiAction::PipeRotate(index) => {
                self.state.games.pipe_loop.rotate(*index);
            }
            UiAction::PipeHint => {
                self.state.card_hint = Some(crate::card_hints::pipe_loop(&self.state));
                return true;
            }
            UiAction::PipeUndo => {
                self.state.games.pipe_loop.undo();
            }
            UiAction::PipeNew => {
                let seed = self.state.games.pipe_loop.seed.wrapping_add(1);
                self.state.games.pipe_loop.reset(seed);
            }
            UiAction::PipePattern(pattern) => {
                let seed = self.state.games.pipe_loop.seed.wrapping_add(1);
                self.state.games.pipe_loop.set_pattern(*pattern, seed);
            }
            UiAction::MazeStep(direction) => {
                self.state.games.maze_walk.step(*direction);
            }
            UiAction::MazeHint => {
                self.state.card_hint = Some(crate::card_hints::maze_walk(&self.state));
                return true;
            }
            UiAction::MazeUndo => {
                self.state.games.maze_walk.undo();
            }
            UiAction::MazeNew => {
                let seed = self.state.games.maze_walk.seed.wrapping_add(1);
                self.state.games.maze_walk.reset(seed);
            }
            UiAction::MazeMode(mode) => {
                let seed = self.state.games.maze_walk.seed.wrapping_add(1);
                self.state.games.maze_walk.set_mode(*mode, seed);
            }
            UiAction::MatchThreeTap(index) => {
                self.state.games.match_three.tap(*index);
            }
            UiAction::MatchThreeHint => {
                self.state.card_hint = Some(crate::card_hints::match_three(&self.state));
                return true;
            }
            UiAction::MatchThreeUndo => {
                self.state.games.match_three.undo();
            }
            _ => return false,
        }
        true
    }
}
