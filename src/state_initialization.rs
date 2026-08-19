use crate::color_sort::{ColorSort, ColorSortDifficulty};
use crate::data::GameData;
use crate::dots_boxes::{DotsBoxes, DotsDifficulty};
use crate::flood_it::{FloodDifficulty, FloodIt};
use crate::match_three::{MatchThree, MatchThreeDifficulty};
use crate::state::AppState;

impl AppState {
    pub fn new(data: &GameData) -> Self {
        let mut state = Self::default();
        state.dots_boxes = DotsBoxes::new_with_config(
            state.dots_boxes.seed,
            DotsDifficulty::Standard,
            &data.puzzles.dots_boxes,
        );
        state.flood_it = FloodIt::new_with_config(
            state.flood_it.seed,
            FloodDifficulty::Standard,
            &data.puzzles.flood_it,
        );
        state.color_sort = ColorSort::new_with_config(
            state.color_sort.seed,
            ColorSortDifficulty::Standard,
            &data.puzzles.color_sort,
        );
        state.match_three = MatchThree::new_with_config(
            state.match_three.seed,
            MatchThreeDifficulty::Standard,
            &data.puzzles.match_three,
        );
        state
    }
}
