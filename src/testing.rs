//! Intentional public seams used by the crate-level deterministic test harness.

use macroquad::prelude::Vec2;

pub use crate::cabinet_status::status as cabinet_status;
pub use crate::data::GameData;
pub use crate::domain::Direction;
pub use crate::game_descriptor::{descriptor, GameDescriptor};
pub use crate::progression::completed_games;
pub use crate::state::{AppState, GameId, Screen};
pub use crate::state_snapshots::GameSnapshot;
pub use crate::ui::UiAction;

pub fn with_desktop_layout<T>(run: impl FnOnce() -> T) -> T {
    crate::ui::with_desktop_layout(run)
}

pub fn with_compact_landscape_layout<T>(run: impl FnOnce() -> T) -> T {
    crate::ui::with_compact_landscape_layout(run)
}

pub fn with_portrait_layout<T>(run: impl FnOnce() -> T) -> T {
    crate::ui::with_portrait_layout(run)
}

pub fn variant_label(state: &AppState, game: GameId) -> String {
    crate::game_variants::label(state, game)
}

pub fn cycle_variant(state: &mut AppState, data: &GameData, game: GameId) {
    crate::game_variants::cycle(state, data, game);
}

pub fn clicks_for_game(state: &AppState, game: GameId, point: Vec2) -> Vec<UiAction> {
    match game {
        GameId::Solitaire => crate::solitaire_ui::solitaire_clicks(state, point),
        GameId::FreeCell => crate::freecell_ui::freecell_clicks(state, point),
        GameId::Sudoku => crate::sudoku_ui::sudoku_clicks(state, point),
        GameId::Minesweeper => crate::minesweeper_ui::clicks(state, point),
        GameId::Game2048 => crate::ui::game_clicks(state, point),
        GameId::Nonogram => crate::nonogram_ui::nonogram_clicks(state, point),
        GameId::Yahtzee => crate::fivefold_ui::fivefold_clicks(state, point),
        GameId::Reversi => crate::reversi_ui::reversi_clicks(state, point),
        GameId::LightsOut => crate::lights_out_ui::clicks(state, point),
        GameId::TicTacToe => crate::tic_tac_toe_ui::clicks(state, point),
        GameId::MemoryPairs => crate::memory_pairs_ui::clicks(state, point),
        GameId::SlidingPuzzle => crate::sliding_puzzle_ui::clicks(state, point),
        GameId::Mastermind => crate::mastermind_ui::clicks(state, point),
        GameId::Spider => crate::spider_ui::clicks(state, point),
        GameId::WordSearch => crate::word_search_ui::clicks(state, point),
        GameId::Hangman => crate::hangman_ui::clicks(state, point),
        GameId::ConnectFour => crate::connect_four_ui::clicks(state, point),
        GameId::Checkers => crate::checkers_ui::clicks(state, point),
        GameId::PegSolitaire => crate::peg_solitaire_ui::clicks(state, point),
        GameId::MahjongSolitaire => crate::mahjong_solitaire_ui::clicks(state, point),
        GameId::Snake => crate::snake_ui::clicks(state, point),
        GameId::Breakout => crate::breakout_ui::clicks(state, point),
        GameId::HigherLower => crate::higher_lower_ui::clicks(state, point),
        GameId::KlondikeGolf => crate::klondike_golf_ui::clicks(state, point),
        GameId::Blackjack => crate::blackjack_ui::clicks(state, point),
        GameId::SpiderSolitaire => crate::spider_solitaire_ui::clicks(state, point),
        GameId::DungeonSweeper => crate::dungeon_sweeper_ui::clicks(state, point),
        GameId::Potion2048 => crate::potion_2048_ui::clicks(state, point),
        GameId::TinyTowerDefence => crate::tiny_tower_defence_ui::clicks(state, point),
        GameId::OneRoomRoguelike => crate::one_room_roguelike_ui::clicks(state, point),
        GameId::DailyDungeon => crate::daily_dungeon_ui::clicks(state, point),
        GameId::DotsBoxes => crate::dots_boxes_ui::clicks(state, point),
        GameId::Sokoban => crate::sokoban_ui::clicks(state, point),
        GameId::Mancala => crate::mancala_ui::clicks(state, point),
        GameId::Hanoi => crate::hanoi_ui::clicks(state, point),
        GameId::NumberMatch => crate::number_match_ui::clicks(state, point),
        GameId::FloodIt => crate::flood_it_ui::clicks(state, point),
        GameId::ColorSort => crate::color_sort_ui::clicks(state, point),
        GameId::Battleship => crate::battleship_ui::clicks(state, point),
        GameId::WordGrid => crate::word_grid_ui::clicks(state, point),
        GameId::PipeLoop => crate::pipe_loop_ui::clicks(state, point),
        GameId::MazeWalk => crate::maze_walk_ui::clicks(state, point),
        GameId::MatchThree => crate::match_three_ui::clicks(state, point),
        GameId::Pyramid => crate::pyramid_ui::clicks(state, point),
        GameId::TriPeaks => crate::tri_peaks_ui::clicks(state, point),
        GameId::Nim => crate::nim_ui::clicks(state, point),
        GameId::WordLadder => crate::word_ladder_ui::clicks(state, point),
        GameId::SpaceInvaders => crate::space_invaders_ui::clicks(state, point),
        GameId::Asteroids => crate::asteroids_ui::clicks(state, point),
        GameId::Frogger => crate::frogger_ui::clicks(state, point),
        GameId::MunchMaze => crate::munch_maze_ui::clicks(state, point),
        GameId::BlockStack => crate::block_stack_ui::clicks(state, point),
        GameId::TerrainCannon => crate::terrain_cannon_ui::clicks(state, point),
        GameId::FlingFury => crate::fling_fury_ui::clicks(state, point),
        GameId::PaddleDuel => crate::paddle_duel_ui::clicks(state, point),
        GameId::RiddleRoom
        | GameId::PatternVault
        | GameId::SumCircuit
        | GameId::OrbitOrder
        | GameId::WordForge => crate::misc_ui::clicks(state, point),
    }
}

