//! Regression coverage for the tests module.

use super::*;

#[test]
fn cycling_skips_locked_cosmetics() {
    assert_eq!(next_card_back(0, 1), 0);
    assert_eq!(next_card_back(0, 2), 1);
    assert_eq!(next_card_back(1, 5), 2);
}

#[test]
fn every_cosmetic_category_has_a_free_default() {
    assert_eq!(next_board_theme(0, 0), 0);
    assert_eq!(next_sound_set(0, 0), 0);
    assert_eq!(next_cabinet_decoration(0, 0), 0);
}

#[test]
fn cosmetic_catalogs_report_unlock_counts_and_next_costs() {
    assert_eq!(CosmeticKind::ALL.len(), 4);
    assert_eq!(CosmeticKind::CardBack.unlocked_count(0), 1);
    assert_eq!(CosmeticKind::CardBack.unlocked_count(5), 3);
    assert_eq!(CosmeticKind::BoardTheme.next_cost(0), Some(3));
    assert_eq!(CosmeticKind::BoardTheme.next_cost(7), None);
    assert_eq!(CosmeticKind::CardBack.normalize(2, 0), 0);
    assert_eq!(CosmeticKind::CardBack.normalize(1, 2), 1);
    assert_eq!(CosmeticKind::SoundSet.normalize(7, 0), 0);
    assert_eq!(CosmeticKind::SoundSet.options()[1].name, "Rain on glass");
    assert_eq!(CosmeticKind::SoundSet.options()[1].cost, 4);
    assert_eq!(total_options(), 12);
    assert_eq!(total_unlocked(0), 4);
    assert_eq!(total_unlocked(8), 12);
}
