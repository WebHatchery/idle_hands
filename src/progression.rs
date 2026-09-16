//! One-time cabinet achievements and the shared stamp total.

use crate::{
    content::{AchievementEntry, GameContent},
    state::{CollectionRecords, GameId},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementId {
    FirstFinish,
    Game(GameId),
    FullCabinet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AchievementProgress {
    pub current: usize,
    pub target: usize,
}

impl AchievementProgress {
    pub fn is_complete(self) -> bool {
        self.current >= self.target
    }
}

impl AchievementId {
    pub const ALL: [Self; 62] = [
        Self::FirstFinish,
        Self::Game(GameId::Solitaire),
        Self::Game(GameId::FreeCell),
        Self::Game(GameId::Sudoku),
        Self::Game(GameId::Minesweeper),
        Self::Game(GameId::Game2048),
        Self::Game(GameId::Nonogram),
        Self::Game(GameId::Yahtzee),
        Self::Game(GameId::Reversi),
        Self::Game(GameId::LightsOut),
        Self::Game(GameId::TicTacToe),
        Self::Game(GameId::MemoryPairs),
        Self::Game(GameId::SlidingPuzzle),
        Self::Game(GameId::Mastermind),
        Self::Game(GameId::Spider),
        Self::Game(GameId::WordSearch),
        Self::Game(GameId::Hangman),
        Self::Game(GameId::ConnectFour),
        Self::Game(GameId::Checkers),
        Self::Game(GameId::PegSolitaire),
        Self::Game(GameId::MahjongSolitaire),
        Self::Game(GameId::Snake),
        Self::Game(GameId::Breakout),
        Self::Game(GameId::HigherLower),
        Self::Game(GameId::KlondikeGolf),
        Self::Game(GameId::Blackjack),
        Self::Game(GameId::SpiderSolitaire),
        Self::Game(GameId::DungeonSweeper),
        Self::Game(GameId::Potion2048),
        Self::Game(GameId::TinyTowerDefence),
        Self::Game(GameId::OneRoomRoguelike),
        Self::Game(GameId::DailyDungeon),
        Self::Game(GameId::DotsBoxes),
        Self::Game(GameId::Sokoban),
        Self::Game(GameId::Mancala),
        Self::Game(GameId::Hanoi),
        Self::Game(GameId::NumberMatch),
        Self::Game(GameId::FloodIt),
        Self::Game(GameId::ColorSort),
        Self::Game(GameId::Battleship),
        Self::Game(GameId::WordGrid),
        Self::Game(GameId::PipeLoop),
        Self::Game(GameId::MazeWalk),
        Self::Game(GameId::MatchThree),
        Self::Game(GameId::Pyramid),
        Self::Game(GameId::TriPeaks),
        Self::Game(GameId::Nim),
        Self::Game(GameId::WordLadder),
        Self::Game(GameId::SpaceInvaders),
        Self::Game(GameId::Asteroids),
        Self::Game(GameId::Frogger),
        Self::Game(GameId::MunchMaze),
        Self::Game(GameId::BlockStack),
        Self::Game(GameId::TerrainCannon),
        Self::Game(GameId::FlingFury),
        Self::Game(GameId::PaddleDuel),
        Self::Game(GameId::RiddleRoom),
        Self::Game(GameId::PatternVault),
        Self::Game(GameId::SumCircuit),
        Self::Game(GameId::OrbitOrder),
        Self::Game(GameId::WordForge),
        Self::FullCabinet,
    ];

    fn content_id(self) -> String {
        match self {
            Self::FirstFinish => "first_finish".into(),
            Self::Game(game) => format!("game:{}", game.key()),
            Self::FullCabinet => "full_cabinet".into(),
        }
    }

    fn entry(self, content: &GameContent) -> Option<&AchievementEntry> {
        let id = self.content_id();
        content.achievements.iter().find(|entry| entry.id == id)
    }

    pub fn title_from(self, content: &GameContent) -> String {
        self.entry(content)
            .map(|entry| entry.title.clone())
            .unwrap_or_else(|| self.content_id())
    }

    pub fn stamp_value_from(self, content: &GameContent) -> u16 {
        self.entry(content).map_or(1, |entry| entry.stamp_value)
    }

    pub fn description_from(self, content: &GameContent) -> String {
        self.entry(content)
            .map(|entry| entry.description.clone())
            .unwrap_or_else(|| format!("Finish {}", self.title_from(content)))
    }

    pub fn progress(self, records: &CollectionRecords) -> AchievementProgress {
        match self {
            Self::FirstFinish => AchievementProgress {
                current: usize::from(completed_games(records) > 0),
                target: 1,
            },
            Self::Game(game) => AchievementProgress {
                current: usize::from(game_complete(records, game)),
                target: 1,
            },
            Self::FullCabinet => AchievementProgress {
                current: completed_games(records),
                target: GameId::ALL.len(),
            },
        }
    }

    pub fn progress_label(self, records: &CollectionRecords) -> String {
        let progress = self.progress(records);
        if progress.is_complete() {
            "COMPLETE".into()
        } else {
            format!("{} / {}", progress.current, progress.target)
        }
    }

    pub fn next_locked(records: &CollectionRecords) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|achievement| !earned(records, *achievement))
    }

    pub fn index(self) -> usize {
        // Every achievement is listed exactly once in the canonical order.
        Self::ALL
            .iter()
            .position(|candidate| *candidate == self)
            .unwrap()
    }
}

pub fn completed_games(records: &CollectionRecords) -> usize {
    GameId::ALL
        .into_iter()
        .filter(|&game| game_complete(records, game))
        .count()
}

