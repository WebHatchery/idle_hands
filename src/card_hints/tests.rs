//! Table-driven regression coverage for the cabinet hint contract.

use crate::state::AppState;

use super::*;

type HintCase = (&'static str, fn(&AppState) -> String, &'static str);

#[test]
fn every_default_hint_is_non_empty_deterministic_and_descriptive() {
    let state = AppState::default();
    let cases: &[HintCase] = &[
        ("solitaire", solitaire, ""),
        ("freecell", freecell, ""),
        ("pyramid", pyramid, ""),
        ("tri_peaks", tri_peaks, ""),
        ("klondike_golf", klondike_golf, ""),
        ("spider_solitaire", spider_solitaire, ""),
        ("nim", nim, "forced-win route"),
        ("game_2048", game_2048, ""),
        ("tic_tac_toe", tic_tac_toe, "Try square 5."),
        ("lights_out", lights_out, ""),
        ("memory_pairs", memory_pairs, "No known pair yet"),
        ("sliding_puzzle", sliding_puzzle, ""),
        ("mastermind", mastermind, "Try the red peg"),
        ("sudoku", sudoku, "Enter "),
        ("minesweeper", minesweeper, "Reveal row 5"),
        ("nonogram", nonogram, "Fill row"),
        ("word_search", word_search, "Try STILL"),
        ("hangman", hangman, ""),
        ("connect_four", connect_four, "Drop a disc"),
        ("checkers", checkers, ""),
        ("reversi", reversi, ""),
        ("peg_solitaire", peg_solitaire, ""),
        ("mahjong_solitaire", mahjong_solitaire, ""),
        ("snake", snake, ""),
        ("breakout", breakout, ""),
        ("higher_lower", higher_lower, ""),
        ("blackjack", blackjack, ""),
        ("dungeon_sweeper", dungeon_sweeper, ""),
        ("potion_2048", potion_2048, ""),
        ("tiny_tower_defence", tiny_tower_defence, ""),
        ("one_room_roguelike", one_room_roguelike, ""),
        ("daily_dungeon", daily_dungeon, ""),
        ("dots_boxes", dots_boxes, ""),
        ("sokoban", sokoban, "Move UP"),
        ("mancala", mancala, ""),
        ("hanoi", hanoi, "peg"),
        ("number_match", number_match, "Pair cells"),
        ("flood_it", flood_it, "Choose"),
        ("color_sort", color_sort, "Pour"),
        ("battleship", battleship, "Sweep near"),
        ("word_grid", word_grid, "candidates remain"),
        ("word_ladder", word_ladder, "toward the target"),
        ("pipe_loop", pipe_loop, "Rotate tile"),
        ("maze_walk", maze_walk, "Walk"),
        ("fivefold", fivefold, "Roll DICE"),
        ("spider", spider, ""),
        ("match_three", match_three, "Swap tiles"),
    ];

    for (name, hint, expected) in cases {
        let first = hint(&state);
        assert!(!first.is_empty(), "{name} returned an empty hint");
        assert_eq!(
            first,
            hint(&state),
            "{name} changed between identical reads"
        );
        if !expected.is_empty() {
            assert!(
                first.contains(expected),
                "{name} hint {first:?} did not contain {expected:?}"
            );
        }
    }
}

#[test]
fn freecell_hint_finds_an_ordinary_cascade_move() {
    let mut state = AppState::default();
    state.games.freecell.cells = [None; 4];
    state.games.freecell.foundations = [0; 4];
    state.games.freecell.cascades = vec![
        vec![crate::cards::Card {
            rank: 7,
            suit: 0,
            face_up: true,
        }],
        vec![crate::cards::Card {
            rank: 8,
            suit: 1,
            face_up: true,
        }],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    assert_eq!(
        freecell(&state),
        "Try the top card in cascade 1 on cascade 2."
    );
}

#[test]
fn pyramid_hint_finds_an_exposed_pair() {
    let mut state = AppState::default();
    state.games.pyramid.pyramid = vec![None; 28];
    state.games.pyramid.pyramid[26] = Some(crate::cards::Card {
        rank: 5,
        suit: 0,
        face_up: true,
    });
    state.games.pyramid.pyramid[27] = Some(crate::cards::Card {
        rank: 8,
        suit: 1,
        face_up: true,
    });
    state.games.pyramid.stock.clear();

    assert_eq!(pyramid(&state), "Pair exposed cards 27 and 28.");
}
