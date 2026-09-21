//! Regression coverage for the tests module.

use idle_hands::testing::modules::progression::*;

#[test]
fn first_finish_awards_once_and_full_cabinet_is_worth_two() {
    let mut records = CollectionRecords {
        best_2048: 2048,
        ..Default::default()
    };
    records.record_time(GameId::Game2048.index());
    let mut earned_flags = Vec::new();
    let mut stamps = 0;
    let content = idle_hands::testing::data::GameData::load().unwrap().content;
    sync_with_content(&mut earned_flags, &mut stamps, &records, &content);
    assert_eq!(stamps, 2);
    assert_eq!(earned_flags.len(), AchievementId::ALL.len());
    assert!(earned_flags[AchievementId::FirstFinish.index()]);
    sync_with_content(&mut earned_flags, &mut stamps, &records, &content);
    assert_eq!(stamps, 2);
}

#[test]
// This case marks every catalog slot explicitly to verify the full-cabinet boundary.
#[allow(clippy::field_reassign_with_default)]
fn full_cabinet_counts_every_playable_game() {
    let mut records = CollectionRecords::default();
    records.solitaire_best_moves = Some(1);
    records.freecell_best_moves = Some(1);
    records.sudoku[0] = Some(1);
    records.minesweeper[0] = Some(1);
    records.best_2048 = 2048;
    records.record_time(GameId::Game2048.index());
    records.nonogram[0] = Some(1);
    records.fivefold_best_total = 1;
    records.reversi_best_score = 1;
    records.lights_out_best_moves = Some(1);
    records.tic_tac_toe_best_moves = Some(1);
    records.memory_pairs_best_moves = Some(1);
    records.sliding_puzzle_best_moves = Some(1);
    records.mastermind_best_rows = Some(1);
    records.spider_best_moves = Some(1);
    records.word_search_best_moves = Some(1);
    records.hangman_best_moves = Some(1);
    records.connect_four_best_moves = Some(1);
    records.checkers_best_moves = Some(1);
    records.peg_solitaire_best_moves = Some(1);
    records.mahjong_solitaire_best_moves = Some(1);
    records.snake_best_score = Some(1);
    records.breakout_best_score = Some(1);
    records.higher_lower_best_score = Some(1);
    records.klondike_golf_best_moves = Some(1);
    records.blackjack_best_wins = Some(1);
    records.spider_solitaire_best_moves = Some(1);
    records.dungeon_sweeper_best_moves = Some(1);
    records.potion_2048_best_score = Some(2048);
    records.tiny_tower_defence_best_wave = Some(8);
    records.one_room_roguelike_best_score = Some(50);
    records.daily_dungeon_best_score = Some(50);
    records.dots_boxes_best_score = Some(8);
    records.sokoban_best_moves = Some(20);
    records.mancala_best_score = Some(25);
    records.hanoi_best_moves = Some(31);
    records.number_match_best_moves = Some(18);
    records.flood_it_best_moves = Some(12);
    records.color_sort_best_moves = Some(24);
    records.battleship_best_moves = Some(16);
    records.word_grid_best_moves = Some(4);
    records.pipe_loop_best_moves = Some(30);
    records.maze_walk_best_moves = Some(12);
    records.match_three_best_score = Some(120);
    records.pyramid_best_moves = Some(28);
    records.tri_peaks_best_moves = Some(28);
    records.nim_best_moves = Some(10);
    records.word_ladder_best_moves = Some(5);
    records.space_invaders_best_score = Some(120);
    records.asteroids_best_score = Some(120);
    records.frogger_best_score = Some(75);
    records.munch_maze_best_score = Some(100);
    records.block_stack_best_score = Some(1000);
    records.terrain_cannon_best_score = Some(300);
    records.fling_fury_best_score = Some(200);
    records.paddle_duel_best_score = Some(7);
    records.misc_best_moves = [Some(5), Some(5), Some(4), Some(8), Some(5)];

    assert_eq!(completed_games(&records), GameId::ALL.len());
    assert!(earned(&records, AchievementId::FullCabinet));
    records.daily_dungeon_best_score = None;
    assert_eq!(completed_games(&records), GameId::ALL.len() - 1);
    assert!(!earned(&records, AchievementId::FullCabinet));
}

#[test]
fn achievement_progress_explains_single_and_collection_goals() {
    let records = CollectionRecords::default();
    let content = idle_hands::testing::data::GameData::load().unwrap().content;
    assert_eq!(
        AchievementId::FirstFinish.description_from(&content),
        "Finish any drawer"
    );
    assert_eq!(
        AchievementId::Game(GameId::Solitaire).description_from(&content),
        "Finish Solitaire"
    );
    assert_eq!(
        AchievementId::FullCabinet.description_from(&content),
        "Finish every drawer"
    );
    assert_eq!(
        AchievementId::FullCabinet.progress(&records),
        AchievementProgress {
            current: 0,
            target: GameId::ALL.len()
        }
    );
    assert_eq!(
        AchievementId::FullCabinet.progress_label(&records),
        format!("0 / {}", GameId::ALL.len())
    );
}

#[test]
fn achievement_progress_marks_finished_drawers_complete() {
    let records = CollectionRecords {
        solitaire_best_moves: Some(42),
        ..Default::default()
    };
    assert_eq!(
        AchievementId::FirstFinish.progress_label(&records),
        "COMPLETE"
    );
    assert_eq!(
        AchievementId::Game(GameId::Solitaire).progress_label(&records),
        "COMPLETE"
    );
    assert_eq!(
        AchievementId::Game(GameId::FreeCell).progress_label(&records),
        "0 / 1"
    );
}

#[test]
fn next_locked_achievement_follows_the_collection_order() {
    let empty = CollectionRecords::default();
    assert_eq!(
        AchievementId::next_locked(&empty),
        Some(AchievementId::FirstFinish)
    );
    let mut records = CollectionRecords {
        best_2048: 2048,
        ..Default::default()
    };
    records.record_time(GameId::Game2048.index());
    assert_eq!(
        AchievementId::next_locked(&records),
        Some(AchievementId::Game(GameId::Solitaire))
    );
}