pub(crate) fn game_complete(records: &CollectionRecords, game: GameId) -> bool {
    match game {
        GameId::Solitaire => records.solitaire_best_moves.is_some(),
        GameId::FreeCell => records.freecell_best_moves.is_some(),
        GameId::Sudoku => records.sudoku.iter().any(Option::is_some),
        GameId::Minesweeper => records.minesweeper.iter().any(Option::is_some),
        GameId::Game2048 => records.completed_2048(),
        GameId::Nonogram => records.nonogram.iter().any(Option::is_some),
        GameId::Yahtzee => records.fivefold_best_total > 0,
        GameId::Reversi => records.reversi_best_score > 0,
        GameId::LightsOut => records.lights_out_best_moves.is_some(),
        GameId::TicTacToe => records.tic_tac_toe_best_moves.is_some(),
        GameId::MemoryPairs => records.memory_pairs_best_moves.is_some(),
        GameId::SlidingPuzzle => records.sliding_puzzle_best_moves.is_some(),
        GameId::Mastermind => records.mastermind_best_rows.is_some(),
        GameId::Spider => records.spider_best_moves.is_some(),
        GameId::WordSearch => records.word_search_best_moves.is_some(),
        GameId::Hangman => records.hangman_best_moves.is_some(),
        GameId::ConnectFour => records.connect_four_best_moves.is_some(),
        GameId::Checkers => records.checkers_best_moves.is_some(),
        GameId::PegSolitaire => records.peg_solitaire_best_moves.is_some(),
        GameId::MahjongSolitaire => records.mahjong_solitaire_best_moves.is_some(),
        GameId::Snake => records.snake_best_score.is_some(),
        GameId::Breakout => records.breakout_best_score.is_some(),
        GameId::HigherLower => records.higher_lower_best_score.is_some(),
        GameId::KlondikeGolf => records.klondike_golf_best_moves.is_some(),
        GameId::Blackjack => records.blackjack_best_wins.is_some(),
        GameId::SpiderSolitaire => records.spider_solitaire_best_moves.is_some(),
        GameId::DungeonSweeper => records.dungeon_sweeper_best_moves.is_some(),
        GameId::Potion2048 => records.potion_2048_best_score.is_some(),
        GameId::TinyTowerDefence => records.tiny_tower_defence_best_wave.is_some(),
        GameId::OneRoomRoguelike => records.one_room_roguelike_best_score.is_some(),
        GameId::DailyDungeon => records.daily_dungeon_best_score.is_some(),
        GameId::DotsBoxes => records.dots_boxes_best_score.is_some(),
        GameId::Sokoban => records.sokoban_best_moves.is_some(),
        GameId::Mancala => records.mancala_best_score.is_some(),
        GameId::Hanoi => records.hanoi_best_moves.is_some(),
        GameId::NumberMatch => records.number_match_best_moves.is_some(),
        GameId::FloodIt => records.flood_it_best_moves.is_some(),
        GameId::ColorSort => records.color_sort_best_moves.is_some(),
        GameId::Battleship => records.battleship_best_moves.is_some(),
        GameId::WordGrid => records.word_grid_best_moves.is_some(),
        GameId::PipeLoop => records.pipe_loop_best_moves.is_some(),
        GameId::MazeWalk => records.maze_walk_best_moves.is_some(),
        GameId::MatchThree => records.match_three_best_score.is_some(),
        GameId::Pyramid => records.pyramid_best_moves.is_some(),
        GameId::TriPeaks => records.tri_peaks_best_moves.is_some(),
        GameId::Nim => records.nim_best_moves.is_some(),
        GameId::WordLadder => records.word_ladder_best_moves.is_some(),
        GameId::SpaceInvaders => records.space_invaders_best_score.is_some(),
        GameId::Asteroids => records.asteroids_best_score.is_some(),
        GameId::Frogger => records.frogger_best_score.is_some(),
        GameId::MunchMaze => records.munch_maze_best_score.is_some(),
        GameId::BlockStack => records.block_stack_best_score.is_some(),
        GameId::TerrainCannon => records.terrain_cannon_best_score.is_some(),
        GameId::FlingFury => records.fling_fury_best_score.is_some(),
        GameId::PaddleDuel => records.paddle_duel_best_score.is_some(),
        GameId::RiddleRoom => records.misc_best_moves[0].is_some(),
        GameId::PatternVault => records.misc_best_moves[1].is_some(),
        GameId::SumCircuit => records.misc_best_moves[2].is_some(),
        GameId::OrbitOrder => records.misc_best_moves[3].is_some(),
        GameId::WordForge => records.misc_best_moves[4].is_some(),
    }
}

pub fn earned(records: &CollectionRecords, achievement: AchievementId) -> bool {
    match achievement {
        AchievementId::FirstFinish => completed_games(records) > 0,
        AchievementId::Game(game) => game_complete(records, game),
        AchievementId::FullCabinet => completed_games(records) == GameId::ALL.len(),
    }
}

pub fn sync_with_content(
    earned_flags: &mut Vec<bool>,
    stamps: &mut u16,
    records: &CollectionRecords,
    content: &GameContent,
) -> Vec<AchievementId> {
    earned_flags.resize(AchievementId::ALL.len(), false);
    let mut newly_earned = Vec::new();
    for achievement in AchievementId::ALL {
        let index = achievement.index();
        if !earned_flags[index] && earned(records, achievement) {
            earned_flags[index] = true;
            *stamps = stamps.saturating_add(achievement.stamp_value_from(content));
            newly_earned.push(achievement);
        }
    }
    newly_earned
}

#[cfg(test)]
#[path = "../tests/legacy/progression/tests.rs"]
mod tests;
