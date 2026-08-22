//! The single game-surface route used by both input and rendering.

use crate::state::{AppState, GameId, Screen};
use crate::ui::UiAction;
use macroquad::prelude::{Texture2D, Vec2};

pub fn clicks(state: &AppState, p: Vec2) -> Vec<UiAction> {
    if let Some(actions) = crate::game_result_ui::clicks(state, p) {
        return actions;
    }
    match state.screen {
        Screen::Game(GameId::Game2048) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape::game2048_clicks(state, p)
        }
        Screen::Game(GameId::Game2048) if crate::ui::is_portrait() => {
            crate::responsive_ui::game2048_clicks(state, p)
        }
        Screen::Game(GameId::Game2048) => crate::ui::game_clicks(state, p),
        Screen::Game(GameId::Minesweeper) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::minesweeper_clicks(state, p)
        }
        Screen::Game(GameId::Minesweeper) if crate::ui::is_portrait() => {
            crate::responsive_puzzles::minesweeper_clicks(state, p)
        }
        Screen::Game(GameId::Minesweeper) => crate::minesweeper_ui::clicks(state, p),
        Screen::Game(GameId::Sudoku) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::sudoku_clicks(state, p)
        }
        Screen::Game(GameId::Sudoku) if crate::ui::is_portrait() => {
            crate::responsive_sudoku::clicks(state, p)
        }
        Screen::Game(GameId::Sudoku) => crate::sudoku_ui::sudoku_clicks(state, p),
        Screen::Game(GameId::Nonogram) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::nonogram_clicks(state, p)
        }
        Screen::Game(GameId::Nonogram) if crate::ui::is_portrait() => {
            crate::responsive_puzzles::nonogram_clicks(state, p)
        }
        Screen::Game(GameId::Nonogram) => crate::nonogram_ui::nonogram_clicks(state, p),
        Screen::Game(GameId::Solitaire) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_cards::solitaire_clicks(state, p)
        }
        Screen::Game(GameId::Solitaire) if crate::ui::is_portrait() => {
            crate::responsive_cards::solitaire_clicks(state, p)
        }
        Screen::Game(GameId::Solitaire) => crate::solitaire_ui::solitaire_clicks(state, p),
        Screen::Game(GameId::FreeCell) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_cards::freecell_clicks(state, p)
        }
        Screen::Game(GameId::FreeCell) if crate::ui::is_portrait() => {
            crate::responsive_cards::freecell_clicks(state, p)
        }
        Screen::Game(GameId::FreeCell) => crate::freecell_ui::freecell_clicks(state, p),
        Screen::Game(GameId::Yahtzee) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_cards::fivefold_clicks(state, p)
        }
        Screen::Game(GameId::Yahtzee) if crate::ui::is_portrait() => {
            crate::responsive_fivefold::fivefold_clicks(state, p)
        }
        Screen::Game(GameId::Yahtzee) => crate::fivefold_ui::fivefold_clicks(state, p),
        Screen::Game(GameId::Reversi) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::reversi_clicks(state, p)
        }
        Screen::Game(GameId::Reversi) if crate::ui::is_portrait() => {
            crate::responsive_cards::reversi_clicks(state, p)
        }
        Screen::Game(GameId::Reversi) => crate::reversi_ui::reversi_clicks(state, p),
        Screen::Game(GameId::LightsOut) => crate::lights_out_ui::clicks(state, p),
        Screen::Game(GameId::TicTacToe) => crate::tic_tac_toe_ui::clicks(state, p),
        Screen::Game(GameId::MemoryPairs) => crate::memory_pairs_ui::clicks(state, p),
        Screen::Game(GameId::SlidingPuzzle) => crate::sliding_puzzle_ui::clicks(state, p),
        Screen::Game(GameId::Spider) => crate::spider_ui::clicks(state, p),
        Screen::Game(GameId::WordSearch) => crate::word_search_ui::clicks(state, p),
        Screen::Game(GameId::Hangman) => crate::hangman_ui::clicks(state, p),
        Screen::Game(GameId::ConnectFour) => crate::connect_four_ui::clicks(state, p),
        Screen::Game(GameId::Checkers) => crate::checkers_ui::clicks(state, p),
        Screen::Game(GameId::PegSolitaire) => crate::peg_solitaire_ui::clicks(state, p),
        Screen::Game(GameId::MahjongSolitaire) => crate::mahjong_solitaire_ui::clicks(state, p),
        Screen::Game(GameId::Snake) => crate::snake_ui::clicks(state, p),
        Screen::Game(GameId::Breakout) => crate::breakout_ui::clicks(state, p),
        Screen::Game(GameId::SpaceInvaders) => crate::space_invaders_ui::clicks(state, p),
        Screen::Game(GameId::Asteroids) => crate::asteroids_ui::clicks(state, p),
        Screen::Game(GameId::Frogger) => crate::frogger_ui::clicks(state, p),
        Screen::Game(GameId::MunchMaze) => crate::munch_maze_ui::clicks(state, p),
        Screen::Game(GameId::BlockStack) => crate::block_stack_ui::clicks(state, p),
        Screen::Game(GameId::TerrainCannon) => crate::terrain_cannon_ui::clicks(state, p),
        Screen::Game(GameId::FlingFury) => crate::fling_fury_ui::clicks(state, p),
        Screen::Game(GameId::PaddleDuel) => crate::paddle_duel_ui::clicks(state, p),
        Screen::Game(GameId::HigherLower) => crate::higher_lower_ui::clicks(state, p),
        Screen::Game(GameId::KlondikeGolf) => crate::klondike_golf_ui::clicks(state, p),
        Screen::Game(GameId::Blackjack) => crate::blackjack_ui::clicks(state, p),
        Screen::Game(GameId::SpiderSolitaire) => crate::spider_solitaire_ui::clicks(state, p),
        Screen::Game(GameId::Pyramid) => crate::pyramid_ui::clicks(state, p),
        Screen::Game(GameId::TriPeaks) => crate::tri_peaks_ui::clicks(state, p),
        Screen::Game(GameId::Nim) => crate::nim_ui::clicks(state, p),
        Screen::Game(GameId::DungeonSweeper) => crate::dungeon_sweeper_ui::clicks(state, p),
        Screen::Game(GameId::Potion2048) => crate::potion_2048_ui::clicks(state, p),
        Screen::Game(GameId::TinyTowerDefence) => crate::tiny_tower_defence_ui::clicks(state, p),
        Screen::Game(GameId::OneRoomRoguelike) => crate::one_room_roguelike_ui::clicks(state, p),
        Screen::Game(GameId::DailyDungeon) => crate::daily_dungeon_ui::clicks(state, p),
        Screen::Game(GameId::DotsBoxes) => crate::dots_boxes_ui::clicks(state, p),
        Screen::Game(GameId::Sokoban) => crate::sokoban_ui::clicks(state, p),
        Screen::Game(GameId::Mancala) => crate::mancala_ui::clicks(state, p),
        Screen::Game(GameId::Hanoi) => crate::hanoi_ui::clicks(state, p),
        Screen::Game(GameId::NumberMatch) => crate::number_match_ui::clicks(state, p),
        Screen::Game(GameId::FloodIt) => crate::flood_it_ui::clicks(state, p),
        Screen::Game(GameId::ColorSort) => crate::color_sort_ui::clicks(state, p),
        Screen::Game(GameId::Battleship) => crate::battleship_ui::clicks(state, p),
        Screen::Game(GameId::WordGrid) => crate::word_grid_ui::clicks(state, p),
        Screen::Game(GameId::WordLadder) => crate::word_ladder_ui::clicks(state, p),
        Screen::Game(GameId::PipeLoop) => crate::pipe_loop_ui::clicks(state, p),
        Screen::Game(GameId::MazeWalk) => crate::maze_walk_ui::clicks(state, p),
        Screen::Game(GameId::MatchThree) => crate::match_three_ui::clicks(state, p),
        Screen::Game(GameId::Mastermind) => crate::mastermind_ui::clicks(state, p),
        Screen::Game(GameId::RiddleRoom)
        | Screen::Game(GameId::PatternVault)
        | Screen::Game(GameId::SumCircuit)
        | Screen::Game(GameId::OrbitOrder)
        | Screen::Game(GameId::WordForge) => crate::misc_ui::clicks(state, p),
        _ => Vec::new(),
    }
}

