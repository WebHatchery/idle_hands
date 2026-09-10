//! Application state and the deterministic 2048 rules engine.

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
use crate::game_store::GameStore;
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
use crate::solitaire::Solitaire;
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
use serde::{Deserialize, Serialize};

#[path = "state_initialization.rs"]
mod state_initialization;
#[path = "state_profile.rs"]
mod state_profile;
pub use state_profile::ProfileSave;

pub use crate::state_navigation::Screen;
pub use crate::state_records::CollectionRecords;

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
    WordGrid,
    PipeLoop,
    MazeWalk,
    MatchThree,
    Pyramid,
    TriPeaks,
    Nim,
    WordLadder,
    SpaceInvaders,
    Asteroids,
    Frogger,
    MunchMaze,
    BlockStack,
    TerrainCannon,
    FlingFury,
    PaddleDuel,
    RiddleRoom,
    PatternVault,
    SumCircuit,
    OrbitOrder,
    WordForge,
}
impl GameId {
    pub const ALL: [Self; 60] = [
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
        Self::WordGrid,
        Self::PipeLoop,
        Self::MazeWalk,
        Self::MatchThree,
        Self::Pyramid,
        Self::TriPeaks,
        Self::Nim,
        Self::WordLadder,
        Self::SpaceInvaders,
        Self::Asteroids,
        Self::Frogger,
        Self::MunchMaze,
        Self::BlockStack,
        Self::TerrainCannon,
        Self::FlingFury,
        Self::PaddleDuel,
        Self::RiddleRoom,
        Self::PatternVault,
        Self::SumCircuit,
        Self::OrbitOrder,
        Self::WordForge,
    ];
    pub fn descriptor(self) -> &'static crate::game_descriptor::GameDescriptor {
        crate::game_descriptor::descriptor(self)
    }
    pub fn title(self) -> &'static str {
        self.descriptor().title
    }
    pub fn subtitle(self) -> &'static str {
        self.descriptor().subtitle
    }
    pub fn index(self) -> usize {
        self.descriptor().index
    }
    pub fn save_key(self) -> &'static str {
        self.descriptor().save_key
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub games: GameStore,
    pub screen: Screen,
    pub selected: usize,
    pub achievements: Vec<bool>,
    pub stamps: u16,
    pub card_back: u8,
    pub board_theme: u8,
    pub sound_set: u8,
    pub cabinet_decoration: u8,
    pub confirm_restart: bool,
    pub pending_restart: Option<crate::ui::UiAction>,
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
    pub tutorial_seen: Vec<bool>,
    pub favorites: Vec<bool>,
    pub recent_games: Vec<GameId>,
    pub card_hint: Option<String>,
    pub favorites_view: bool,
    pub recent_view: bool,
    pub achievements_view: bool,
    pub achievement_filter: u8,
    pub cabinet_filter: u8,
    pub cabinet_scroll: usize,
    pub library_scroll: usize,
    pub fivefold_score_page: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionSave {
    pub version: String,
    #[serde(default = "default_selected")]
    pub selected: usize,
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
    #[serde(default)]
    pub word_grid: WordGrid,
    #[serde(default)]
    pub pipe_loop: PipeLoop,
    #[serde(default)]
    pub maze_walk: MazeWalk,
    #[serde(default)]
    pub match_three: MatchThree,
    #[serde(default)]
    pub pyramid: Pyramid,
    #[serde(default)]
    pub tri_peaks: TriPeaks,
    #[serde(default)]
    pub nim: Nim,
    #[serde(default)]
    pub word_ladder: WordLadder,
    #[serde(default)]
    pub space_invaders: SpaceInvaders,
    #[serde(default)]
    pub asteroids: Asteroids,
    #[serde(default)]
    pub frogger: Frogger,
    #[serde(default)]
    pub munch_maze: MunchMaze,
    #[serde(default)]
    pub block_stack: BlockStack,
    #[serde(default)]
    pub terrain_cannon: TerrainCannon,
    #[serde(default)]
    pub fling_fury: FlingFury,
    #[serde(default)]
    pub paddle_duel: PaddleDuel,
    #[serde(default = "default_riddle_room")]
    pub riddle_room: MiscGame,
    #[serde(default = "default_pattern_vault")]
    pub pattern_vault: MiscGame,
    #[serde(default = "default_sum_circuit")]
    pub sum_circuit: MiscGame,
    #[serde(default = "default_orbit_order")]
    pub orbit_order: MiscGame,
    #[serde(default = "default_word_forge")]
    pub word_forge: MiscGame,
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
    pub achievements: Vec<bool>,
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
    pub tutorial_seen: Vec<bool>,
    #[serde(default)]
    pub favorites: Vec<bool>,
    #[serde(default)]
    pub recent_games: Vec<GameId>,
}

fn default_selected() -> usize {
    4
}

fn default_riddle_room() -> MiscGame {
    MiscGame::new(0x4D49_5343_0001, MiscKind::RiddleRoom)
}

