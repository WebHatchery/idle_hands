use crate::color_sort::{ColorSort, ColorSortDifficulty};
use crate::data::GameData;
use crate::dots_boxes::{DotsBoxes, DotsDifficulty};
use crate::flood_it::{FloodDifficulty, FloodIt};
use crate::match_three::{MatchThree, MatchThreeDifficulty};
use crate::state::AppState;

impl AppState {
    pub fn new(data: &GameData) -> Self {
        let mut state = Self::default();
        state.games.dots_boxes = DotsBoxes::new_with_config(
            state.games.dots_boxes.seed,
            DotsDifficulty::Standard,
            &data.puzzles.dots_boxes,
        );
        state.games.flood_it = FloodIt::new_with_config(
            state.games.flood_it.seed,
            FloodDifficulty::Standard,
            &data.puzzles.flood_it,
        );
        state.games.color_sort = ColorSort::new_with_config(
            state.games.color_sort.seed,
            ColorSortDifficulty::Standard,
            &data.puzzles.color_sort,
        );
        state.games.match_three = MatchThree::new_with_config(
            state.games.match_three.seed,
            MatchThreeDifficulty::Standard,
            &data.puzzles.match_three,
        );
        state
    }

    pub fn new_random(data: &GameData, seed: u64) -> Self {
        let mut state = Self::new(data);
        state.games.game = crate::game_2048::Game2048::new(seed_at(seed, 0));
        state.games.minesweeper = crate::minesweeper::Minesweeper::beginner(seed_at(seed, 1));
        state.games.sudoku =
            crate::sudoku::Sudoku::with_difficulty(crate::sudoku::SudokuDifficulty::Medium);
        state.games.nonogram = crate::nonogram::Nonogram::new_with_seed(
            crate::nonogram::NonogramPreset::Medium,
            seed_at(seed, 45),
        );
        state.games.solitaire = crate::solitaire::Solitaire::new(seed_at(seed, 2));
        state.games.freecell = crate::freecell::FreeCell::new(seed_at(seed, 3));
        state.games.fivefold = crate::fivefold::Fivefold::new(seed_at(seed, 4));
        state.games.reversi =
            crate::reversi::Reversi::new(seed_at(seed, 5), crate::reversi::AiLevel::Gentle);
        state.games.lights_out = crate::lights_out::LightsOut::new(seed_at(seed, 6));
        state.games.tic_tac_toe = crate::tic_tac_toe::TicTacToe::new(seed_at(seed, 7));
        state.games.memory_pairs = crate::memory_pairs::MemoryPairs::new(seed_at(seed, 8));
        state.games.sliding_puzzle = crate::sliding_puzzle::SlidingPuzzle::new(seed_at(seed, 9));
        state.games.mastermind = crate::mastermind::Mastermind::new(seed_at(seed, 10));
        state.games.spider = crate::spider::Spider::new(seed_at(seed, 11));
        state.games.word_search = crate::word_search::WordSearch::new(seed_at(seed, 12));
        state.games.hangman = crate::hangman::Hangman::new(seed_at(seed, 13));
        state.games.connect_four = crate::connect_four::ConnectFour::new(seed_at(seed, 14));
        state.games.checkers = crate::checkers::Checkers::new(seed_at(seed, 15));
        state.games.peg_solitaire = crate::peg_solitaire::PegSolitaire::new(seed_at(seed, 16));
        state.games.mahjong_solitaire =
            crate::mahjong_solitaire::MahjongSolitaire::new(seed_at(seed, 17));
        state.games.snake = crate::snake::Snake::new(seed_at(seed, 18));
        state.games.breakout = crate::breakout::Breakout::new(seed_at(seed, 19));
        state.games.higher_lower = crate::higher_lower::HigherLower::new(seed_at(seed, 20));
        state.games.klondike_golf = crate::klondike_golf::KlondikeGolf::new(seed_at(seed, 21));
        state.games.blackjack = crate::blackjack::Blackjack::new(seed_at(seed, 22));
        state.games.spider_solitaire =
            crate::spider_solitaire::SpiderSolitaire::new(seed_at(seed, 23));
        state.games.dungeon_sweeper =
            crate::dungeon_sweeper::DungeonSweeper::new(seed_at(seed, 24));
        state.games.potion_2048 = crate::potion_2048::Potion2048::new(seed_at(seed, 25));
        state.games.tiny_tower_defence =
            crate::tiny_tower_defence::TinyTowerDefence::new(seed_at(seed, 26));
        state.games.one_room_roguelike =
            crate::one_room_roguelike::OneRoomRoguelike::new(seed_at(seed, 27));
        state.games.daily_dungeon = crate::daily_dungeon::DailyDungeon::new(seed_at(seed, 28));
        state.games.dots_boxes = DotsBoxes::new_with_config(
            seed_at(seed, 29),
            DotsDifficulty::Standard,
            &data.puzzles.dots_boxes,
        );
        state.games.sokoban = crate::sokoban::Sokoban::new_with_seed(seed_at(seed, 30));
        state.games.mancala = crate::mancala::Mancala::new(seed_at(seed, 31));
        state.games.hanoi = crate::hanoi::Hanoi::new_with_seed(seed_at(seed, 32));
        state.games.number_match = crate::number_match::NumberMatch::new(seed_at(seed, 33));
        state.games.flood_it = FloodIt::new_with_config(
            seed_at(seed, 34),
            FloodDifficulty::Standard,
            &data.puzzles.flood_it,
        );
        state.games.color_sort = ColorSort::new_with_config(
            seed_at(seed, 35),
            ColorSortDifficulty::Standard,
            &data.puzzles.color_sort,
        );
        state.games.battleship = crate::battleship::Battleship::new(seed_at(seed, 36));
        state.games.word_grid = crate::word_grid::WordGrid::new(seed_at(seed, 37));
        state.games.pipe_loop = crate::pipe_loop::PipeLoop::new(seed_at(seed, 38));
        state.games.maze_walk = crate::maze_walk::MazeWalk::new(seed_at(seed, 39));
        state.games.match_three = MatchThree::new_with_config(
            seed_at(seed, 40),
            MatchThreeDifficulty::Standard,
            &data.puzzles.match_three,
        );
        state.games.pyramid = crate::pyramid::Pyramid::new(seed_at(seed, 41));
        state.games.tri_peaks = crate::tri_peaks::TriPeaks::new(seed_at(seed, 42));
        state.games.nim = crate::nim::Nim::new(seed_at(seed, 43));
        state.games.word_ladder = crate::word_ladder::WordLadder::new(seed_at(seed, 44));
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
