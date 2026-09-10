use super::*;

#[test]
fn cabinet_filters_keep_new_drawers_open_and_completed_drawers_done() {
    let mut state = AppState::default();
    assert!(matches_filter(&state, GameId::Game2048, 0));
    assert!(matches_filter(&state, GameId::Game2048, 1));
    assert!(!matches_filter(&state, GameId::Game2048, 2));

    state.records.best_2048 = 2048;
    assert!(matches_filter(&state, GameId::Game2048, 0));
    assert!(!matches_filter(&state, GameId::Game2048, 1));
    assert!(matches_filter(&state, GameId::Game2048, 2));
}

#[test]
fn cabinet_filter_unknown_values_show_all_games() {
    let state = AppState::default();
    assert!(matches_filter(&state, GameId::Solitaire, 99));
}

#[test]
fn category_filters_partition_the_whole_collection() {
    let state = AppState::default();
    let total: usize = CATEGORY_FILTERS
        .iter()
        .map(|filter| filter_count(&state, *filter))
        .sum();
    assert_eq!(total, GameId::ALL.len());
    assert!(matches_filter(&state, GameId::Solitaire, 3));
    assert!(matches_filter(&state, GameId::Sudoku, 4));
    assert!(matches_filter(&state, GameId::WordSearch, 6));
    assert!(matches_filter(&state, GameId::Snake, 7));
    assert!(matches_filter(&state, GameId::Breakout, 7));
    assert!(matches_filter(&state, GameId::TinyTowerDefence, 7));
    assert!(!matches_filter(&state, GameId::DungeonSweeper, 7));
    assert!(!matches_filter(&state, GameId::Battleship, 7));
    assert!(!matches_filter(&state, GameId::OneRoomRoguelike, 7));
    assert!(matches_filter(&state, GameId::DungeonSweeper, 4));
    assert!(matches_filter(&state, GameId::Battleship, 5));
    assert!(matches_filter(&state, GameId::OneRoomRoguelike, 8));
    assert_eq!(filter_count(&state, 3), 11);
    assert_eq!(filter_count(&state, 4), 10);
    assert_eq!(filter_count(&state, 5), 9);
    assert_eq!(filter_count(&state, 6), 7);
    assert_eq!(filter_count(&state, 7), 11);
    assert_eq!(filter_count(&state, 8), 12);
}

#[test]
fn category_progress_counts_finished_drawers_without_losing_the_total() {
    let mut state = AppState::default();
    assert_eq!(
        category_progress(&state, 3),
        CategoryProgress {
            completed: 0,
            total: 11,
        }
    );

    state.records.solitaire_best_moves = Some(42);
    state.records.freecell_best_moves = Some(31);
    assert_eq!(
        category_progress(&state, 3),
        CategoryProgress {
            completed: 2,
            total: 11,
        }
    );
    assert_eq!(category_progress(&state, 3).remaining(), 9);
    assert!(!category_progress(&state, 3).is_complete());
}

#[test]
fn category_progress_ignores_drawers_from_other_categories() {
    let mut state = AppState::default();
    state.records.solitaire_best_moves = Some(42);
    state.records.sudoku[0] = Some(12);

    assert_eq!(category_progress(&state, 3).completed, 1);
    assert_eq!(category_progress(&state, 4).completed, 1);
}

#[test]
fn collection_progress_spans_every_drawer() {
    let mut state = AppState::default();
    state.records.solitaire_best_moves = Some(42);

    assert_eq!(
        collection_progress(&state),
        CategoryProgress {
            completed: 1,
            total: GameId::ALL.len(),
        }
    );
}

#[test]
fn next_unfinished_game_follows_category_order() {
    let mut state = AppState::default();
    assert_eq!(next_unfinished_game(&state, 3), Some(GameId::Solitaire));

    state.records.solitaire_best_moves = Some(42);
    assert_eq!(next_unfinished_game(&state, 3), Some(GameId::FreeCell));

    for game in GameId::ALL {
        if category_filter(game) == 3 {
            match game {
                GameId::Solitaire => state.records.solitaire_best_moves = Some(42),
                GameId::FreeCell => state.records.freecell_best_moves = Some(31),
                GameId::MemoryPairs => state.records.memory_pairs_best_moves = Some(1),
                GameId::Spider => state.records.spider_best_moves = Some(1),
                GameId::MahjongSolitaire => state.records.mahjong_solitaire_best_moves = Some(1),
                GameId::HigherLower => state.records.higher_lower_best_score = Some(1),
                GameId::KlondikeGolf => state.records.klondike_golf_best_moves = Some(1),
                GameId::Blackjack => state.records.blackjack_best_wins = Some(1),
                GameId::SpiderSolitaire => state.records.spider_solitaire_best_moves = Some(1),
                GameId::Pyramid => state.records.pyramid_best_moves = Some(1),
                GameId::TriPeaks => state.records.tri_peaks_best_moves = Some(1),
                _ => {}
            }
        }
    }
    assert_eq!(next_unfinished_game(&state, 3), None);
}
