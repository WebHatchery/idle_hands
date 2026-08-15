use super::*;

#[test]
fn first_finish_awards_once_and_full_cabinet_is_worth_two() {
    let records = CollectionRecords {
        best_2048: 2048,
        ..Default::default()
    };
    let mut earned_flags = [false; 10];
    let mut stamps = 0;
    sync(&mut earned_flags, &mut stamps, &records);
    assert_eq!(stamps, 2);
    assert!(earned_flags[AchievementId::FirstFinish.index()]);
    sync(&mut earned_flags, &mut stamps, &records);
    assert_eq!(stamps, 2);
}

#[test]
fn completed_game_count_requires_each_collection_game() {
    let mut records = CollectionRecords::default();
    records.sudoku[0] = Some(12);
    records.nonogram[2] = Some(20);
    assert_eq!(completed_games(&records), 2);
}

#[test]
#[allow(clippy::field_reassign_with_default)]
fn full_cabinet_counts_every_playable_game() {
    let mut records = CollectionRecords::default();
    records.solitaire_best_moves = Some(1);
    records.freecell_best_moves = Some(1);
    records.sudoku[0] = Some(1);
    records.minesweeper[0] = Some(1);
    records.best_2048 = 2048;
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

    assert_eq!(completed_games(&records), GameId::ALL.len());
    assert!(earned(&records, AchievementId::FullCabinet));
    records.daily_dungeon_best_score = None;
    assert_eq!(completed_games(&records), GameId::ALL.len() - 1);
    assert!(!earned(&records, AchievementId::FullCabinet));
}
