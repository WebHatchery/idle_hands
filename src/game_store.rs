//! Runtime-owned rule state for the cabinet's games.
//!
//! `AppState` keeps the shell and profile around this store. `Deref` is a
//! compatibility bridge for the existing game UI and reducers; new code should
//! use `state.games` when it needs to make ownership explicit.

use crate::asteroids::Asteroids;
use crate::battleship::Battleship;
use crate::blackjack::Blackjack;
use crate::block_stack::BlockStack;
use crate::breakout::Breakout;
use crate::checkers::Checkers;
use crate::color_sort::ColorSort;
use crate::connect_four::ConnectFour;
use crate::daily_dungeon::DailyDungeon;
use crate::dots_boxes::DotsBoxes;
use crate::dungeon_sweeper::DungeonSweeper;
use crate::fivefold::Fivefold;
use crate::fling_fury::FlingFury;
use crate::flood_it::FloodIt;
use crate::freecell::FreeCell;
use crate::frogger::Frogger;
use crate::game_2048::Game2048;
use crate::hangman::Hangman;
use crate::hanoi::Hanoi;
use crate::higher_lower::HigherLower;
use crate::klondike_golf::KlondikeGolf;
use crate::lights_out::LightsOut;
use crate::mahjong_solitaire::MahjongSolitaire;
use crate::mancala::Mancala;
use crate::mastermind::Mastermind;
use crate::match_three::MatchThree;
use crate::maze_walk::MazeWalk;
use crate::memory_pairs::MemoryPairs;
use crate::minesweeper::Minesweeper;
use crate::misc_games::{MiscGame, MiscKind};
use crate::munch_maze::MunchMaze;
use crate::nim::Nim;
use crate::nonogram::Nonogram;
use crate::number_match::NumberMatch;
use crate::one_room_roguelike::OneRoomRoguelike;
use crate::paddle_duel::PaddleDuel;
use crate::peg_solitaire::PegSolitaire;
use crate::pipe_loop::PipeLoop;
use crate::potion_2048::Potion2048;
use crate::pyramid::Pyramid;
use crate::reversi::Reversi;
use crate::sliding_puzzle::SlidingPuzzle;
use crate::snake::Snake;
use crate::sokoban::Sokoban;
use crate::solitaire::{CardSource, Solitaire};
use crate::space_invaders::SpaceInvaders;
use crate::spider::Spider;
use crate::spider_solitaire::SpiderSolitaire;
use crate::sudoku::Sudoku;
use crate::terrain_cannon::TerrainCannon;
use crate::tic_tac_toe::TicTacToe;
use crate::tiny_tower_defence::TinyTowerDefence;
use crate::tri_peaks::TriPeaks;
use crate::word_grid::WordGrid;
use crate::word_ladder::WordLadder;
use crate::word_search::WordSearch;

#[derive(Debug, Clone)]
pub struct GameStore {
    pub game: Game2048,
    pub minesweeper: Minesweeper,
    pub sudoku: Sudoku,
    pub nonogram: Nonogram,
    pub nonogram_zoomed: bool,
    pub nonogram_focus: (usize, usize),
    pub solitaire: Solitaire,
    pub solitaire_peek: Option<CardSource>,
    pub freecell: FreeCell,
    pub fivefold: Fivefold,
    pub reversi: Reversi,
    pub lights_out: LightsOut,
    pub tic_tac_toe: TicTacToe,
    pub memory_pairs: MemoryPairs,
    pub sliding_puzzle: SlidingPuzzle,
    pub mastermind: Mastermind,
    pub spider: Spider,
    pub word_search: WordSearch,
    pub hangman: Hangman,
    pub connect_four: ConnectFour,
    pub checkers: Checkers,
    pub peg_solitaire: PegSolitaire,
    pub mahjong_solitaire: MahjongSolitaire,
    pub snake: Snake,
    pub breakout: Breakout,
    pub higher_lower: HigherLower,
    pub klondike_golf: KlondikeGolf,
    pub blackjack: Blackjack,
    pub spider_solitaire: SpiderSolitaire,
    pub dungeon_sweeper: DungeonSweeper,
    pub potion_2048: Potion2048,
    pub tiny_tower_defence: TinyTowerDefence,
    pub one_room_roguelike: OneRoomRoguelike,
    pub daily_dungeon: DailyDungeon,
    pub dots_boxes: DotsBoxes,
    pub sokoban: Sokoban,
    pub mancala: Mancala,
    pub hanoi: Hanoi,
    pub number_match: NumberMatch,
    pub flood_it: FloodIt,
    pub color_sort: ColorSort,
    pub battleship: Battleship,
    pub word_grid: WordGrid,
    pub pipe_loop: PipeLoop,
    pub maze_walk: MazeWalk,
    pub match_three: MatchThree,
    pub pyramid: Pyramid,
    pub tri_peaks: TriPeaks,
    pub nim: Nim,
    pub word_ladder: WordLadder,
    pub space_invaders: SpaceInvaders,
    pub asteroids: Asteroids,
    pub frogger: Frogger,
    pub munch_maze: MunchMaze,
    pub block_stack: BlockStack,
    pub terrain_cannon: TerrainCannon,
    pub fling_fury: FlingFury,
    pub paddle_duel: PaddleDuel,
    pub riddle_room: MiscGame,
    pub pattern_vault: MiscGame,
    pub sum_circuit: MiscGame,
    pub orbit_order: MiscGame,
    pub word_forge: MiscGame,
}

