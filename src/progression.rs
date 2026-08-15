//! One-time cabinet achievements and the shared stamp total.

use crate::state::{CollectionRecords, GameId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementId {
    FirstFinish,
    Game(GameId),
    FullCabinet,
}

impl AchievementId {
    pub const ALL: [Self; 10] = [
        Self::FirstFinish,
        Self::Game(GameId::Solitaire),
        Self::Game(GameId::FreeCell),
        Self::Game(GameId::Sudoku),
        Self::Game(GameId::Minesweeper),
        Self::Game(GameId::Game2048),
        Self::Game(GameId::Nonogram),
        Self::Game(GameId::Yahtzee),
        Self::Game(GameId::Reversi),
        Self::FullCabinet,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Self::FirstFinish => "First finish",
            Self::Game(game) => match game {
                GameId::Solitaire => "Card table",
                GameId::FreeCell => "Open cells",
                GameId::Sudoku => "Number keeper",
                GameId::Minesweeper => "Quiet surveyor",
                GameId::Game2048 => "Tile climber",
                GameId::Nonogram => "Pattern keeper",
                GameId::Yahtzee => "Scorekeeper",
                GameId::Reversi => "Board turner",
                GameId::LightsOut => "Light keeper",
                GameId::TicTacToe => "Three-in-a-row keeper",
                GameId::MemoryPairs => "Pair keeper",
                GameId::SlidingPuzzle => "Tile keeper",
                GameId::Mastermind => "Code keeper",
                GameId::Spider => "Web keeper",
                GameId::WordSearch => "Word keeper",
                GameId::Hangman => "Letter keeper",
                GameId::ConnectFour => "Disc keeper",
                GameId::Checkers => "Piece keeper",
                GameId::PegSolitaire => "Peg keeper",
                GameId::MahjongSolitaire => "Tile keeper",
                GameId::Snake => "Coil keeper",
                GameId::Breakout => "Brick keeper",
                GameId::HigherLower => "Card keeper",
                GameId::KlondikeGolf => "Golf keeper",
                GameId::Blackjack => "Hand keeper",
                GameId::SpiderSolitaire => "Web keeper",
                GameId::DungeonSweeper => "Dungeon keeper",
                GameId::Potion2048 => "Potion keeper",
                GameId::TinyTowerDefence => "Tower keeper",
                GameId::OneRoomRoguelike => "Room keeper",
                GameId::DailyDungeon => "Daily keeper",
                GameId::DotsBoxes => "Square keeper",
                GameId::Sokoban => "Crate keeper",
                GameId::Mancala => "Stone keeper",
                GameId::Hanoi => "Disk keeper",
                GameId::NumberMatch => "Number keeper",
                GameId::FloodIt => "Color keeper",
                GameId::ColorSort => "Sort keeper",
                GameId::Battleship => "Fleet keeper",
                GameId::WordGrid => "Word keeper",
                GameId::PipeLoop => "Pipe keeper",
            },
            Self::FullCabinet => "Full cabinet",
        }
    }

    pub fn stamp_value(self) -> u16 {
        match self {
            Self::FullCabinet => 2,
            _ => 1,
        }
    }

    pub fn index(self) -> usize {
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

fn game_complete(records: &CollectionRecords, game: GameId) -> bool {
    match game {
        GameId::Solitaire => records.solitaire_best_moves.is_some(),
        GameId::FreeCell => records.freecell_best_moves.is_some(),
        GameId::Sudoku => records.sudoku.iter().any(Option::is_some),
        GameId::Minesweeper => records.minesweeper.iter().any(Option::is_some),
        GameId::Game2048 => records.best_2048 >= 2048,
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
    }
}

pub fn earned(records: &CollectionRecords, achievement: AchievementId) -> bool {
    match achievement {
        AchievementId::FirstFinish => completed_games(records) > 0,
        AchievementId::Game(game) => game_complete(records, game),
        AchievementId::FullCabinet => completed_games(records) == GameId::ALL.len(),
    }
}

pub fn sync(
    earned_flags: &mut [bool; 10],
    stamps: &mut u16,
    records: &CollectionRecords,
) -> Vec<AchievementId> {
    let mut newly_earned = Vec::new();
    for achievement in AchievementId::ALL {
        let index = achievement.index();
        if !earned_flags[index] && earned(records, achievement) {
            earned_flags[index] = true;
            *stamps = stamps.saturating_add(achievement.stamp_value());
            newly_earned.push(achievement);
        }
    }
    newly_earned
}

#[cfg(test)]
mod tests;
