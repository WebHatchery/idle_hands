//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_word_ladder_title_uses_a_narrow_header_size() {
    assert_eq!(title_size(false, true), 20.);
    assert_eq!(title_size(true, false), 22.);
    assert_eq!(title_size(false, false), 28.);
}

#[test]
fn portrait_word_ladder_controls_stay_inside_the_logical_width() {
    crate::ui::with_portrait_layout(|| {
        let layout = layout();
        assert!(layout.mode.right() <= crate::responsive_ui::WIDTH);
        assert!(layout.new_game.right() < layout.mode.x);
    });
}

#[test]
fn partial_word_input_replaces_stale_validation_feedback() {
    let mut game = WordLadder::new(0);
    game.message = "That word is not in the dictionary".into();
    game.current = "AB".into();

    let feedback = contextual_message(&game);
    assert!(feedback.starts_with("INPUT 2/5  ·  "));
    assert!(!feedback.contains("not in the dictionary"));
}
