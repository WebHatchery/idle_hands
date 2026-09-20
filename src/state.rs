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
use std::sync::Arc;

#[path = "state_game_id.rs"]
mod state_game_id;
#[path = "state_initialization.rs"]
mod state_initialization;
pub use state_game_id::GameId;
#[path = "state_profile.rs"]
mod state_profile;
#[path = "state_save.rs"]
mod state_save;
pub use state_profile::ProfileSave;
pub use state_save::CollectionSave;

pub use crate::state_navigation::Screen;
pub use crate::state_records::CollectionRecords;

pub use crate::game_2048::Game2048;

pub use crate::state_snapshots::GameSnapshot;

impl AppState {
    pub fn game_title(&self, game: GameId) -> &str {
        self.content
            .game(game)
            .map_or_else(|| game.title(), |entry| entry.title.as_str())
    }

    pub fn game_subtitle(&self, game: GameId) -> &str {
        self.content
            .game(game)
            .map_or_else(|| game.subtitle(), |entry| entry.subtitle.as_str())
    }

    pub fn game_save_key(&self, game: GameId) -> &str {
        self.content
            .game(game)
            .map_or_else(|| game.save_key(), |entry| entry.save_key.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub content: Arc<crate::content::GameContent>,
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
    pub game_setup_open: bool,
    pub confirm_reset: bool,
    pub profile_name: String,
    pub sound: bool,
    pub sound_level: u8,
    pub lifecycle_paused: bool,
    pub save_recovery: Option<crate::save_recovery::SaveRecoveryNotice>,
    pub notice_log_view: bool,
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
    pub daily_archive_view: bool,
    pub achievements_view: bool,
    pub tutorial_filter: bool,
    pub achievement_filter: u8,
    pub records_filter: u8,
    pub rules_filter: u8,
    pub cabinet_filter: u8,
    pub cabinet_sort: u8,
    pub cabinet_scroll: usize,
    pub library_scroll: usize,
    pub daily_archive_scroll: usize,
    pub fivefold_score_page: usize,
}

impl Default for AppState {
    fn default() -> Self {
        let content = crate::data::GameData::default_content();
        let profile_name = crate::profile_data::name(&content, 0).to_owned();
        Self {
            content,
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
            game_setup_open: false,
            confirm_reset: false,
            profile_name,
            sound: true,
            sound_level: crate::audio_settings::DEFAULT_LEVEL,
            lifecycle_paused: false,
            save_recovery: None,
            notice_log_view: false,
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
            daily_archive_view: false,
            achievements_view: false,
            tutorial_filter: false,
            achievement_filter: 0,
            records_filter: 0,
            rules_filter: 0,
            cabinet_filter: 0,
            cabinet_sort: 0,
            cabinet_scroll: 0,
            library_scroll: 0,
            daily_archive_scroll: 0,
            fivefold_score_page: 0,
        }
    }
}

#[cfg(test)]
#[path = "../tests/legacy/state/tests.rs"]
mod tests;
