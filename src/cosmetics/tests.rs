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
