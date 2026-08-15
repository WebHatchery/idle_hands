//! Application state and the deterministic 2048 rules engine.

use crate::battleship::Battleship;
use crate::blackjack::Blackjack;
use crate::breakout::Breakout;
use crate::checkers::Checkers;
use crate::color_sort::ColorSort;
use crate::connect_four::ConnectFour;
use crate::daily_dungeon::DailyDungeon;
use crate::dots_boxes::DotsBoxes;
use crate::dungeon_sweeper::DungeonSweeper;
use crate::fivefold::Fivefold;
use crate::flood_it::FloodIt;
use crate::freecell::FreeCell;
use crate::hangman::Hangman;
use crate::hanoi::Hanoi;
use crate::higher_lower::HigherLower;
use crate::klondike_golf::KlondikeGolf;
use crate::lights_out::LightsOut;
use crate::mahjong_solitaire::MahjongSolitaire;
use crate::mancala::Mancala;
use crate::mastermind::Mastermind;
use crate::memory_pairs::MemoryPairs;
use crate::minesweeper::Minesweeper;
use crate::nonogram::Nonogram;
use crate::number_match::NumberMatch;
use crate::one_room_roguelike::OneRoomRoguelike;
use crate::peg_solitaire::PegSolitaire;
use crate::potion_2048::Potion2048;
use crate::reversi::Reversi;
use crate::sliding_puzzle::SlidingPuzzle;
use crate::snake::Snake;
use crate::sokoban::Sokoban;
use crate::solitaire::Solitaire;
use crate::spider::Spider;
use crate::spider_solitaire::SpiderSolitaire;
use crate::sudoku::Sudoku;
use crate::tic_tac_toe::TicTacToe;
use crate::tiny_tower_defence::TinyTowerDefence;
use crate::word_search::WordSearch;
use serde::{Deserialize, Serialize};

pub use crate::game_2048::Game2048;

