//! Application state and the deterministic 2048 rules engine.

use crate::blackjack::Blackjack;
use crate::breakout::Breakout;
use crate::checkers::Checkers;
use crate::connect_four::ConnectFour;
use crate::fivefold::Fivefold;
use crate::freecell::FreeCell;
use crate::hangman::Hangman;
use crate::higher_lower::HigherLower;
use crate::klondike_golf::KlondikeGolf;
use crate::lights_out::LightsOut;
use crate::mahjong_solitaire::MahjongSolitaire;
use crate::mastermind::Mastermind;
use crate::memory_pairs::MemoryPairs;
use crate::minesweeper::Minesweeper;
use crate::nonogram::Nonogram;
use crate::peg_solitaire::PegSolitaire;
use crate::reversi::Reversi;
use crate::sliding_puzzle::SlidingPuzzle;
use crate::snake::Snake;
use crate::solitaire::Solitaire;
use crate::spider::Spider;
use crate::sudoku::Sudoku;
use crate::tic_tac_toe::TicTacToe;
use crate::word_search::WordSearch;
use serde::{Deserialize, Serialize};

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
}
impl GameId {
    pub const ALL: [Self; 25] = [
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game2048 {
    pub cells: [u16; 16],
    pub score: u32,
    pub best: u32,
    pub seed: u64,
    #[serde(skip)]
    undo: Option<([u16; 16], u32, u64)>,
}
impl Default for Game2048 {
    fn default() -> Self {
        Self::new(0x1D1E_2048)
    }
}
impl Game2048 {
    pub fn new(seed: u64) -> Self {
        let mut game = Self {
            cells: [0; 16],
            score: 0,
            best: 0,
            seed,
            undo: None,
        };
        game.spawn();
        game.spawn();
        game
    }
    pub fn move_in(&mut self, direction: Direction) -> bool {
        let before = self.cells;
        let before_score = self.score;
        let before_seed = self.seed;
        let mut changed = false;
        for line in 0..4 {
            let indices = match direction {
                Direction::Left => [line * 4, line * 4 + 1, line * 4 + 2, line * 4 + 3],
                Direction::Right => [line * 4 + 3, line * 4 + 2, line * 4 + 1, line * 4],
                Direction::Up => [line, line + 4, line + 8, line + 12],
                Direction::Down => [line + 12, line + 8, line + 4, line],
            };
            let values: Vec<u16> = indices
                .iter()
                .map(|&i| self.cells[i])
                .filter(|&v| v != 0)
                .collect();
            let mut merged = Vec::with_capacity(4);
            let mut i = 0;
            while i < values.len() {
                if i + 1 < values.len() && values[i] == values[i + 1] {
                    merged.push(values[i] * 2);
                    self.score += values[i] as u32 * 2;
                    i += 2;
                } else {
                    merged.push(values[i]);
                    i += 1;
                }
            }
            for (slot, &index) in indices.iter().enumerate() {
                let value = merged.get(slot).copied().unwrap_or(0);
                if self.cells[index] != value {
                    changed = true;
                }
                self.cells[index] = value;
            }
        }
        if changed {
            self.undo = Some((before, before_score, before_seed));
            self.spawn();
            self.best = self.best.max(self.score);
        }
        changed
    }
    pub fn undo(&mut self) -> bool {
        if let Some((cells, score, seed)) = self.undo.take() {
            self.cells = cells;
            self.score = score;
            self.seed = seed;
            true
        } else {
            false
        }
    }
    pub fn can_undo(&self) -> bool {
        self.undo.is_some()
    }
    #[allow(dead_code)]
    pub fn can_move(&self) -> bool {
        self.cells.contains(&0)
            || (0..4).any(|r| (0..3).any(|c| self.cells[r * 4 + c] == self.cells[r * 4 + c + 1]))
            || (0..3).any(|r| (0..4).any(|c| self.cells[r * 4 + c] == self.cells[(r + 1) * 4 + c]))
    }
    pub fn won(&self) -> bool {
        self.cells.iter().any(|&v| v >= 2048)
    }
    fn spawn(&mut self) {
        let empty: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v == 0).then_some(i))
            .collect();
        if empty.is_empty() {
            return;
        }
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442690888963407);
        let index = empty[(self.seed as usize) % empty.len()];
        self.seed = self
            .seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442690888963407);
        self.cells[index] = if self.seed & 7 == 0 { 4 } else { 2 };
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameSnapshot {
    Game2048(Game2048),
    Minesweeper(Minesweeper),
    Sudoku(Sudoku),
    Nonogram(Nonogram),
    Solitaire(Solitaire),
    FreeCell(FreeCell),
    Fivefold(Fivefold),
    Reversi(Reversi),
    LightsOut(LightsOut),
    TicTacToe(TicTacToe),
    MemoryPairs(MemoryPairs),
    SlidingPuzzle(SlidingPuzzle),
    Mastermind(Mastermind),
    Spider(Spider),
    WordSearch(WordSearch),
    Hangman(Hangman),
    ConnectFour(ConnectFour),
    Checkers(Checkers),
    PegSolitaire(PegSolitaire),
    MahjongSolitaire(MahjongSolitaire),
    Snake(Snake),
    Breakout(Breakout),
    HigherLower(HigherLower),
    KlondikeGolf(KlondikeGolf),
    Blackjack(Blackjack),
}
impl GameSnapshot {
    pub fn from_state(state: &AppState, game: GameId) -> Self {
        match game {
            GameId::Game2048 => Self::Game2048(state.game.clone()),
            GameId::Minesweeper => Self::Minesweeper(state.minesweeper.clone()),
            GameId::Sudoku => Self::Sudoku(state.sudoku.clone()),
            GameId::Nonogram => Self::Nonogram(state.nonogram.clone()),
            GameId::Solitaire => Self::Solitaire(state.solitaire.clone()),
            GameId::FreeCell => Self::FreeCell(state.freecell.clone()),
            GameId::Yahtzee => Self::Fivefold(state.fivefold.clone()),
            GameId::Reversi => Self::Reversi(state.reversi.clone()),
            GameId::LightsOut => Self::LightsOut(state.lights_out.clone()),
            GameId::TicTacToe => Self::TicTacToe(state.tic_tac_toe.clone()),
            GameId::MemoryPairs => Self::MemoryPairs(state.memory_pairs.clone()),
            GameId::SlidingPuzzle => Self::SlidingPuzzle(state.sliding_puzzle.clone()),
            GameId::Mastermind => Self::Mastermind(state.mastermind.clone()),
            GameId::Spider => Self::Spider(state.spider.clone()),
            GameId::WordSearch => Self::WordSearch(state.word_search.clone()),
            GameId::Hangman => Self::Hangman(state.hangman.clone()),
            GameId::ConnectFour => Self::ConnectFour(state.connect_four.clone()),
            GameId::Checkers => Self::Checkers(state.checkers.clone()),
            GameId::PegSolitaire => Self::PegSolitaire(state.peg_solitaire.clone()),
            GameId::MahjongSolitaire => Self::MahjongSolitaire(state.mahjong_solitaire.clone()),
            GameId::Snake => Self::Snake(state.snake.clone()),
            GameId::Breakout => Self::Breakout(state.breakout.clone()),
            GameId::HigherLower => Self::HigherLower(state.higher_lower.clone()),
            GameId::KlondikeGolf => Self::KlondikeGolf(state.klondike_golf.clone()),
            GameId::Blackjack => Self::Blackjack(state.blackjack.clone()),
        }
    }
    pub fn apply_to(self, state: &mut AppState) {
        match self {
            Self::Game2048(game) => state.game = game,
            Self::Minesweeper(game) => state.minesweeper = game,
            Self::Sudoku(game) => state.sudoku = game,
            Self::Nonogram(game) => state.nonogram = game,
            Self::Solitaire(game) => state.solitaire = game,
            Self::FreeCell(game) => state.freecell = game,
            Self::Fivefold(game) => state.fivefold = game,
            Self::Reversi(game) => state.reversi = game,
            Self::LightsOut(game) => state.lights_out = game,
            Self::TicTacToe(game) => state.tic_tac_toe = game,
            Self::MemoryPairs(game) => state.memory_pairs = game,
            Self::SlidingPuzzle(game) => state.sliding_puzzle = game,
            Self::Mastermind(game) => state.mastermind = game,
            Self::Spider(game) => state.spider = game,
            Self::WordSearch(game) => state.word_search = game,
            Self::Hangman(game) => state.hangman = game,
            Self::ConnectFour(game) => state.connect_four = game,
            Self::Checkers(game) => state.checkers = game,
            Self::PegSolitaire(game) => state.peg_solitaire = game,
            Self::MahjongSolitaire(game) => state.mahjong_solitaire = game,
            Self::Snake(game) => state.snake = game,
            Self::Breakout(game) => state.breakout = game,
            Self::HigherLower(game) => state.higher_lower = game,
            Self::KlondikeGolf(game) => state.klondike_golf = game,
            Self::Blackjack(game) => state.blackjack = game,
        }
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
