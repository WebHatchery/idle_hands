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