fn default_pattern_vault() -> MiscGame {
    MiscGame::new(0x4D49_5343_0002, MiscKind::PatternVault)
}

fn default_sum_circuit() -> MiscGame {
    MiscGame::new(0x4D49_5343_0003, MiscKind::SumCircuit)
}

fn default_orbit_order() -> MiscGame {
    MiscGame::new(0x4D49_5343_0004, MiscKind::OrbitOrder)
}

fn default_word_forge() -> MiscGame {
    MiscGame::new(0x4D49_5343_0005, MiscKind::WordForge)
}

impl CollectionSave {
    #[cfg(test)]
    pub fn from_state(state: &AppState, version: &str) -> Self {
        Self {
            version: version.to_owned(),
            selected: state.selected,
            game: state.games.game.clone(),
            minesweeper: state.games.minesweeper.clone(),
            sudoku: state.games.sudoku.clone(),
            nonogram: state.games.nonogram.clone(),
            solitaire: state.games.solitaire.clone(),
            freecell: state.games.freecell.clone(),
            fivefold: state.games.fivefold.clone(),
            reversi: state.games.reversi.clone(),
            lights_out: state.games.lights_out.clone(),
            tic_tac_toe: state.games.tic_tac_toe.clone(),
            memory_pairs: state.games.memory_pairs.clone(),
            sliding_puzzle: state.games.sliding_puzzle.clone(),
            mastermind: state.games.mastermind.clone(),
            spider: state.games.spider.clone(),
            word_search: state.games.word_search.clone(),
            hangman: state.games.hangman.clone(),
            connect_four: state.games.connect_four.clone(),
            checkers: state.games.checkers.clone(),
            peg_solitaire: state.games.peg_solitaire.clone(),
            mahjong_solitaire: state.games.mahjong_solitaire.clone(),
            snake: state.games.snake.clone(),
            breakout: state.games.breakout.clone(),
            higher_lower: state.games.higher_lower.clone(),
            klondike_golf: state.games.klondike_golf.clone(),
            blackjack: state.games.blackjack.clone(),
            spider_solitaire: state.games.spider_solitaire.clone(),
            dungeon_sweeper: state.games.dungeon_sweeper.clone(),
            potion_2048: state.games.potion_2048.clone(),
            tiny_tower_defence: state.games.tiny_tower_defence.clone(),
            one_room_roguelike: state.games.one_room_roguelike.clone(),
            daily_dungeon: state.games.daily_dungeon.clone(),
            dots_boxes: state.games.dots_boxes.clone(),
            sokoban: state.games.sokoban.clone(),
            mancala: state.games.mancala.clone(),
            hanoi: state.games.hanoi.clone(),
            number_match: state.games.number_match.clone(),
            flood_it: state.games.flood_it.clone(),
            color_sort: state.games.color_sort.clone(),
            battleship: state.games.battleship.clone(),
            word_grid: state.games.word_grid.clone(),
            pipe_loop: state.games.pipe_loop.clone(),
            maze_walk: state.games.maze_walk.clone(),
            match_three: state.games.match_three.clone(),
            pyramid: state.games.pyramid.clone(),
            tri_peaks: state.games.tri_peaks.clone(),
            nim: state.games.nim.clone(),
            word_ladder: state.games.word_ladder.clone(),
            space_invaders: state.games.space_invaders.clone(),
            asteroids: state.games.asteroids.clone(),
            frogger: state.games.frogger.clone(),
            munch_maze: state.games.munch_maze.clone(),
            block_stack: state.games.block_stack.clone(),
            terrain_cannon: state.games.terrain_cannon.clone(),
            fling_fury: state.games.fling_fury.clone(),
            paddle_duel: state.games.paddle_duel.clone(),
            riddle_room: state.games.riddle_room.clone(),
            pattern_vault: state.games.pattern_vault.clone(),
            sum_circuit: state.games.sum_circuit.clone(),
            orbit_order: state.games.orbit_order.clone(),
            word_forge: state.games.word_forge.clone(),
            profile_name: state.profile_name.clone(),
            sound: state.sound,
            reduced_motion: state.reduced_motion,
            high_contrast: state.high_contrast,
            large_text: state.large_text,
            mine_flag_mode: state.mine_flag_mode,
            mine_records: state.mine_records,
            sudoku_note_mode: state.sudoku_note_mode,
            records: state.records.clone(),
            achievements: state.achievements.clone(),
            stamps: state.stamps,
            card_back: state.card_back,
            board_theme: state.board_theme,
            sound_set: state.sound_set,
            cabinet_decoration: state.cabinet_decoration,
            tutorial_seen: state.tutorial_seen.clone(),
            favorites: state.favorites.clone(),
            recent_games: state.recent_games.clone(),
        }
    }
    pub fn apply_to(self, state: &mut AppState) {
        state.selected = self.selected.min(GameId::ALL.len().saturating_sub(1));
        state.games.game = self.game;
        state.games.minesweeper = self.minesweeper;
        state.games.sudoku = self.sudoku;
        state.games.nonogram = self.nonogram;
        state.games.solitaire = self.solitaire;
        state.games.freecell = self.freecell;
        state.games.fivefold = self.fivefold;
        state.games.reversi = self.reversi;
        state.games.lights_out = self.lights_out;
        state.games.tic_tac_toe = self.tic_tac_toe;
        state.games.memory_pairs = self.memory_pairs;
        state.games.sliding_puzzle = self.sliding_puzzle;
        state.games.mastermind = self.mastermind;
        state.games.spider = self.spider;
        state.games.word_search = self.word_search;
        state.games.hangman = self.hangman;
        state.games.connect_four = self.connect_four;
        state.games.checkers = self.checkers;
        state.games.peg_solitaire = self.peg_solitaire;
        state.games.mahjong_solitaire = self.mahjong_solitaire;
        state.games.snake = self.snake;
        state.games.breakout = self.breakout;
        state.games.higher_lower = self.higher_lower;
        state.games.klondike_golf = self.klondike_golf;
        state.games.blackjack = self.blackjack;
        state.games.spider_solitaire = self.spider_solitaire;
        state.games.dungeon_sweeper = self.dungeon_sweeper;
        state.games.potion_2048 = self.potion_2048;
        state.games.tiny_tower_defence = self.tiny_tower_defence;
        state.games.one_room_roguelike = self.one_room_roguelike;
        state.games.daily_dungeon = self.daily_dungeon;
        state.games.dots_boxes = self.dots_boxes;
        state.games.sokoban = self.sokoban;
        state.games.mancala = self.mancala;
        state.games.hanoi = self.hanoi;
        state.games.number_match = self.number_match;
        state.games.flood_it = self.flood_it;
        state.games.color_sort = self.color_sort;
        state.games.battleship = self.battleship;
        state.games.word_grid = self.word_grid;
        state.games.pipe_loop = self.pipe_loop;
        state.games.maze_walk = self.maze_walk;
        state.games.match_three = self.match_three;
        state.games.pyramid = self.pyramid;
        state.games.tri_peaks = self.tri_peaks;
        state.games.nim = self.nim;
        state.games.word_ladder = self.word_ladder;
        state.games.space_invaders = self.space_invaders;
        state.games.asteroids = self.asteroids;
        state.games.frogger = self.frogger;
        state.games.munch_maze = self.munch_maze;
        state.games.block_stack = self.block_stack;
        state.games.terrain_cannon = self.terrain_cannon;
        state.games.fling_fury = self.fling_fury;
        state.games.paddle_duel = self.paddle_duel;
        state.games.riddle_room = self.riddle_room;
        state.games.pattern_vault = self.pattern_vault;
        state.games.sum_circuit = self.sum_circuit;
        state.games.orbit_order = self.orbit_order;
        state.games.word_forge = self.word_forge;
        state.profile_name = self.profile_name;
        state.sound = self.sound;
        state.reduced_motion = self.reduced_motion;
        state.high_contrast = self.high_contrast;
        state.large_text = self.large_text;
        state.mine_flag_mode = self.mine_flag_mode;
        state.mine_records = self.mine_records;
        state.sudoku_note_mode = self.sudoku_note_mode;
        state.records = self.records;
        state.achievements = state_profile::normalize_achievements(self.achievements);
        state.stamps = self.stamps;
        state.card_back =
            crate::cosmetics::CosmeticKind::CardBack.normalize(self.card_back, state.stamps);
        state.board_theme =
            crate::cosmetics::CosmeticKind::BoardTheme.normalize(self.board_theme, state.stamps);
        state.sound_set =
            crate::cosmetics::CosmeticKind::SoundSet.normalize(self.sound_set, state.stamps);
        state.cabinet_decoration = crate::cosmetics::CosmeticKind::CabinetDecoration
            .normalize(self.cabinet_decoration, state.stamps);
        state.tutorial_seen = state_profile::normalize_tutorial_seen(self.tutorial_seen);
        state.favorites = state_profile::normalize_favorites(self.favorites);
        state.recent_games = state_profile::normalize_recent_games(self.recent_games);
    }
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            games: GameStore::default(),
            screen: Screen::Cabinet,
            selected: 4,
            achievements: vec![false; crate::progression::AchievementId::ALL.len()],
            stamps: 0,
            card_back: 0,
            board_theme: 0,
            sound_set: 0,
            cabinet_decoration: 0,
            confirm_restart: false,
            pending_restart: None,
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
            tutorial_seen: vec![false; GameId::ALL.len()],
            favorites: vec![false; GameId::ALL.len()],
            recent_games: Vec::new(),
            card_hint: None,
            favorites_view: false,
            recent_view: false,
            achievements_view: false,
            achievement_filter: 0,
            cabinet_filter: 0,
            cabinet_scroll: 0,
            library_scroll: 0,
            fivefold_score_page: 0,
        }
    }
}

#[cfg(test)]
mod tests;