pub mod games {
    pub mod asteroids {
        pub use crate::asteroids::*;
    }
    pub mod block_stack {
        pub use crate::block_stack::*;
    }
    pub mod breakout {
        pub use crate::breakout::*;
    }
    pub mod blackjack {
        pub use crate::blackjack::*;
    }
    pub mod battleship {
        pub use crate::battleship::*;
    }
    pub mod checkers {
        pub use crate::checkers::*;
    }
    pub mod color_sort {
        pub use crate::color_sort::*;
    }
    pub mod connect_four {
        pub use crate::connect_four::*;
    }
    pub mod daily_dungeon {
        pub use crate::daily_dungeon::*;
    }
    pub mod dots_boxes {
        pub use crate::dots_boxes::*;
    }
    pub mod dungeon_sweeper {
        pub use crate::dungeon_sweeper::*;
    }
    pub mod fivefold {
        pub use crate::fivefold::*;
    }
    pub mod fling_fury {
        pub use crate::fling_fury::*;
    }
    pub mod flood_it {
        pub use crate::flood_it::*;
    }
    pub mod freecell {
        pub use crate::freecell::*;
    }
    pub mod frogger {
        pub use crate::frogger::*;
    }
    pub mod game_2048 {
        pub use crate::game_2048::*;
    }
    pub mod hangman {
        pub use crate::hangman::*;
    }
    pub mod hanoi {
        pub use crate::hanoi::*;
    }
    pub mod higher_lower {
        pub use crate::higher_lower::*;
    }
    pub mod klondike_golf {
        pub use crate::klondike_golf::*;
    }
    pub mod lights_out {
        pub use crate::lights_out::*;
    }
    pub mod mahjong_solitaire {
        pub use crate::mahjong_solitaire::*;
    }
    pub mod mancala {
        pub use crate::mancala::*;
    }
    pub mod mastermind {
        pub use crate::mastermind::*;
    }
    pub mod match_three {
        pub use crate::match_three::*;
    }
    pub mod maze_walk {
        pub use crate::maze_walk::*;
    }
    pub mod memory_pairs {
        pub use crate::memory_pairs::*;
    }
    pub mod minesweeper {
        pub use crate::minesweeper::*;
    }
    pub mod misc_games {
        pub use crate::misc_games::*;
    }
    pub mod munch_maze {
        pub use crate::munch_maze::*;
    }
    pub mod nim {
        pub use crate::nim::*;
    }
    pub mod nonogram {
        pub use crate::nonogram::*;
    }
    pub mod number_match {
        pub use crate::number_match::*;
    }
    pub mod one_room_roguelike {
        pub use crate::one_room_roguelike::*;
    }
    pub mod paddle_duel {
        pub use crate::paddle_duel::*;
    }
    pub mod peg_solitaire {
        pub use crate::peg_solitaire::*;
    }
    pub mod pipe_loop {
        pub use crate::pipe_loop::*;
    }
    pub mod potion_2048 {
        pub use crate::potion_2048::*;
    }
    pub mod pyramid {
        pub use crate::pyramid::*;
    }
    pub mod reversi {
        pub use crate::reversi::*;
    }
    pub mod sliding_puzzle {
        pub use crate::sliding_puzzle::*;
    }
    pub mod snake {
        pub use crate::snake::*;
    }
    pub mod sokoban {
        pub use crate::sokoban::*;
    }
    pub mod solitaire {
        pub use crate::solitaire::*;
    }
    pub mod space_invaders {
        pub use crate::space_invaders::*;
    }
    pub mod spider {
        pub use crate::spider::*;
    }
    pub mod spider_solitaire {
        pub use crate::spider_solitaire::*;
    }
    pub mod sudoku {
        pub use crate::sudoku::*;
    }
    pub mod terrain_cannon {
        pub use crate::terrain_cannon::*;
    }
    pub mod tic_tac_toe {
        pub use crate::tic_tac_toe::*;
    }
    pub mod tiny_tower_defence {
        pub use crate::tiny_tower_defence::*;
    }
    pub mod tri_peaks {
        pub use crate::tri_peaks::*;
    }
    pub mod word_grid {
        pub use crate::word_grid::*;
    }
    pub mod word_ladder {
        pub use crate::word_ladder::*;
    }
    pub mod word_search {
        pub use crate::word_search::*;
    }
}

