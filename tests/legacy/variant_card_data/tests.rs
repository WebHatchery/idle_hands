//! Regression coverage for the tests module.

use idle_hands::testing::modules::variant_card_data::*;

fn assert_card_fits(layout: CardLayout, width: f32, height: f32) {
    assert!(layout.rect.x >= 0.);
    assert!(layout.rect.y >= 0.);
    assert!(layout.rect.right() <= width);
    assert!(layout.rect.bottom() <= height);
    assert!(layout.label_rect.x >= layout.rect.x);
    assert!(layout.label_rect.y >= layout.rect.y);
    assert!(layout.label_rect.right() <= layout.rect.right());
    assert!(layout.label_rect.bottom() <= layout.rect.bottom());
    assert!(layout.label_size > 0.);
    assert!(layout.title_size > 0.);
}

#[test]
fn desktop_rule_card_stays_inside_the_logical_viewport() {
    assert_card_fits(layout(1280., false, false), 1280., 720.);
}

#[test]
fn compact_rule_card_stays_inside_the_logical_viewport() {
    assert_card_fits(layout(844., false, true), 844., 390.);
}

#[test]
fn portrait_rule_card_stays_inside_the_logical_viewport() {
    assert_card_fits(layout(360., true, false), 360., 780.);
}
