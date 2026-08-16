use super::*;

#[test]
fn default_solitaire_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!solitaire(&state).is_empty());
    assert_eq!(solitaire(&state), solitaire(&state));
}

#[test]
fn default_freecell_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!freecell(&state).is_empty());
    assert_eq!(freecell(&state), freecell(&state));
}

#[test]
fn default_pyramid_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!pyramid(&state).is_empty());
    assert_eq!(pyramid(&state), pyramid(&state));
}

#[test]
fn pyramid_hint_finds_an_exposed_pair() {
    let mut state = AppState::default();
    state.pyramid.pyramid = vec![None; 28];
    state.pyramid.pyramid[26] = Some(crate::cards::Card {
        rank: 5,
        suit: 0,
        face_up: true,
    });
    state.pyramid.pyramid[27] = Some(crate::cards::Card {
        rank: 8,
        suit: 1,
        face_up: true,
    });
    state.pyramid.stock.clear();
    assert_eq!(pyramid(&state), "Pair exposed cards 27 and 28.");
}

#[test]
fn default_tri_peaks_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!tri_peaks(&state).is_empty());
    assert_eq!(tri_peaks(&state), tri_peaks(&state));
}

#[test]
fn default_klondike_golf_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!klondike_golf(&state).is_empty());
    assert_eq!(klondike_golf(&state), klondike_golf(&state));
}

#[test]
fn default_spider_solitaire_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!spider_solitaire(&state).is_empty());
    assert_eq!(spider_solitaire(&state), spider_solitaire(&state));
}

#[test]
fn default_nim_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(nim(&state), "Select heap 1 and tap TAKE 3.");
    assert_eq!(nim(&state), nim(&state));
}

#[test]
fn default_2048_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!game_2048(&state).is_empty());
    assert_eq!(game_2048(&state), game_2048(&state));
}

#[test]
fn default_tic_tac_toe_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(tic_tac_toe(&state), "Try square 5.");
    assert_eq!(tic_tac_toe(&state), tic_tac_toe(&state));
}

#[test]
fn default_lights_out_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!lights_out(&state).is_empty());
    assert_eq!(lights_out(&state), lights_out(&state));
}

#[test]
fn default_memory_pairs_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(memory_pairs(&state), "Pair cards 1 and 12.");
    assert_eq!(memory_pairs(&state), memory_pairs(&state));
}

#[test]
fn default_sliding_puzzle_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!sliding_puzzle(&state).is_empty());
    assert_eq!(sliding_puzzle(&state), sliding_puzzle(&state));
}

#[test]
fn default_mastermind_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(mastermind(&state), "Try the red peg in slot 1.");
    assert_eq!(mastermind(&state), mastermind(&state));
}

#[test]
fn default_sudoku_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(sudoku(&state), "Enter 4 in row 1, column 3.");
    assert_eq!(sudoku(&state), sudoku(&state));
}

#[test]
fn default_minesweeper_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(minesweeper(&state), "Reveal row 5, column 5.");
    assert_eq!(minesweeper(&state), minesweeper(&state));
}

#[test]
fn default_nonogram_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(nonogram(&state), "Fill row 1, column 1.");
    assert_eq!(nonogram(&state), nonogram(&state));
}

#[test]
fn default_word_search_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(
        word_search(&state),
        "Try QUIET from row 1, column 1 to row 1, column 5."
    );
    assert_eq!(word_search(&state), word_search(&state));
}

#[test]
fn default_hangman_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!hangman(&state).is_empty());
    assert_eq!(hangman(&state), hangman(&state));
}

#[test]
fn default_connect_four_has_a_deterministic_hint() {
    let state = AppState::default();
    assert_eq!(connect_four(&state), "Drop a disc in column 4.");
    assert_eq!(connect_four(&state), connect_four(&state));
}

#[test]
fn default_checkers_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!checkers(&state).is_empty());
    assert_eq!(checkers(&state), checkers(&state));
}

#[test]
fn default_reversi_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!reversi(&state).is_empty());
    assert_eq!(reversi(&state), reversi(&state));
}

#[test]
fn default_peg_solitaire_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!peg_solitaire(&state).is_empty());
    assert_eq!(peg_solitaire(&state), peg_solitaire(&state));
}

#[test]
fn default_mahjong_solitaire_has_a_deterministic_hint() {
    let state = AppState::default();
    assert!(!mahjong_solitaire(&state).is_empty());
    assert_eq!(mahjong_solitaire(&state), mahjong_solitaire(&state));
}

#[test]
fn default_snake_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!snake(&state).is_empty());
    assert_eq!(snake(&state), snake(&state));
}

#[test]
fn default_breakout_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!breakout(&state).is_empty());
    assert_eq!(breakout(&state), breakout(&state));
}

#[test]
fn default_higher_lower_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!higher_lower(&state).is_empty());
    assert_eq!(higher_lower(&state), higher_lower(&state));
}

#[test]
fn default_blackjack_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!blackjack(&state).is_empty());
    assert_eq!(blackjack(&state), blackjack(&state));
}

#[test]
fn default_dungeon_sweeper_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!dungeon_sweeper(&state).is_empty());
    assert_eq!(dungeon_sweeper(&state), dungeon_sweeper(&state));
}

#[test]
fn default_potion_2048_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!potion_2048(&state).is_empty());
    assert_eq!(potion_2048(&state), potion_2048(&state));
}

#[test]
fn default_tiny_tower_defence_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!tiny_tower_defence(&state).is_empty());
    assert_eq!(tiny_tower_defence(&state), tiny_tower_defence(&state));
}

#[test]
fn default_one_room_roguelike_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!one_room_roguelike(&state).is_empty());
    assert_eq!(one_room_roguelike(&state), one_room_roguelike(&state));
}

#[test]
fn default_daily_dungeon_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!daily_dungeon(&state).is_empty());
    assert_eq!(daily_dungeon(&state), daily_dungeon(&state));
}

#[test]
fn default_dots_boxes_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(!dots_boxes(&state).is_empty());
    assert_eq!(dots_boxes(&state), dots_boxes(&state));
}

#[test]
fn default_sokoban_has_a_deterministic_hint() {
    let state = AppState::default();

    assert!(sokoban(&state).contains("Move UP"));
    assert_eq!(sokoban(&state), sokoban(&state));
}
