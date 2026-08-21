//! Desktop-only tap-route checks for every cabinet game.
//!
//! The scan stays inside the logical 1280×720 desktop surface and asks the
//! real game route for actions. It catches a screen whose visible controls no
//! longer produce an input action without coupling tests to drawing pixels.

use crate::state::{AppState, GameId, Screen};
use crate::ui::UiAction;
use macroquad::prelude::{vec2, Vec2};
use std::{collections::HashSet, mem::discriminant};

pub(super) fn run(game: GameId) {
    crate::ui::with_desktop_layout(|| {
        let state = AppState {
            screen: Screen::Game(game),
            selected: game.index(),
            tutorial: None,
            ..AppState::default()
        };

        let mut target = None;
        let mut action_kinds = HashSet::new();
        let mut has_recovery_action = false;
        for y in (80..=700).step_by(10) {
            for x in (160..=1260).step_by(10) {
                let actions = desktop_clicks(&state, game, vec2(x as f32 + 5., y as f32 + 5.));
                for action in &actions {
                    if !matches!(action, UiAction::Cabinet) {
                        action_kinds.insert(discriminant(action));
                        has_recovery_action |= super::support::is_recovery_action(action);
                    }
                }
                if !actions.is_empty()
                    && actions
                        .iter()
                        .any(|action| !matches!(action, UiAction::Cabinet))
                {
                    target = Some((x, y));
                }
            }
        }

        assert!(
            target.is_some(),
            "{} desktop surface did not expose a tap target",
            game.title()
        );
        assert!(
            action_kinds.len() >= 2,
            "{} desktop surface exposed fewer than two distinct game actions",
            game.title()
        );
        assert!(
            has_recovery_action,
            "{} desktop surface did not expose a visible recovery action",
            game.title()
        );
    });
}

fn desktop_clicks(state: &AppState, game: GameId, point: Vec2) -> Vec<UiAction> {
    match game {
        GameId::Solitaire => crate::solitaire_ui::solitaire_clicks(state, point),
        GameId::FreeCell => crate::freecell_ui::freecell_clicks(state, point),
        GameId::Sudoku => crate::sudoku_ui::sudoku_clicks(state, point),
        GameId::Minesweeper => crate::minesweeper_ui::clicks(state, point),
        GameId::Game2048 => crate::ui::game_clicks(state, point),
        GameId::Nonogram => crate::nonogram_ui::nonogram_clicks(state, point),
        GameId::Yahtzee => crate::fivefold_ui::fivefold_clicks(state, point),
        GameId::Reversi => crate::reversi_ui::reversi_clicks(state, point),
        GameId::LightsOut => crate::lights_out_ui::clicks(state, point),
        GameId::TicTacToe => crate::tic_tac_toe_ui::clicks(state, point),
        GameId::MemoryPairs => crate::memory_pairs_ui::clicks(state, point),
        GameId::SlidingPuzzle => crate::sliding_puzzle_ui::clicks(state, point),
        GameId::Mastermind => crate::mastermind_ui::clicks(state, point),
        GameId::Spider => crate::spider_ui::clicks(state, point),
        GameId::WordSearch => crate::word_search_ui::clicks(state, point),
        GameId::Hangman => crate::hangman_ui::clicks(state, point),
        GameId::ConnectFour => crate::connect_four_ui::clicks(state, point),
        GameId::Checkers => crate::checkers_ui::clicks(state, point),
        GameId::PegSolitaire => crate::peg_solitaire_ui::clicks(state, point),
        GameId::MahjongSolitaire => crate::mahjong_solitaire_ui::clicks(state, point),
        GameId::Snake => crate::snake_ui::clicks(state, point),
        GameId::Breakout => crate::breakout_ui::clicks(state, point),
        GameId::HigherLower => crate::higher_lower_ui::clicks(state, point),
        GameId::KlondikeGolf => crate::klondike_golf_ui::clicks(state, point),
        GameId::Blackjack => crate::blackjack_ui::clicks(state, point),
        GameId::SpiderSolitaire => crate::spider_solitaire_ui::clicks(state, point),
        GameId::DungeonSweeper => crate::dungeon_sweeper_ui::clicks(state, point),
        GameId::Potion2048 => crate::potion_2048_ui::clicks(state, point),
        GameId::TinyTowerDefence => crate::tiny_tower_defence_ui::clicks(state, point),
        GameId::OneRoomRoguelike => crate::one_room_roguelike_ui::clicks(state, point),
        GameId::DailyDungeon => crate::daily_dungeon_ui::clicks(state, point),
        GameId::DotsBoxes => crate::dots_boxes_ui::clicks(state, point),
        GameId::Sokoban => crate::sokoban_ui::clicks(state, point),
        GameId::Mancala => crate::mancala_ui::clicks(state, point),
        GameId::Hanoi => crate::hanoi_ui::clicks(state, point),
        GameId::NumberMatch => crate::number_match_ui::clicks(state, point),
        GameId::FloodIt => crate::flood_it_ui::clicks(state, point),
        GameId::ColorSort => crate::color_sort_ui::clicks(state, point),
        GameId::Battleship => crate::battleship_ui::clicks(state, point),
        GameId::WordGrid => crate::word_grid_ui::clicks(state, point),
        GameId::PipeLoop => crate::pipe_loop_ui::clicks(state, point),
        GameId::MazeWalk => crate::maze_walk_ui::clicks(state, point),
        GameId::MatchThree => crate::match_three_ui::clicks(state, point),
        GameId::Pyramid => crate::pyramid_ui::clicks(state, point),
        GameId::TriPeaks => crate::tri_peaks_ui::clicks(state, point),
        GameId::Nim => crate::nim_ui::clicks(state, point),
        GameId::WordLadder => crate::word_ladder_ui::clicks(state, point),
        GameId::SpaceInvaders => crate::space_invaders_ui::clicks(state, point),
        GameId::Asteroids => crate::asteroids_ui::clicks(state, point),
        GameId::Frogger => crate::frogger_ui::clicks(state, point),
        GameId::MunchMaze => crate::munch_maze_ui::clicks(state, point),
        GameId::BlockStack => crate::block_stack_ui::clicks(state, point),
        GameId::TerrainCannon => crate::terrain_cannon_ui::clicks(state, point),
        GameId::FlingFury => crate::fling_fury_ui::clicks(state, point),
        GameId::PaddleDuel => crate::paddle_duel_ui::clicks(state, point),
        GameId::RiddleRoom
        | GameId::PatternVault
        | GameId::SumCircuit
        | GameId::OrbitOrder
        | GameId::WordForge => crate::misc_ui::clicks(state, point),
    }
}