/// Source-module seams for the migrated regression suites.
///
/// These modules are deliberately grouped below the hidden `testing` boundary:
/// production callers continue to use the crate's normal public surface while
/// integration tests can exercise the existing rule and layout seams without
/// reintroducing test code into `src/`.
pub mod modules {
    macro_rules! expose_modules {
        ($($module:ident),* $(,)?) => {
            $(
                #[allow(ambiguous_glob_reexports, unused_imports)]
                pub mod $module {
                    pub use crate::$module::*;
                    pub use crate::cards::Card;
                    pub use crate::data::GameData;
                    pub use crate::domain::Direction;
                    pub use crate::state::{AppState, CollectionRecords, GameId, Screen};
                    pub use crate::ui::UiAction;
                    pub use macroquad::prelude::*;
                    pub use macroquad_toolkit::notifications::LoggedNotification;
                    pub use macroquad_toolkit::rng::SeededRng;
                }
            )*
        };
    }

    pub mod game {
        pub use crate::domain::Direction;
        pub use crate::game::*;
        pub use crate::state::{AppState, GameId, Screen};
        pub use crate::ui::UiAction;
        pub use macroquad::prelude::*;

        pub mod game_restart {
            pub use crate::game::testing::game_restart::*;
        }
        pub mod game_capture_records {
            pub use crate::game::testing::game_capture_records::*;
            pub use crate::state::{AppState, GameId, Screen};
        }
        pub mod game_capture_rules {
            pub use crate::game::testing::game_capture_rules::*;
            pub use crate::state::{AppState, GameId, Screen};
        }
    }

    #[allow(ambiguous_glob_reexports, unused_imports)]
    pub mod ui {
        pub use crate::state::{AppState, GameId, Screen};
        pub use crate::testing::{
            with_compact_landscape_layout, with_desktop_layout, with_portrait_layout,
        };
        pub use crate::ui::UiAction;
        pub use crate::ui::*;
        pub use macroquad::prelude::*;

        pub mod restart_modal {
            pub use crate::ui::testing::restart_modal::*;
        }
    }

