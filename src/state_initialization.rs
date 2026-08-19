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

    pub fn new_random(data: &GameData, seed: u64) -> Self {
        let mut state = Self::new(data);
        state.game = crate::game_2048::Game2048::new(seed_at(seed, 0));
        state.minesweeper = crate::minesweeper::Minesweeper::beginner(seed_at(seed, 1));
        state.sudoku =
            crate::sudoku::Sudoku::with_difficulty(crate::sudoku::SudokuDifficulty::Medium);
        state.nonogram = crate::nonogram::Nonogram::new_with_seed(
            crate::nonogram::NonogramPreset::Medium,
            seed_at(seed, 45),
        );
        state.solitaire = crate::solitaire::Solitaire::new(seed_at(seed, 2));
        state.freecell = crate::freecell::FreeCell::new(seed_at(seed, 3));
        state.fivefold = crate::fivefold::Fivefold::new(seed_at(seed, 4));
        state.reversi =
            crate::reversi::Reversi::new(seed_at(seed, 5), crate::reversi::AiLevel::Gentle);
        state.lights_out = crate::lights_out::LightsOut::new(seed_at(seed, 6));
        state.tic_tac_toe = crate::tic_tac_toe::TicTacToe::new(seed_at(seed, 7));
        state.memory_pairs = crate::memory_pairs::MemoryPairs::new(seed_at(seed, 8));
        state.sliding_puzzle = crate::sliding_puzzle::SlidingPuzzle::new(seed_at(seed, 9));
        state.mastermind = crate::mastermind::Mastermind::new(seed_at(seed, 10));
        state.spider = crate::spider::Spider::new(seed_at(seed, 11));
        state.word_search = crate::word_search::WordSearch::new(seed_at(seed, 12));
        state.hangman = crate::hangman::Hangman::new(seed_at(seed, 13));
        state.connect_four = crate::connect_four::ConnectFour::new(seed_at(seed, 14));
        state.checkers = crate::checkers::Checkers::new(seed_at(seed, 15));
        state.peg_solitaire = crate::peg_solitaire::PegSolitaire::new(seed_at(seed, 16));
        state.mahjong_solitaire =
            crate::mahjong_solitaire::MahjongSolitaire::new(seed_at(seed, 17));
        state.snake = crate::snake::Snake::new(seed_at(seed, 18));
        state.breakout = crate::breakout::Breakout::new(seed_at(seed, 19));
        state.higher_lower = crate::higher_lower::HigherLower::new(seed_at(seed, 20));
        state.klondike_golf = crate::klondike_golf::KlondikeGolf::new(seed_at(seed, 21));
        state.blackjack = crate::blackjack::Blackjack::new(seed_at(seed, 22));
        state.spider_solitaire = crate::spider_solitaire::SpiderSolitaire::new(seed_at(seed, 23));
        state.dungeon_sweeper = crate::dungeon_sweeper::DungeonSweeper::new(seed_at(seed, 24));
        state.potion_2048 = crate::potion_2048::Potion2048::new(seed_at(seed, 25));
        state.tiny_tower_defence =
            crate::tiny_tower_defence::TinyTowerDefence::new(seed_at(seed, 26));
        state.one_room_roguelike =
            crate::one_room_roguelike::OneRoomRoguelike::new(seed_at(seed, 27));
        state.daily_dungeon = crate::daily_dungeon::DailyDungeon::new(seed_at(seed, 28));
        state.dots_boxes = DotsBoxes::new_with_config(
            seed_at(seed, 29),
            DotsDifficulty::Standard,
            &data.puzzles.dots_boxes,
        );
        state.sokoban = crate::sokoban::Sokoban::new_with_seed(seed_at(seed, 30));
        state.mancala = crate::mancala::Mancala::new(seed_at(seed, 31));
        state.hanoi = crate::hanoi::Hanoi::new_with_seed(seed_at(seed, 32));
        state.number_match = crate::number_match::NumberMatch::new(seed_at(seed, 33));
        state.flood_it = FloodIt::new_with_config(
            seed_at(seed, 34),
            FloodDifficulty::Standard,
            &data.puzzles.flood_it,
        );
        state.color_sort = ColorSort::new_with_config(
            seed_at(seed, 35),
            ColorSortDifficulty::Standard,
            &data.puzzles.color_sort,
        );
        state.battleship = crate::battleship::Battleship::new(seed_at(seed, 36));
        state.word_grid = crate::word_grid::WordGrid::new(seed_at(seed, 37));
        state.pipe_loop = crate::pipe_loop::PipeLoop::new(seed_at(seed, 38));
        state.maze_walk = crate::maze_walk::MazeWalk::new(seed_at(seed, 39));
        state.match_three = MatchThree::new_with_config(
            seed_at(seed, 40),
            MatchThreeDifficulty::Standard,
            &data.puzzles.match_three,
        );
        state.pyramid = crate::pyramid::Pyramid::new(seed_at(seed, 41));
        state.tri_peaks = crate::tri_peaks::TriPeaks::new(seed_at(seed, 42));
        state.nim = crate::nim::Nim::new(seed_at(seed, 43));
        state.word_ladder = crate::word_ladder::WordLadder::new(seed_at(seed, 44));
        state
    }
}

fn seed_at(seed: u64, index: u64) -> u64 {
    let mut value = seed.wrapping_add(0x9E37_79B9_7F4A_7C15u64.wrapping_mul(index + 1));
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    let result = value ^ (value >> 31);
    if result == 0 {
        1
    } else {
        result
    }
}
