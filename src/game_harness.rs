//! Host-level contract suites for every game in the cabinet.
//!
//! The game modules keep their detailed rule tests beside their state types.
//! These suites cover the shared boundary that every drawer must cross:
//! deterministic construction, catalog metadata, variant cycling, and
//! independent snapshot restore.

mod asteroids;
mod battleship;
mod blackjack;
mod block_stack;
mod breakout;
mod checkers;
mod color_sort;
mod connect_four;
mod daily_dungeon;
mod dots_boxes;
mod dungeon_sweeper;
mod fivefold;
mod fling_fury;
mod flood_it;
mod freecell;
mod frogger;
mod game_2048;
mod hangman;
mod hanoi;
mod higher_lower;
mod klondike_golf;
mod lights_out;
mod mahjong_solitaire;
mod mancala;
mod mastermind;
mod match_three;
mod maze_walk;
mod memory_pairs;
mod minesweeper;
mod munch_maze;
mod nim;
mod nonogram;
mod number_match;
mod one_room_roguelike;
mod orbit_order;
mod paddle_duel;
mod pattern_vault;
mod peg_solitaire;
mod pipe_loop;
mod potion_2048;
mod pyramid;
mod reversi;
mod riddle_room;
mod sliding_puzzle;
mod snake;
mod sokoban;
mod solitaire;
mod space_invaders;
mod spider;
mod spider_solitaire;
mod sudoku;
mod sum_circuit;
mod terrain_cannon;
mod tic_tac_toe;
mod tiny_tower_defence;
mod tri_peaks;
mod word_forge;
mod word_grid;
mod word_ladder;
mod word_search;

mod support;

use crate::state::GameId;

const SUITES: [(GameId, &str); 60] = [
    (solitaire::GAME, "solitaire"),
    (freecell::GAME, "freecell"),
    (sudoku::GAME, "sudoku"),
    (minesweeper::GAME, "minesweeper"),
    (game_2048::GAME, "game_2048"),
    (nonogram::GAME, "nonogram"),
    (fivefold::GAME, "fivefold"),
    (reversi::GAME, "reversi"),
    (lights_out::GAME, "lights_out"),
    (tic_tac_toe::GAME, "tic_tac_toe"),
    (memory_pairs::GAME, "memory_pairs"),
    (sliding_puzzle::GAME, "sliding_puzzle"),
    (mastermind::GAME, "mastermind"),
    (spider::GAME, "spider"),
    (word_search::GAME, "word_search"),
    (hangman::GAME, "hangman"),
    (connect_four::GAME, "connect_four"),
    (checkers::GAME, "checkers"),
    (peg_solitaire::GAME, "peg_solitaire"),
    (mahjong_solitaire::GAME, "mahjong_solitaire"),
    (snake::GAME, "snake"),
    (breakout::GAME, "breakout"),
    (higher_lower::GAME, "higher_lower"),
    (klondike_golf::GAME, "klondike_golf"),
    (blackjack::GAME, "blackjack"),
    (spider_solitaire::GAME, "spider_solitaire"),
    (dungeon_sweeper::GAME, "dungeon_sweeper"),
    (potion_2048::GAME, "potion_2048"),
    (tiny_tower_defence::GAME, "tiny_tower_defence"),
    (one_room_roguelike::GAME, "one_room_roguelike"),
    (daily_dungeon::GAME, "daily_dungeon"),
    (dots_boxes::GAME, "dots_boxes"),
    (sokoban::GAME, "sokoban"),
    (mancala::GAME, "mancala"),
    (hanoi::GAME, "hanoi"),
    (number_match::GAME, "number_match"),
    (flood_it::GAME, "flood_it"),
    (color_sort::GAME, "color_sort"),
    (battleship::GAME, "battleship"),
    (word_grid::GAME, "word_grid"),
    (pipe_loop::GAME, "pipe_loop"),
    (maze_walk::GAME, "maze_walk"),
    (match_three::GAME, "match_three"),
    (pyramid::GAME, "pyramid"),
    (tri_peaks::GAME, "tri_peaks"),
    (nim::GAME, "nim"),
    (word_ladder::GAME, "word_ladder"),
    (space_invaders::GAME, "space_invaders"),
    (asteroids::GAME, "asteroids"),
    (frogger::GAME, "frogger"),
    (munch_maze::GAME, "munch_maze"),
    (block_stack::GAME, "block_stack"),
    (terrain_cannon::GAME, "terrain_cannon"),
    (fling_fury::GAME, "fling_fury"),
    (paddle_duel::GAME, "paddle_duel"),
    (riddle_room::GAME, "riddle_room"),
    (pattern_vault::GAME, "pattern_vault"),
    (sum_circuit::GAME, "sum_circuit"),
    (orbit_order::GAME, "orbit_order"),
    (word_forge::GAME, "word_forge"),
];

#[test]
fn suite_registry_is_in_lockstep_with_the_game_catalog() {
    assert_eq!(SUITES.len(), GameId::ALL.len());
    for (index, ((suite_game, suite_name), catalog_game)) in
        SUITES.iter().zip(GameId::ALL).enumerate()
    {
        assert_eq!(
            *suite_game, catalog_game,
            "suite {suite_name} is out of order"
        );
        assert_eq!(suite_game.index(), index);
        assert!(!suite_name.is_empty());
    }
}