    expose_modules!(
        achievements_data,
        achievements_ui,
        analytics,
        accessibility,
        asteroids,
        asteroids_ui,
        audio_settings,
        battleship,
        blackjack,
        blackjack_ui,
        block_stack,
        block_stack_ui,
        breakout,
        breakout_ui,
        cabinet_data,
        cabinet_status,
        cabinet_ui,
        capture_registry,
        card_hints,
        card_render,
        cards,
        checkers,
        collection_summary,
        color_sort,
        connect_four,
        connect_four_ui,
        content,
        continue_data,
        cosmetics,
        credits_data,
        daily_archive_data,
        daily_archive_ui,
        daily_challenge,
        daily_dungeon,
        data,
        dense_focus_ui,
        domain,
        dots_boxes,
        dots_boxes_ui,
        drawer_info_ui,
        dungeon_sweeper,
        dungeon_sweeper_ui,
        favorites_data,
        favorites_ui,
        finder_data,
        finder_ui,
        fivefold,
        fling_fury,
        fling_fury_ui,
        flood_it,
        freecell,
        frogger,
        frogger_ui,
        game_2048,
        game_actions,
        game_descriptor,
        game_input,
        game_result_ui,
        game_store,
        game_variant_ui,
        game_variants,
        grid,
        hangman,
        hangman_ui,
        hanoi,
        help_data,
        higher_lower,
        higher_lower_ui,
        input,
        klondike_golf,
        klondike_golf_ui,
        library_ui,
        lifecycle,
        lifecycle_pause_ui,
        lights_out,
        lights_out_ui,
        mahjong_solitaire,
        mahjong_solitaire_ui,
        mancala,
        mastermind,
        mastermind_ui,
        match_three,
        match_three_ui,
        maze_walk,
        maze_walk_ui,
        memory_pairs,
        memory_pairs_ui,
        minesweeper,
        misc_games,
        misc_ui,
        mobile_tutorial_ui,
        munch_maze,
        munch_maze_ui,
        nim,
        nim_ui,
        nonogram,
        notice_log_ui,
        number_match,
        number_match_ui,
        one_room_roguelike,
        one_room_roguelike_ui,
        paddle_duel,
        paddle_duel_ui,
        peg_solitaire,
        peg_solitaire_ui,
        persistence_models,
        pipe_loop,
        pipe_loop_ui,
        potion_2048,
        potion_2048_ui,
        profile_data,
        profile_ui,
        progression,
        pyramid,
        pyramid_ui,
        records_data,
        records_ui,
        responsive_cabinet,
        responsive_cards,
        responsive_fivefold,
        responsive_landscape,
        responsive_landscape_cabinet,
        responsive_landscape_cards,
        responsive_landscape_games,
        responsive_landscape_library,
        responsive_landscape_rules,
        responsive_library,
        responsive_sudoku,
        responsive_puzzles,
        responsive_ui,
        reversi,
        rules_data,
        save_recovery,
        save_recovery_ui,
        settings_data,
        sliding_puzzle,
        sliding_puzzle_ui,
        snake,
        snake_ui,
        sokoban,
        solitaire,
        solitaire_ui,
        space_invaders,
        space_invaders_ui,
        spider,
        spider_solitaire,
        spider_solitaire_ui,
        spider_ui,
        sound,
        state,
        state_navigation,
        state_records,
        state_snapshots,
        statistics_ui,
        stats_data,
        storefront_data,
        sudoku,
        terrain_cannon,
        terrain_cannon_ui,
        tic_tac_toe,
        tic_tac_toe_ui,
        tiny_tower_defence,
        tiny_tower_defence_ui,
        theme,
        tri_peaks,
        tri_peaks_ui,
        tutorial_data,
        tutorial_library_data,
        tutorial_library_ui,
        tutorial_ui,
        ui_action,
        ui_game_routes,
        variant_card_data,
        word_grid,
        word_grid_ui,
        word_ladder,
        word_ladder_ui,
        word_search,
        word_search_ui,
    );
}

// Keep the short `testing::state` style used inside migrated suites while the
// explicit `modules` namespace remains the canonical import location.
pub use modules::*;