pub fn draw(state: &AppState, frogger_frog: Option<&Texture2D>, frogger_car: Option<&Texture2D>) {
    match state.screen {
        Screen::Game(GameId::Game2048) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape::draw_2048(state)
        }
        Screen::Game(GameId::Game2048) if crate::ui::is_portrait() => {
            crate::responsive_ui::draw_2048(state)
        }
        Screen::Game(GameId::Game2048) => crate::ui::draw_2048(state),
        Screen::Game(GameId::Minesweeper) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::draw_minesweeper(state)
        }
        Screen::Game(GameId::Minesweeper) if crate::ui::is_portrait() => {
            crate::responsive_puzzles::draw_minesweeper(state)
        }
        Screen::Game(GameId::Minesweeper) => crate::minesweeper_ui::draw(state),
        Screen::Game(GameId::Sudoku) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::draw_sudoku(state)
        }
        Screen::Game(GameId::Sudoku) if crate::ui::is_portrait() => {
            crate::responsive_sudoku::draw(state)
        }
        Screen::Game(GameId::Sudoku) => crate::sudoku_ui::draw_sudoku(state),
        Screen::Game(GameId::Nonogram) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::draw_nonogram(state)
        }
        Screen::Game(GameId::Nonogram) if crate::ui::is_portrait() => {
            crate::responsive_puzzles::draw_nonogram(state)
        }
        Screen::Game(GameId::Nonogram) => crate::nonogram_ui::draw_nonogram(state),
        Screen::Game(GameId::Solitaire) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_cards::draw_solitaire(state)
        }
        Screen::Game(GameId::Solitaire) if crate::ui::is_portrait() => {
            crate::responsive_cards::draw_solitaire(state)
        }
        Screen::Game(GameId::Solitaire) => crate::solitaire_ui::draw_solitaire(state),
        Screen::Game(GameId::FreeCell) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_cards::draw_freecell(state)
        }
        Screen::Game(GameId::FreeCell) if crate::ui::is_portrait() => {
            crate::responsive_cards::draw_freecell(state)
        }
        Screen::Game(GameId::FreeCell) => crate::freecell_ui::draw_freecell(state),
        Screen::Game(GameId::Yahtzee) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_cards::draw_fivefold(state)
        }
        Screen::Game(GameId::Yahtzee) if crate::ui::is_portrait() => {
            crate::responsive_fivefold::draw_fivefold(state)
        }
        Screen::Game(GameId::Yahtzee) => crate::fivefold_ui::draw_fivefold(state),
        Screen::Game(GameId::Reversi) if crate::ui::is_compact_landscape() => {
            crate::responsive_landscape_games::draw_reversi(state)
        }
        Screen::Game(GameId::Reversi) if crate::ui::is_portrait() => {
            crate::responsive_cards::draw_reversi(state)
        }
        Screen::Game(GameId::Reversi) => crate::reversi_ui::draw_reversi(state),
        Screen::Game(GameId::LightsOut) => crate::lights_out_ui::draw(state),
        Screen::Game(GameId::TicTacToe) => crate::tic_tac_toe_ui::draw(state),
        Screen::Game(GameId::MemoryPairs) => crate::memory_pairs_ui::draw(state),
        Screen::Game(GameId::SlidingPuzzle) => crate::sliding_puzzle_ui::draw(state),
        Screen::Game(GameId::Spider) => crate::spider_ui::draw(state),
        Screen::Game(GameId::WordSearch) => crate::word_search_ui::draw(state),
        Screen::Game(GameId::Hangman) => crate::hangman_ui::draw(state),
        Screen::Game(GameId::ConnectFour) => crate::connect_four_ui::draw(state),
        Screen::Game(GameId::Checkers) => crate::checkers_ui::draw(state),
        Screen::Game(GameId::PegSolitaire) => crate::peg_solitaire_ui::draw(state),
        Screen::Game(GameId::MahjongSolitaire) => crate::mahjong_solitaire_ui::draw(state),
        Screen::Game(GameId::Snake) => crate::snake_ui::draw(state),
        Screen::Game(GameId::Breakout) => crate::breakout_ui::draw(state),
        Screen::Game(GameId::SpaceInvaders) => crate::space_invaders_ui::draw(state),
        Screen::Game(GameId::Asteroids) => crate::asteroids_ui::draw(state),
        Screen::Game(GameId::Frogger) => crate::frogger_ui::draw(state, frogger_frog, frogger_car),
        Screen::Game(GameId::MunchMaze) => crate::munch_maze_ui::draw(state),
        Screen::Game(GameId::BlockStack) => crate::block_stack_ui::draw(state),
        Screen::Game(GameId::TerrainCannon) => crate::terrain_cannon_ui::draw(state),
        Screen::Game(GameId::FlingFury) => crate::fling_fury_ui::draw(state),
        Screen::Game(GameId::PaddleDuel) => crate::paddle_duel_ui::draw(state),
        Screen::Game(GameId::HigherLower) => crate::higher_lower_ui::draw(state),
        Screen::Game(GameId::KlondikeGolf) => crate::klondike_golf_ui::draw(state),
        Screen::Game(GameId::Blackjack) => crate::blackjack_ui::draw(state),
        Screen::Game(GameId::SpiderSolitaire) => crate::spider_solitaire_ui::draw(state),
        Screen::Game(GameId::Pyramid) => crate::pyramid_ui::draw(state),
        Screen::Game(GameId::TriPeaks) => crate::tri_peaks_ui::draw(state),
        Screen::Game(GameId::Nim) => crate::nim_ui::draw(state),
        Screen::Game(GameId::DungeonSweeper) => crate::dungeon_sweeper_ui::draw(state),
        Screen::Game(GameId::Potion2048) => crate::potion_2048_ui::draw(state),
        Screen::Game(GameId::TinyTowerDefence) => crate::tiny_tower_defence_ui::draw(state),
        Screen::Game(GameId::OneRoomRoguelike) => crate::one_room_roguelike_ui::draw(state),
        Screen::Game(GameId::DailyDungeon) => crate::daily_dungeon_ui::draw(state),
        Screen::Game(GameId::DotsBoxes) => crate::dots_boxes_ui::draw(state),
        Screen::Game(GameId::Sokoban) => crate::sokoban_ui::draw(state),
        Screen::Game(GameId::Mancala) => crate::mancala_ui::draw(state),
        Screen::Game(GameId::Hanoi) => crate::hanoi_ui::draw(state),
        Screen::Game(GameId::NumberMatch) => crate::number_match_ui::draw(state),
        Screen::Game(GameId::FloodIt) => crate::flood_it_ui::draw(state),
        Screen::Game(GameId::ColorSort) => crate::color_sort_ui::draw(state),
        Screen::Game(GameId::Battleship) => crate::battleship_ui::draw(state),
        Screen::Game(GameId::WordGrid) => crate::word_grid_ui::draw(state),
        Screen::Game(GameId::WordLadder) => crate::word_ladder_ui::draw(state),
        Screen::Game(GameId::PipeLoop) => crate::pipe_loop_ui::draw(state),
        Screen::Game(GameId::MazeWalk) => crate::maze_walk_ui::draw(state),
        Screen::Game(GameId::MatchThree) => crate::match_three_ui::draw(state),
        Screen::Game(GameId::Mastermind) => crate::mastermind_ui::draw(state),
        Screen::Game(GameId::RiddleRoom)
        | Screen::Game(GameId::PatternVault)
        | Screen::Game(GameId::SumCircuit)
        | Screen::Game(GameId::OrbitOrder)
        | Screen::Game(GameId::WordForge) => crate::misc_ui::draw(state),
        _ => {}
    }
    crate::game_result_ui::draw(state);
}