impl Default for GameStore {
    fn default() -> Self {
        Self {
            game: Game2048::default(),
            minesweeper: Minesweeper::beginner(0x001D_1E51),
            sudoku: Sudoku::new(),
            nonogram: Nonogram::default(),
            nonogram_zoomed: false,
            nonogram_focus: (0, 0),
            solitaire: Solitaire::default(),
            solitaire_peek: None,
            freecell: FreeCell::default(),
            fivefold: Fivefold::default(),
            reversi: Reversi::default(),
            lights_out: LightsOut::default(),
            tic_tac_toe: TicTacToe::default(),
            memory_pairs: MemoryPairs::default(),
            sliding_puzzle: SlidingPuzzle::default(),
            mastermind: Mastermind::default(),
            spider: Spider::default(),
            word_search: WordSearch::default(),
            hangman: Hangman::default(),
            connect_four: ConnectFour::default(),
            checkers: Checkers::default(),
            peg_solitaire: PegSolitaire::default(),
            mahjong_solitaire: MahjongSolitaire::default(),
            snake: Snake::default(),
            breakout: Breakout::default(),
            higher_lower: HigherLower::default(),
            klondike_golf: KlondikeGolf::default(),
            blackjack: Blackjack::default(),
            spider_solitaire: SpiderSolitaire::default(),
            dungeon_sweeper: DungeonSweeper::default(),
            potion_2048: Potion2048::default(),
            tiny_tower_defence: TinyTowerDefence::default(),
            one_room_roguelike: OneRoomRoguelike::default(),
            daily_dungeon: DailyDungeon::default(),
            dots_boxes: DotsBoxes::default(),
            sokoban: Sokoban::default(),
            mancala: Mancala::default(),
            hanoi: Hanoi::default(),
            number_match: NumberMatch::default(),
            flood_it: FloodIt::default(),
            color_sort: ColorSort::default(),
            battleship: Battleship::default(),
            word_grid: WordGrid::default(),
            pipe_loop: PipeLoop::default(),
            maze_walk: MazeWalk::default(),
            match_three: MatchThree::default(),
            pyramid: Pyramid::default(),
            tri_peaks: TriPeaks::default(),
            nim: Nim::default(),
            word_ladder: WordLadder::default(),
            space_invaders: SpaceInvaders::default(),
            asteroids: Asteroids::default(),
            frogger: Frogger::default(),
            munch_maze: MunchMaze::default(),
            block_stack: BlockStack::default(),
            terrain_cannon: TerrainCannon::default(),
            fling_fury: FlingFury::default(),
            paddle_duel: PaddleDuel::default(),
            riddle_room: MiscGame::new(0x4D49_5343_0001, MiscKind::RiddleRoom),
            pattern_vault: MiscGame::new(0x4D49_5343_0002, MiscKind::PatternVault),
            sum_circuit: MiscGame::new(0x4D49_5343_0003, MiscKind::SumCircuit),
            orbit_order: MiscGame::new(0x4D49_5343_0004, MiscKind::OrbitOrder),
            word_forge: MiscGame::new(0x4D49_5343_0005, MiscKind::WordForge),
        }
    }
}
