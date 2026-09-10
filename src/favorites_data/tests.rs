use super::*;

#[test]
fn browse_mode_follows_the_active_shelf() {
    let favorites = AppState::default();
    assert_eq!(BrowseMode::from_state(&favorites), BrowseMode::Favorites);

    let recent = AppState {
        recent_view: true,
        ..AppState::default()
    };
    assert_eq!(BrowseMode::from_state(&recent), BrowseMode::Recent);
}

#[test]
fn favorite_rows_keep_catalog_order_and_source_positions() {
    let mut state = AppState::default();
    state.favorites[GameId::FreeCell.index()] = true;
    state.favorites[GameId::Solitaire.index()] = true;

    let rows = page_rows(&state, BrowseMode::Favorites, 0, 8);

    assert_eq!(
        rows,
        vec![
            BrowseRow {
                game: GameId::Solitaire,
                source_index: 0,
            },
            BrowseRow {
                game: GameId::FreeCell,
                source_index: 1,
            },
        ]
    );
}

#[test]
fn recent_rows_preserve_launch_order_and_clamp_the_page() {
    let state = AppState {
        recent_view: true,
        recent_games: vec![GameId::WordSearch, GameId::Hangman, GameId::Mastermind],
        ..AppState::default()
    };

    let rows = page_rows(&state, BrowseMode::Recent, 99, 2);

    assert_eq!(rows[0].game, GameId::Hangman);
    assert_eq!(rows[0].source_index, 1);
    assert_eq!(rows[1].game, GameId::Mastermind);
}

#[test]
fn page_start_handles_empty_and_zero_capacity_safely() {
    assert_eq!(page_start(0, 99, 8), 0);
    assert_eq!(page_start(15, 99, 8), 7);
    assert_eq!(page_start(15, 99, 0), 14);
}

#[test]
fn summary_partitions_the_active_browse_shelf() {
    let mut state = AppState::default();
    state.records.solitaire_best_moves = Some(42);
    state.recent_games = vec![GameId::Solitaire, GameId::FreeCell];

    assert_eq!(
        summary(&state, BrowseMode::Recent),
        BrowseSummary {
            total: 2,
            open: 1,
            done: 1,
            locked: 0,
        }
    );
}
