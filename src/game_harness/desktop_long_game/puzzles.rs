//! Deterministic capture scenarios for the cabinet game suite.

use crate::domain::Direction;
use crate::game_2048::Game2048;
use crate::hangman::Hangman;
use crate::lights_out::LightsOut;
use crate::mastermind::Mastermind;
use crate::memory_pairs::MemoryPairs;
use crate::minesweeper::Minesweeper;
use crate::misc_games::{MiscGame, MiscKind};
use crate::nonogram::Nonogram;
use crate::sliding_puzzle::SlidingPuzzle;
use crate::state::GameId;
use crate::sudoku::Sudoku;
use crate::word_grid::WordGrid;
use crate::word_ladder::WordLadder;
use crate::word_search::WordSearch;

use super::{assert_serializable, SESSION_SEED};

pub(super) fn run(game: GameId) {
    match game {
        GameId::Sudoku => sudoku(),
        GameId::Minesweeper => minesweeper(),
        GameId::Game2048 => game_2048(),
        GameId::Nonogram => nonogram(),
        GameId::Yahtzee => fivefold(),
        GameId::LightsOut => lights_out(),
        GameId::Mastermind => mastermind(),
        GameId::MemoryPairs => memory_pairs(),
        GameId::SlidingPuzzle => sliding_puzzle(),
        GameId::Hangman => hangman(),
        GameId::WordSearch => word_search(),
        GameId::WordGrid => word_grid(),
        GameId::WordLadder => word_ladder(),
        GameId::RiddleRoom => misc_rounds(MiscKind::RiddleRoom),
        GameId::PatternVault => misc_rounds(MiscKind::PatternVault),
        GameId::SumCircuit => misc_rounds(MiscKind::SumCircuit),
        GameId::OrbitOrder => misc_rounds(MiscKind::OrbitOrder),
        GameId::WordForge => misc_rounds(MiscKind::WordForge),
        _ => panic!("puzzle session was assigned the wrong game"),
    }
}

fn sudoku() {
    let mut game =
        Sudoku::with_difficulty_and_seed(crate::sudoku::SudokuDifficulty::Medium, SESSION_SEED);
    let mut placements = 0;
    for round in 0..4 {
        while let Some((index, value)) = game.hint_move() {
            assert!(game.place(index, value));
            placements += 1;
        }
        game = Sudoku::with_difficulty_and_seed(
            crate::sudoku::SudokuDifficulty::Medium,
            SESSION_SEED + round + 1,
        );
    }
    assert!(
        placements >= 32,
        "Sudoku should keep accepting seeded hint placements"
    );
    assert_serializable(&game);
}

fn minesweeper() {
    let mut game = Minesweeper::beginner(SESSION_SEED);
    let mut actions = 0;
    for round in 0..8 {
        for _ in 0..96 {
            let Some((index, is_mine)) = game.hint_move() else {
                break;
            };
            let changed = if is_mine {
                game.toggle_flag(index)
            } else {
                game.reveal(index)
            };
            if changed {
                actions += 1;
            } else {
                break;
            }
        }
        game.tick(1.0);
        game = Minesweeper::beginner(SESSION_SEED + round + 1);
    }
    assert!(
        actions >= 24,
        "Minesweeper should survive repeated reveal and flag actions"
    );
    assert_serializable(&game);
}

