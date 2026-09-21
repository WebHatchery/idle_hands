//! Regression coverage for the tests module.

use idle_hands::testing::modules::spider_solitaire_ui::*;
use idle_hands::testing::spider_solitaire::SpiderSolitaire;

#[test]
fn compact_spider_solitaire_subtitle_stays_before_the_rule_card() {
    let subtitle_x = std::hint::black_box(COMPACT_SUBTITLE_X);
    assert!(subtitle_x + 190. < 494.);
}

#[test]
fn portrait_title_uses_the_header_lane_before_the_rule_card() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}

#[test]
fn held_pointer_can_peek_at_a_hidden_tableau_card() {
    idle_hands::testing::ui::with_desktop_layout(|| {
        let game = SpiderSolitaire::new(42);
        let layout = layout();
        let hidden_depth = 0;
        let card = layout.card_rect(0, hidden_depth);
        let point = vec2(card.center().x, card.y + 5.);
        assert_eq!(tableau_card_at(&game, point), Some((0, hidden_depth)));
    });
}

#[test]
fn peek_ignores_space_below_the_tableau_stack() {
    idle_hands::testing::ui::with_desktop_layout(|| {
        let game = SpiderSolitaire::new(42);
        let layout = layout();
        let last = game.tableau[0].len() - 1;
        let point = vec2(
            layout.card_rect(0, last).center().x,
            layout.card_rect(0, last).bottom() + 1.,
        );
        assert_eq!(tableau_card_at(&game, point), None);
    });
}
