//! Host-level contract suites for every game in the cabinet.
//!
//! Each long-session case counts against that game's five-case rule budget.
//! Shared host and input contracts iterate the catalog as table inputs.

#[path = "game_harness/asteroids.rs"]
mod asteroids;
#[path = "game_harness/battleship.rs"]
mod battleship;
#[path = "game_harness/blackjack.rs"]
mod blackjack;
#[path = "game_harness/block_stack.rs"]
mod block_stack;
#[path = "game_harness/breakout.rs"]
mod breakout;
#[path = "game_harness/checkers.rs"]
mod checkers;
#[path = "game_harness/color_sort.rs"]
mod color_sort;
#[path = "game_harness/connect_four.rs"]
mod connect_four;
#[path = "game_harness/daily_dungeon.rs"]
mod daily_dungeon;
#[path = "game_harness/dots_boxes.rs"]
mod dots_boxes;
#[path = "game_harness/dungeon_sweeper.rs"]
mod dungeon_sweeper;
#[path = "game_harness/fivefold.rs"]
mod fivefold;
#[path = "game_harness/fling_fury.rs"]
mod fling_fury;
#[path = "game_harness/flood_it.rs"]
mod flood_it;
#[path = "game_harness/freecell.rs"]
mod freecell;
#[path = "game_harness/frogger.rs"]
mod frogger;
#[path = "game_harness/game_2048.rs"]
mod game_2048;
#[path = "game_harness/hangman.rs"]
mod hangman;
#[path = "game_harness/hanoi.rs"]
mod hanoi;
#[path = "game_harness/higher_lower.rs"]
mod higher_lower;
#[path = "game_harness/klondike_golf.rs"]
mod klondike_golf;
#[path = "game_harness/lights_out.rs"]
mod lights_out;
#[path = "game_harness/mahjong_solitaire.rs"]
mod mahjong_solitaire;
#[path = "game_harness/mancala.rs"]
mod mancala;
#[path = "game_harness/mastermind.rs"]
mod mastermind;
#[path = "game_harness/match_three.rs"]
mod match_three;
#[path = "game_harness/maze_walk.rs"]
mod maze_walk;
#[path = "game_harness/memory_pairs.rs"]
mod memory_pairs;
#[path = "game_harness/minesweeper.rs"]
mod minesweeper;
#[path = "game_harness/munch_maze.rs"]
mod munch_maze;
#[path = "game_harness/nim.rs"]
mod nim;
#[path = "game_harness/nonogram.rs"]
mod nonogram;
#[path = "game_harness/number_match.rs"]
mod number_match;
#[path = "game_harness/one_room_roguelike.rs"]
mod one_room_roguelike;
#[path = "game_harness/orbit_order.rs"]
mod orbit_order;
#[path = "game_harness/paddle_duel.rs"]
mod paddle_duel;
#[path = "game_harness/pattern_vault.rs"]
mod pattern_vault;
#[path = "game_harness/peg_solitaire.rs"]
mod peg_solitaire;
#[path = "game_harness/pipe_loop.rs"]
mod pipe_loop;
#[path = "game_harness/potion_2048.rs"]
mod potion_2048;
#[path = "game_harness/pyramid.rs"]
mod pyramid;
#[path = "game_harness/reversi.rs"]
mod reversi;
#[path = "game_harness/riddle_room.rs"]
mod riddle_room;
#[path = "game_harness/sliding_puzzle.rs"]
mod sliding_puzzle;
#[path = "game_harness/snake.rs"]
mod snake;
#[path = "game_harness/sokoban.rs"]
mod sokoban;
#[path = "game_harness/solitaire.rs"]
mod solitaire;
#[path = "game_harness/space_invaders.rs"]
mod space_invaders;
#[path = "game_harness/spider.rs"]
mod spider;
#[path = "game_harness/spider_solitaire.rs"]
mod spider_solitaire;
#[path = "game_harness/sudoku.rs"]
mod sudoku;
#[path = "game_harness/sum_circuit.rs"]
mod sum_circuit;
#[path = "game_harness/terrain_cannon.rs"]
mod terrain_cannon;
#[path = "game_harness/tic_tac_toe.rs"]
mod tic_tac_toe;
#[path = "game_harness/tiny_tower_defence.rs"]
mod tiny_tower_defence;
#[path = "game_harness/tri_peaks.rs"]
mod tri_peaks;
#[path = "game_harness/word_forge.rs"]
mod word_forge;
#[path = "game_harness/word_grid.rs"]
mod word_grid;
#[path = "game_harness/word_ladder.rs"]
mod word_ladder;
#[path = "game_harness/word_search.rs"]
mod word_search;

#[path = "game_harness/desktop_long_game.rs"]
mod desktop_long_game;
#[path = "game_harness/desktop_ui.rs"]
mod desktop_ui;
#[path = "game_harness/host_contract.rs"]
mod host_contract;
#[path = "game_harness/mobile_ui.rs"]
mod mobile_ui;
#[path = "game_harness/support.rs"]
mod support;

use idle_hands::testing::GameId;

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