pub use crate::state_snapshots::GameSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameId {
    Solitaire,
    FreeCell,
    Sudoku,
    Minesweeper,
    Game2048,
    Nonogram,
    Yahtzee,
    Reversi,
    LightsOut,
    TicTacToe,
    MemoryPairs,
    SlidingPuzzle,
    Mastermind,
    Spider,
    WordSearch,
    Hangman,
    ConnectFour,
    Checkers,
    PegSolitaire,
    MahjongSolitaire,
    Snake,
    Breakout,
    HigherLower,
    KlondikeGolf,
    Blackjack,
    SpiderSolitaire,
    DungeonSweeper,
    Potion2048,
    TinyTowerDefence,
    OneRoomRoguelike,
    DailyDungeon,
    DotsBoxes,
    Sokoban,
    Mancala,
    Hanoi,
    NumberMatch,
    FloodIt,
    ColorSort,
    Battleship,
}
impl GameId {
    pub const ALL: [Self; 39] = [
        Self::Solitaire,
        Self::FreeCell,
        Self::Sudoku,
        Self::Minesweeper,
        Self::Game2048,
        Self::Nonogram,
        Self::Yahtzee,
        Self::Reversi,
        Self::LightsOut,
        Self::TicTacToe,
        Self::MemoryPairs,
        Self::SlidingPuzzle,
        Self::Mastermind,
        Self::Spider,
        Self::WordSearch,
        Self::Hangman,
        Self::ConnectFour,
        Self::Checkers,
        Self::PegSolitaire,
        Self::MahjongSolitaire,
        Self::Snake,
        Self::Breakout,
        Self::HigherLower,
        Self::KlondikeGolf,
        Self::Blackjack,
        Self::SpiderSolitaire,
        Self::DungeonSweeper,
        Self::Potion2048,
        Self::TinyTowerDefence,
        Self::OneRoomRoguelike,
        Self::DailyDungeon,
        Self::DotsBoxes,
        Self::Sokoban,
        Self::Mancala,
        Self::Hanoi,
        Self::NumberMatch,
        Self::FloodIt,
        Self::ColorSort,
        Self::Battleship,
    ];
    pub fn title(self) -> &'static str {
        match self {
            Self::Game2048 => "2048",
            Self::FreeCell => "FreeCell",
            Self::Minesweeper => "Minesweeper",
            Self::Nonogram => "Nonogram",
            Self::Solitaire => "Solitaire",
            Self::Sudoku => "Sudoku",
            Self::Yahtzee => "Fivefold",
            Self::Reversi => "Reversi",
            Self::LightsOut => "Lights Out",
            Self::TicTacToe => "Tic-Tac-Toe",
            Self::MemoryPairs => "Memory",
            Self::SlidingPuzzle => "Sliding Puzzle",
            Self::Mastermind => "Mastermind",
            Self::Spider => "Spider",
            Self::WordSearch => "Word Search",
            Self::Hangman => "Hangman",
            Self::ConnectFour => "Connect Four",
            Self::Checkers => "Checkers",
            Self::PegSolitaire => "Peg Solitaire",
            Self::MahjongSolitaire => "Mahjong Solitaire",
            Self::Snake => "Snake",
            Self::Breakout => "Breakout",
            Self::HigherLower => "Higher or Lower",
            Self::KlondikeGolf => "Klondike Golf",
            Self::Blackjack => "Blackjack",
            Self::SpiderSolitaire => "Spider Solitaire",
            Self::DungeonSweeper => "Dungeon Sweeper",
            Self::Potion2048 => "Potion 2048",
            Self::TinyTowerDefence => "Tiny Tower Defence",
            Self::OneRoomRoguelike => "One Room Roguelike",
            Self::DailyDungeon => "Daily Dungeon",
            Self::DotsBoxes => "Dots & Boxes",
            Self::Sokoban => "Sokoban",
            Self::Mancala => "Mancala",
            Self::Hanoi => "Hanoi",
            Self::NumberMatch => "Number Match",
            Self::FloodIt => "Flood It",
            Self::ColorSort => "Color Sort",
            Self::Battleship => "Battleship",
        }
    }
    pub fn subtitle(self) -> &'static str {
        match self {
            Self::Game2048 => "Slide the cabinet tiles",
            Self::Solitaire => "Classic card table",
            Self::FreeCell => "Four open cells",
            Self::Sudoku => "Numbers in every nook",
            Self::Minesweeper => "Read the quiet field",
            Self::Nonogram => "Paint the hidden picture",
            Self::Yahtzee => "Five dice, thirteen calls",
            Self::Reversi => "Turn the board",
            Self::LightsOut => "Quiet the lights",
            Self::TicTacToe => "Three in a row",
            Self::MemoryPairs => "Find the quiet pairs",
            Self::SlidingPuzzle => "Move the quiet tiles",
            Self::Mastermind => "Read the color code",
            Self::Spider => "Build the quiet webs",
            Self::WordSearch => "Find the hidden words",
            Self::Hangman => "Keep the quiet word",
            Self::ConnectFour => "Drop the quiet discs",
            Self::Checkers => "Turn the quiet pieces",
            Self::PegSolitaire => "Leave one quiet peg",
            Self::MahjongSolitaire => "Pair the quiet tiles",
            Self::Snake => "Guide the quiet coil",
            Self::Breakout => "Bounce the quiet ball",
            Self::HigherLower => "Read the quiet card",
            Self::KlondikeGolf => "Clear the quiet columns",
            Self::Blackjack => "Hold the quiet hand",
            Self::SpiderSolitaire => "Build suited webs",
            Self::DungeonSweeper => "Find the quiet exit",
            Self::Potion2048 => "Brew the next tile",
            Self::TinyTowerDefence => "Keep the quiet lanes",
            Self::OneRoomRoguelike => "Clear one quiet room",
            Self::DailyDungeon => "Recover the daily runes",
            Self::DotsBoxes => "Draw the quiet squares",
            Self::Sokoban => "Push the quiet crates",
            Self::Mancala => "Sow the quiet stones",
            Self::Hanoi => "Move the quiet disks",
            Self::NumberMatch => "Pair the quiet numbers",
            Self::FloodIt => "Fill the quiet field",
            Self::ColorSort => "Sort the quiet colors",
            Self::Battleship => "Find the quiet fleet",
        }
    }
    pub fn index(self) -> usize {
        Self::ALL.iter().position(|game| *game == self).unwrap()
    }
    pub fn save_key(self) -> &'static str {
        match self {
            Self::Solitaire => "solitaire",
            Self::FreeCell => "freecell",
            Self::Sudoku => "sudoku",
            Self::Minesweeper => "minesweeper",
            Self::Game2048 => "2048",
            Self::Nonogram => "nonogram",
            Self::Yahtzee => "fivefold",
            Self::Reversi => "reversi",
            Self::LightsOut => "lights_out",
            Self::TicTacToe => "tic_tac_toe",
            Self::MemoryPairs => "memory_pairs",
            Self::SlidingPuzzle => "sliding_puzzle",
            Self::Mastermind => "mastermind",
            Self::Spider => "spider",
            Self::WordSearch => "word_search",
            Self::Hangman => "hangman",
            Self::ConnectFour => "connect_four",
            Self::Checkers => "checkers",
            Self::PegSolitaire => "peg_solitaire",
            Self::MahjongSolitaire => "mahjong_solitaire",
            Self::Snake => "snake",
            Self::Breakout => "breakout",
            Self::HigherLower => "higher_lower",
            Self::KlondikeGolf => "klondike_golf",
            Self::Blackjack => "blackjack",
            Self::SpiderSolitaire => "spider_solitaire",
            Self::DungeonSweeper => "dungeon_sweeper",
            Self::Potion2048 => "potion_2048",
            Self::TinyTowerDefence => "tiny_tower_defence",
            Self::OneRoomRoguelike => "one_room_roguelike",
            Self::DailyDungeon => "daily_dungeon",
            Self::DotsBoxes => "dots_boxes",
            Self::Sokoban => "sokoban",
            Self::Mancala => "mancala",
            Self::Hanoi => "hanoi",
            Self::NumberMatch => "number_match",
            Self::FloodIt => "flood_it",
            Self::ColorSort => "color_sort",
            Self::Battleship => "battleship",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Cabinet,
    Game(GameId),
    Help,
    Records,
    Rules,
    Credits,
    Settings,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub screen: Screen,
    pub selected: usize,
    pub game: Game2048,
    pub minesweeper: Minesweeper,
    pub sudoku: Sudoku,
    pub nonogram: Nonogram,
    pub nonogram_zoomed: bool,
    pub nonogram_focus: (usize, usize),
    pub solitaire: Solitaire,
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
    pub achievements: [bool; 10],
    pub stamps: u16,
    pub card_back: u8,
    pub board_theme: u8,
    pub sound_set: u8,
    pub cabinet_decoration: u8,
    pub confirm_restart: bool,
    pub confirm_reset: bool,
    pub profile_name: String,
    pub sound: bool,
    pub reduced_motion: bool,
    pub high_contrast: bool,
    pub large_text: bool,
    pub mine_flag_mode: bool,
    pub mine_records: [Option<u32>; 4],
    pub sudoku_note_mode: bool,
    pub records: CollectionRecords,
    pub tutorial: Option<GameId>,
    pub tutorial_seen: [bool; 8],
    pub card_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectionRecords {
    pub best_2048: u32,
    pub minesweeper: [Option<u32>; 4],
    pub sudoku: [Option<u32>; 3],
    pub nonogram: [Option<u32>; 3],
    pub solitaire_best_moves: Option<u32>,
    pub freecell_best_moves: Option<u32>,
    pub fivefold_best_total: u16,
    pub reversi_best_score: u8,
    #[serde(default)]
    pub lights_out_best_moves: Option<u16>,
    #[serde(default)]
    pub tic_tac_toe_best_moves: Option<u8>,
    #[serde(default)]
    pub memory_pairs_best_moves: Option<u16>,
    #[serde(default)]
    pub sliding_puzzle_best_moves: Option<u16>,
    #[serde(default)]
    pub mastermind_best_rows: Option<u8>,
    #[serde(default)]
    pub spider_best_moves: Option<u32>,
    #[serde(default)]
    pub word_search_best_moves: Option<u16>,
    #[serde(default)]
    pub hangman_best_moves: Option<u16>,
    #[serde(default)]
    pub connect_four_best_moves: Option<u8>,
    #[serde(default)]
    pub checkers_best_moves: Option<u16>,
    #[serde(default)]
    pub peg_solitaire_best_moves: Option<u16>,
    #[serde(default)]
    pub mahjong_solitaire_best_moves: Option<u16>,
    #[serde(default)]
    pub snake_best_score: Option<u16>,
    #[serde(default)]
    pub breakout_best_score: Option<u16>,
    #[serde(default)]
    pub higher_lower_best_score: Option<u16>,
    #[serde(default)]
    pub klondike_golf_best_moves: Option<u16>,
    #[serde(default)]
    pub blackjack_best_wins: Option<u16>,
    #[serde(default)]
    pub spider_solitaire_best_moves: Option<u32>,
    #[serde(default)]
    pub dungeon_sweeper_best_moves: Option<u16>,
    #[serde(default)]
    pub potion_2048_best_score: Option<u32>,
    #[serde(default)]
    pub tiny_tower_defence_best_wave: Option<u8>,
    #[serde(default)]
    pub one_room_roguelike_best_score: Option<u32>,
    #[serde(default)]
    pub daily_dungeon_best_score: Option<u32>,
    #[serde(default)]
    pub dots_boxes_best_score: Option<u8>,
    #[serde(default)]
    pub sokoban_best_moves: Option<u16>,
    #[serde(default)]
    pub mancala_best_score: Option<u8>,
    #[serde(default)]
    pub hanoi_best_moves: Option<u16>,
    #[serde(default)]
    pub number_match_best_moves: Option<u16>,
    #[serde(default)]
    pub flood_it_best_moves: Option<u16>,
    #[serde(default)]
    pub color_sort_best_moves: Option<u16>,
    #[serde(default)]
    pub battleship_best_moves: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSave {
    pub version: String,
    pub game: Game2048,
    pub minesweeper: Minesweeper,
    #[serde(default)]
    pub sudoku: Sudoku,
    #[serde(default)]
    pub nonogram: Nonogram,
    #[serde(default)]
    pub solitaire: Solitaire,
    #[serde(default)]
    pub freecell: FreeCell,
    #[serde(default)]
    pub fivefold: Fivefold,
    #[serde(default)]
    pub reversi: Reversi,
    #[serde(default)]
    pub lights_out: LightsOut,
    #[serde(default)]
    pub tic_tac_toe: TicTacToe,
    #[serde(default)]
    pub memory_pairs: MemoryPairs,
    #[serde(default)]
    pub sliding_puzzle: SlidingPuzzle,
    #[serde(default)]
    pub mastermind: Mastermind,
    #[serde(default)]
    pub spider: Spider,
    #[serde(default)]
    pub word_search: WordSearch,
    #[serde(default)]
    pub hangman: Hangman,
    #[serde(default)]
    pub connect_four: ConnectFour,
    #[serde(default)]
    pub checkers: Checkers,
    #[serde(default)]
    pub peg_solitaire: PegSolitaire,
    #[serde(default)]
    pub mahjong_solitaire: MahjongSolitaire,
    #[serde(default)]
    pub snake: Snake,
    #[serde(default)]
    pub breakout: Breakout,
    #[serde(default)]
    pub higher_lower: HigherLower,
    #[serde(default)]
    pub klondike_golf: KlondikeGolf,
    #[serde(default)]
    pub blackjack: Blackjack,
    #[serde(default)]
    pub spider_solitaire: SpiderSolitaire,
    #[serde(default)]
    pub dungeon_sweeper: DungeonSweeper,
    #[serde(default)]
    pub potion_2048: Potion2048,
    #[serde(default)]
    pub tiny_tower_defence: TinyTowerDefence,
    #[serde(default)]
    pub one_room_roguelike: OneRoomRoguelike,
    #[serde(default)]
    pub daily_dungeon: DailyDungeon,
    #[serde(default)]
    pub dots_boxes: DotsBoxes,
    #[serde(default)]
    pub sokoban: Sokoban,
    #[serde(default)]
    pub mancala: Mancala,
    #[serde(default)]
    pub hanoi: Hanoi,
    #[serde(default)]
    pub number_match: NumberMatch,
    #[serde(default)]
    pub flood_it: FloodIt,
    #[serde(default)]
    pub color_sort: ColorSort,
    #[serde(default)]
    pub battleship: Battleship,
    pub profile_name: String,
    pub sound: bool,
    pub reduced_motion: bool,
    #[serde(default)]
    pub high_contrast: bool,
    #[serde(default)]
    pub large_text: bool,
    pub mine_flag_mode: bool,
    pub mine_records: [Option<u32>; 4],
    #[serde(default)]
    pub sudoku_note_mode: bool,
    #[serde(default)]
    pub records: CollectionRecords,
    #[serde(default)]
    pub achievements: [bool; 10],
    #[serde(default)]
    pub stamps: u16,
    #[serde(default)]
    pub card_back: u8,
    #[serde(default)]
    pub board_theme: u8,
    #[serde(default)]
    pub sound_set: u8,
    #[serde(default)]
    pub cabinet_decoration: u8,
    #[serde(default)]
    pub tutorial_seen: [bool; 8],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSave {
    pub version: String,
    pub profile_name: String,
    pub sound: bool,
    pub reduced_motion: bool,
    #[serde(default)]
    pub high_contrast: bool,
    #[serde(default)]
    pub large_text: bool,
    pub mine_flag_mode: bool,
    pub mine_records: [Option<u32>; 4],
    pub sudoku_note_mode: bool,
    pub records: CollectionRecords,
    #[serde(default)]
    pub achievements: [bool; 10],
    #[serde(default)]
    pub stamps: u16,
    #[serde(default)]
    pub card_back: u8,
    #[serde(default)]
    pub board_theme: u8,
    #[serde(default)]
    pub sound_set: u8,
    #[serde(default)]
    pub cabinet_decoration: u8,
    pub tutorial_seen: [bool; 8],
}
impl ProfileSave {
    pub fn from_state(state: &AppState, version: &str) -> Self {
        Self {
            version: version.to_owned(),
            profile_name: state.profile_name.clone(),
            sound: state.sound,
            reduced_motion: state.reduced_motion,
            high_contrast: state.high_contrast,
            large_text: state.large_text,
            mine_flag_mode: state.mine_flag_mode,
            mine_records: state.mine_records,
            sudoku_note_mode: state.sudoku_note_mode,
            records: state.records.clone(),
            achievements: state.achievements,
            stamps: state.stamps,
            card_back: state.card_back,
            board_theme: state.board_theme,
            sound_set: state.sound_set,
            cabinet_decoration: state.cabinet_decoration,
            tutorial_seen: state.tutorial_seen,
        }
    }
    pub fn apply_to(self, state: &mut AppState) {
        state.profile_name = self.profile_name;
        state.sound = self.sound;
        state.reduced_motion = self.reduced_motion;
        state.high_contrast = self.high_contrast;
        state.large_text = self.large_text;
        state.mine_flag_mode = self.mine_flag_mode;
        state.mine_records = self.mine_records;
        state.sudoku_note_mode = self.sudoku_note_mode;
        state.records = self.records;
        state.achievements = self.achievements;
        state.stamps = self.stamps;
        state.card_back = self.card_back;
        state.board_theme = self.board_theme;
        state.sound_set = self.sound_set;
        state.cabinet_decoration = self.cabinet_decoration;
        state.tutorial_seen = self.tutorial_seen;
    }
}

impl CollectionSave {
    pub fn from_state(state: &AppState, version: &str) -> Self {
        Self {
            version: version.to_owned(),
            game: state.game.clone(),
            minesweeper: state.minesweeper.clone(),
            sudoku: state.sudoku.clone(),
            nonogram: state.nonogram.clone(),
            solitaire: state.solitaire.clone(),
            freecell: state.freecell.clone(),
            fivefold: state.fivefold.clone(),
            reversi: state.reversi.clone(),
            lights_out: state.lights_out.clone(),
            tic_tac_toe: state.tic_tac_toe.clone(),
            memory_pairs: state.memory_pairs.clone(),
            sliding_puzzle: state.sliding_puzzle.clone(),
            mastermind: state.mastermind.clone(),
            spider: state.spider.clone(),
            word_search: state.word_search.clone(),
            hangman: state.hangman.clone(),
            connect_four: state.connect_four.clone(),
            checkers: state.checkers.clone(),
            peg_solitaire: state.peg_solitaire.clone(),
            mahjong_solitaire: state.mahjong_solitaire.clone(),
            snake: state.snake.clone(),
            breakout: state.breakout.clone(),
            higher_lower: state.higher_lower.clone(),
            klondike_golf: state.klondike_golf.clone(),
            blackjack: state.blackjack.clone(),
            spider_solitaire: state.spider_solitaire.clone(),
            dungeon_sweeper: state.dungeon_sweeper.clone(),
            potion_2048: state.potion_2048.clone(),
            tiny_tower_defence: state.tiny_tower_defence.clone(),
            one_room_roguelike: state.one_room_roguelike.clone(),
            daily_dungeon: state.daily_dungeon.clone(),
            dots_boxes: state.dots_boxes.clone(),
            sokoban: state.sokoban.clone(),
            mancala: state.mancala.clone(),
            hanoi: state.hanoi.clone(),
            number_match: state.number_match.clone(),
            flood_it: state.flood_it.clone(),
            color_sort: state.color_sort.clone(),
            battleship: state.battleship.clone(),
            profile_name: state.profile_name.clone(),
            sound: state.sound,
            reduced_motion: state.reduced_motion,
            high_contrast: state.high_contrast,
            large_text: state.large_text,
            mine_flag_mode: state.mine_flag_mode,
            mine_records: state.mine_records,
            sudoku_note_mode: state.sudoku_note_mode,
            records: state.records.clone(),
            achievements: state.achievements,
            stamps: state.stamps,
            card_back: state.card_back,
            board_theme: state.board_theme,
            sound_set: state.sound_set,
            cabinet_decoration: state.cabinet_decoration,
            tutorial_seen: state.tutorial_seen,
        }
    }
    pub fn apply_to(self, state: &mut AppState) {
        state.game = self.game;
        state.minesweeper = self.minesweeper;
        state.sudoku = self.sudoku;
        state.nonogram = self.nonogram;
        state.solitaire = self.solitaire;
        state.freecell = self.freecell;
        state.fivefold = self.fivefold;
        state.reversi = self.reversi;
        state.lights_out = self.lights_out;
        state.tic_tac_toe = self.tic_tac_toe;
        state.memory_pairs = self.memory_pairs;
        state.sliding_puzzle = self.sliding_puzzle;
        state.mastermind = self.mastermind;
        state.spider = self.spider;
        state.word_search = self.word_search;
        state.hangman = self.hangman;
        state.connect_four = self.connect_four;
        state.checkers = self.checkers;
        state.peg_solitaire = self.peg_solitaire;
        state.mahjong_solitaire = self.mahjong_solitaire;
        state.snake = self.snake;
        state.breakout = self.breakout;
        state.higher_lower = self.higher_lower;
        state.klondike_golf = self.klondike_golf;
        state.blackjack = self.blackjack;
        state.spider_solitaire = self.spider_solitaire;
        state.dungeon_sweeper = self.dungeon_sweeper;
        state.potion_2048 = self.potion_2048;
        state.tiny_tower_defence = self.tiny_tower_defence;
        state.one_room_roguelike = self.one_room_roguelike;
        state.daily_dungeon = self.daily_dungeon;
        state.dots_boxes = self.dots_boxes;
        state.sokoban = self.sokoban;
        state.mancala = self.mancala;
        state.hanoi = self.hanoi;
        state.number_match = self.number_match;
        state.flood_it = self.flood_it;
        state.color_sort = self.color_sort;
        state.battleship = self.battleship;
        state.profile_name = self.profile_name;
        state.sound = self.sound;
        state.reduced_motion = self.reduced_motion;
        state.high_contrast = self.high_contrast;
        state.large_text = self.large_text;
        state.mine_flag_mode = self.mine_flag_mode;
        state.mine_records = self.mine_records;
        state.sudoku_note_mode = self.sudoku_note_mode;
        state.records = self.records;
        state.achievements = self.achievements;
        state.stamps = self.stamps;
        state.card_back = self.card_back;
        state.board_theme = self.board_theme;
        state.sound_set = self.sound_set;
        state.cabinet_decoration = self.cabinet_decoration;
        state.tutorial_seen = self.tutorial_seen;
    }
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: Screen::Cabinet,
            selected: 4,
            game: Game2048::default(),
            minesweeper: Minesweeper::beginner(0x001D_1E51),
            sudoku: Sudoku::new(),
            nonogram: Nonogram::default(),
            nonogram_zoomed: false,
            nonogram_focus: (0, 0),
            solitaire: Solitaire::default(),
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
            achievements: [false; 10],
            stamps: 0,
            card_back: 0,
            board_theme: 0,
            sound_set: 0,
            cabinet_decoration: 0,
            confirm_restart: false,
            confirm_reset: false,
            profile_name: "Cabinet Guest".into(),
            sound: true,
            reduced_motion: false,
            high_contrast: false,
            large_text: false,
            mine_flag_mode: false,
            mine_records: [None, None, None, None],
            sudoku_note_mode: false,
            records: CollectionRecords::default(),
            tutorial: None,
            tutorial_seen: [false; 8],
            card_hint: None,
        }
    }
}

#[cfg(test)]
mod tests;
