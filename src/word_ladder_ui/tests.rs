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
