//! Regression coverage for the tests module.

use super::*;

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

#[test]
fn cabinet_sort_modes_cycle_and_keep_ordering_deterministic() {
    let mut state = AppState::default();
    assert_eq!(CabinetSort::from_index(0), CabinetSort::Title);
    assert_eq!(CabinetSort::Title.next(), CabinetSort::Progress);
    assert_eq!(CabinetSort::Progress.next(), CabinetSort::Recent);
    assert_eq!(CabinetSort::Recent.next(), CabinetSort::Title);
    assert_eq!(CabinetSort::from_index(8), CabinetSort::Recent);
    assert_eq!(CabinetSort::Title.button_label(), "A-Z");
    assert_eq!(CabinetSort::Progress.button_label(), "OPEN FIRST");
    assert_eq!(CabinetSort::Recent.button_label(), "LAST PLAYED");

    let title_sorted = sorted_games(&state, 9, CabinetSort::Title);
    assert_eq!(title_sorted.first(), Some(&GameId::Game2048));

    state.records.best_2048 = 2048;
    state.records.record_time(GameId::Game2048.index());
    let progress_sorted = sorted_games(&state, 9, CabinetSort::Progress);
    assert_eq!(progress_sorted.last(), Some(&GameId::Game2048));

    state.recent_games = vec![GameId::Solitaire, GameId::Game2048];
    let recent_sorted = sorted_games(&state, 9, CabinetSort::Recent);
    assert_eq!(&recent_sorted[..2], &[GameId::Solitaire, GameId::Game2048]);
}

#[test]
fn availability_counts_match_the_active_category() {
    let counts = availability_counts(&AppState::default(), 3);
    assert_eq!(counts.playable, if cfg!(feature = "demo") { 5 } else { 11 });
    assert_eq!(
        counts.full_version,
        if cfg!(feature = "demo") { 6 } else { 0 }
    );
    assert_eq!(counts.coming_soon, 0);
}
