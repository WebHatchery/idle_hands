//! Regression coverage for the tests module.

use super::*;

#[test]
fn every_misc_game_is_deterministic() {
    for kind in [
        MiscKind::RiddleRoom,
        MiscKind::PatternVault,
        MiscKind::SumCircuit,
        MiscKind::OrbitOrder,
        MiscKind::WordForge,
    ] {
        let left = MiscGame::new(19, kind);
        let right = MiscGame::new(19, kind);
        assert_eq!(
            serde_json::to_string(&left).unwrap(),
            serde_json::to_string(&right).unwrap()
        );
    }
}

#[test]
fn riddle_room_can_be_solved_from_its_published_answer() {
    let mut game = MiscGame::new(7, MiscKind::RiddleRoom);
    for _ in 0..5 {
        let answer = game.answer;
        assert!(game.tap(answer));
    }
    assert!(game.won());
}

#[test]
fn pattern_vault_can_be_solved_from_its_published_answer() {
    let mut game = MiscGame::new(7, MiscKind::PatternVault);
    for _ in 0..5 {
        let answer = game.answer;
        assert!(game.tap(answer));
    }
    assert!(game.won());
}

#[test]
fn sum_circuit_keeps_generating_solvable_rounds() {
    let mut game = MiscGame::new(7, MiscKind::SumCircuit);
    for round in 0..300 {
        let previous = game.board.clone();
        for index in game.solution.clone() {
            assert!(game.tap(index));
        }
        assert!(game.submit());
        assert_eq!(game.round, round + 1);
        assert!(!game.won());
        assert_ne!(game.board, previous);
        assert!(game.selected.is_empty());
    }
}

#[test]
fn sum_circuit_does_not_require_a_selection_order() {
    let mut game = MiscGame::new(7, MiscKind::SumCircuit);
    for index in game.solution.clone().into_iter().rev() {
        game.tap(index);
    }
    assert!(game.submit());
    assert_eq!(game.round, 1);
}

#[test]
fn orbit_order_can_be_finished_with_swaps() {
    let mut game = MiscGame::new(7, MiscKind::OrbitOrder);
    for target in 1..=game.board.len() as u8 {
        let first = game
            .board
            .iter()
            .position(|value| *value == target)
            .unwrap();
        if first != usize::from(target - 1) {
            game.tap(usize::from(target - 1));
            game.tap(first);
        }
    }
    assert!(game.won());
}

#[test]
fn word_forge_continues_with_random_words_and_repeated_letters() {
    let mut game = MiscGame::new(7, MiscKind::WordForge);
    let mut words = std::collections::HashSet::new();
    for round in 0..300 {
        let previous = game.target_word.clone();
        words.insert(previous.clone());
        assert_ne!(game.board, previous.as_bytes());
        assert!(game.board.len() <= 5);
        for letter in previous.bytes() {
            let index = game
                .board
                .iter()
                .enumerate()
                .position(|(i, value)| *value == letter && !game.selected.contains(&i))
                .unwrap();
            assert!(game.tap(index));
        }
        assert!(game.submit());
        assert_eq!(game.round, round + 1);
        assert!(!game.won());
        assert_ne!(game.target_word, previous);
    }
    assert!(words.len() > 30);
    let saved = serde_json::to_string(&game).unwrap();
    let restored: MiscGame = serde_json::from_str(&saved).unwrap();
    assert_eq!(restored.round, 300);
}

#[test]
fn undo_restores_a_misc_move() {
    let mut game = MiscGame::new(7, MiscKind::RiddleRoom);
    let before = game.moves;
    game.tap(1);
    assert!(game.undo());
    assert_eq!(game.moves, before);
}

#[test]
fn patterns_vary_and_offer_four_distinct_answers() {
    let mut prompts = std::collections::HashSet::new();
    for seed in 0..100 {
        let game = MiscGame::new(seed, MiscKind::PatternVault);
        prompts.insert(game.prompt.clone());
        assert_eq!(
            game.options
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            4
        );
        assert_eq!(game.prompt.split('·').count(), 6);
    }
    assert!(prompts.len() > 80);
}

#[test]
fn orbit_boards_are_varied_and_need_more_than_four_swaps() {
    let mut boards = std::collections::HashSet::new();
    let mut lengths = std::collections::HashSet::new();
    for seed in 0..100 {
        let mut game = MiscGame::new(seed, MiscKind::OrbitOrder);
        boards.insert(game.board.clone());
        for i in 0..game.board.len() {
            let position = game
                .board
                .iter()
                .position(|v| usize::from(*v) == i + 1)
                .unwrap();
            if position != i {
                game.tap(i);
                game.tap(position);
            }
        }
        assert!(game.won());
        assert!(game.moves >= 5);
        lengths.insert(game.moves);
    }
    assert!(boards.len() > 90);
    assert!(lengths.len() > 1);
}

#[test]
fn sum_circuit_accepts_every_combination_with_the_right_total() {
    let mut alternatives = 0;
    for seed in 0..20 {
        let initial = MiscGame::new(seed, MiscKind::SumCircuit);
        for mask in 1..(1 << initial.board.len()) {
            let indices: Vec<usize> = (0..initial.board.len())
                .filter(|i| mask & (1 << i) != 0)
                .collect();
            let sum: u16 = indices.iter().map(|i| u16::from(initial.board[*i])).sum();
            if sum != initial.target {
                continue;
            }
            let mut game = initial.clone();
            for index in &indices {
                assert!(game.tap(*index));
            }
            assert!(game.submit());
            assert_eq!(game.round, 1);
            assert_eq!(game.mistakes, 0);
            assert_eq!(game.score, indices.len() as u16);
            if indices != initial.solution {
                alternatives += 1;
            }
        }
    }
    assert!(alternatives > 100);
}

#[test]
fn sum_wrong_total_is_recoverable_and_round_advance_can_be_undone() {
    let mut game = MiscGame::new(7, MiscKind::SumCircuit);
    let original = game.board.clone();
    let wrong = game
        .board
        .iter()
        .position(|v| u16::from(*v) != game.target)
        .unwrap();
    game.tap(wrong);
    game.submit();
    assert_eq!(game.round, 0);
    assert_eq!(game.mistakes, 1);
    assert_eq!(game.board, original);
    assert!(game.selected.is_empty());
    for index in game.solution.clone() {
        game.tap(index);
    }
    game.submit();
    let advanced = serde_json::to_string(&game).unwrap();
    assert!(game.undo());
    assert_eq!(game.round, 0);
    assert_eq!(game.board, original);
    game.submit();
    assert_eq!(serde_json::to_string(&game).unwrap(), advanced);
}
