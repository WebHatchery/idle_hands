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
fn sum_circuit_requires_the_target_set() {
    let mut game = MiscGame::new(7, MiscKind::SumCircuit);
    for _ in 0..4 {
        for index in game.solution.clone() {
            game.tap(index);
        }
        assert!(game.submit());
    }
    assert!(game.won());
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
    for target in 1..=5_u8 {
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
fn word_forge_accepts_the_target_order() {
    let mut game = MiscGame::new(7, MiscKind::WordForge);
    for _ in 0..5 {
        for letter in game.target_word.clone().bytes() {
            let index = game
                .board
                .iter()
                .position(|value| *value == letter)
                .unwrap();
            game.tap(index);
        }
        assert!(game.submit());
    }
    assert!(game.won());
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