fn game_2048() {
    let mut game = Game2048::new(SESSION_SEED);
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let mut moves = 0;
    for round in 0..8 {
        for direction in directions.into_iter().cycle().take(96) {
            if game.move_in(direction) {
                moves += 1;
            }
            if !game.can_move() {
                break;
            }
        }
        game = Game2048::new(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 64,
        "2048 should keep moving after the opening board"
    );
    assert_serializable(&game);
}

fn nonogram() {
    let mut game = Nonogram::new_with_seed(crate::nonogram::NonogramPreset::Medium, SESSION_SEED);
    let mut marks = 0;
    for round in 0..5 {
        while let Some((index, filled)) = game.hint_cell() {
            if !filled {
                game.toggle_mode();
            }
            assert!(game.toggle(index));
            if !filled {
                game.toggle_mode();
            }
            marks += 1;
        }
        game = Nonogram::new_with_seed(
            crate::nonogram::NonogramPreset::Medium,
            SESSION_SEED + round + 1,
        );
    }
    assert!(
        marks >= 64,
        "Nonogram should keep accepting fill and cross marks"
    );
    assert_serializable(&game);
}

fn fivefold() {
    let mut game = crate::fivefold::Fivefold::new(SESSION_SEED);
    let mut rounds = 0;
    for round in 0..16 {
        for _ in 0..3 {
            assert!(game.roll());
        }
        let category = game
            .hint_category()
            .unwrap_or(crate::fivefold::Category::Chance);
        assert!(game.choose_category(category));
        rounds += 1;
        game = crate::fivefold::Fivefold::new(SESSION_SEED + round + 1);
    }
    assert_eq!(rounds, 16);
    assert_serializable(&game);
}

fn lights_out() {
    let mut game = LightsOut::new(SESSION_SEED);
    let mut presses = 0;
    for round in 0..12 {
        while let Some(index) = game.hint_move() {
            assert!(game.press(index));
            presses += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        presses >= 24,
        "Lights Out should retain its solution path across rounds"
    );
    assert_serializable(&game);
}

fn mastermind() {
    let mut game = Mastermind::new(SESSION_SEED);
    let mut guesses = 0;
    for round in 0..12 {
        for _ in 0..8 {
            while let Some((_, color)) = game.hint_pick() {
                if !game.pick(color) {
                    break;
                }
            }
            if !game.submit() {
                break;
            }
            guesses += 1;
            if game.hint_pick().is_none() {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        guesses >= 12,
        "Mastermind should retain its repeated guess flow"
    );
    assert_serializable(&game);
}

fn memory_pairs() {
    let mut game = MemoryPairs::new(SESSION_SEED);
    let mut selections = 0;
    for round in 0..8 {
        for _ in 0..48 {
            let Some(first) = game.hint_choice() else {
                break;
            };
            if !game.select(first) {
                break;
            }
            selections += 1;
            let Some(second) = game.hint_choice() else {
                break;
            };
            assert!(game.select(second));
            selections += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        selections >= 64,
        "Memory should retain pair selection after many turns"
    );
    assert_serializable(&game);
}

fn sliding_puzzle() {
    let mut game = SlidingPuzzle::new(SESSION_SEED);
    let mut moves = 0;
    for round in 0..12 {
        for _ in 0..96 {
            let Some(index) = game.hint_move() else {
                break;
            };
            if game.move_tile(index) {
                moves += 1;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        moves >= 96,
        "Sliding Puzzle should keep accepting blank-neighbor moves"
    );
    assert_serializable(&game);
}

fn hangman() {
    let mut game = Hangman::new(SESSION_SEED);
    let mut guesses = 0;
    for round in 0..16 {
        for _ in 0..32 {
            let Some(letter) = game.hint_letter() else {
                break;
            };
            assert!(game.guess(letter));
            guesses += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        guesses >= 48,
        "Hangman should keep accepting hint-driven guesses"
    );
    assert_serializable(&game);
}

fn word_search() {
    let mut game = WordSearch::new(SESSION_SEED);
    let mut found = 0;
    for round in 0..8 {
        while let Some((_, start, end)) = game.hint_word() {
            assert!(game.select(start));
            assert!(game.select(end));
            found += 1;
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        found >= 24,
        "Word Search should retain its word endpoints late in a round"
    );
    assert_serializable(&game);
}

fn word_grid() {
    let mut game = WordGrid::new(SESSION_SEED);
    let mut submissions = 0;
    for round in 0..16 {
        for _ in 0..6 {
            let Some(word) = game.hint_word() else {
                break;
            };
            for letter in word.bytes() {
                assert!(game.tap_letter(letter - b'A'));
            }
            assert!(game.submit());
            submissions += 1;
            if game.hint_word().is_none() {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        submissions >= 32,
        "Word Grid should retain its keyboard-free entry path"
    );
    assert_serializable(&game);
}

fn word_ladder() {
    let mut game = WordLadder::new(SESSION_SEED);
    let mut steps = 0;
    for round in 0..12 {
        for _ in 0..12 {
            let Some(word) = game.hint_word() else {
                break;
            };
            for letter in word.bytes() {
                assert!(game.tap_letter(letter - b'A'));
            }
            assert!(game.submit());
            steps += 1;
            if game.hint_word().is_none() {
                break;
            }
        }
        game.reset(SESSION_SEED + round + 1);
    }
    assert!(
        steps >= 12,
        "Word Ladder should retain its multi-step route"
    );
    assert_serializable(&game);
}

fn misc_rounds(kind: MiscKind) {
    let mut game = MiscGame::new(SESSION_SEED, kind);
    let mut rounds = 0;
    for round in 0..8 {
        while !game.won() {
            match kind {
                MiscKind::RiddleRoom | MiscKind::PatternVault => {
                    let answer = game.answer;
                    assert!(game.tap(answer));
                }
                MiscKind::SumCircuit => {
                    for index in game.solution.clone() {
                        assert!(game.tap(index));
                    }
                    assert!(game.submit());
                }
                MiscKind::OrbitOrder => {
                    let Some(first) = game.board.iter().position(|value| {
                        *value
                            != (game
                                .board
                                .iter()
                                .position(|candidate| candidate == value)
                                .unwrap()
                                + 1) as u8
                    }) else {
                        break;
                    };
                    let target = game.board[first] as usize - 1;
                    assert!(game.tap(first));
                    assert!(game.tap(target));
                }
                MiscKind::WordForge => {
                    let mut used = Vec::new();
                    let target_word = game.target_word.clone();
                    for letter in target_word.bytes() {
                        let index = game
                            .board
                            .iter()
                            .enumerate()
                            .find(|(index, value)| **value == letter && !used.contains(index))
                            .map(|(index, _)| index)
                            .expect("word forge target must be on its board");
                        used.push(index);
                        assert!(game.tap(index));
                    }
                    assert!(game.submit());
                }
            }
            if game.moves > 128 {
                break;
            }
        }
        rounds += 1;
        game.reset(SESSION_SEED + round + 1);
    }
    assert_eq!(rounds, 8);
    assert_serializable(&game);
}
