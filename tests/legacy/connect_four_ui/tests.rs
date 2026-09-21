//! Regression coverage for the tests module.

use idle_hands::testing::modules::connect_four_ui::*;

#[test]
fn portrait_drop_controls_do_not_need_a_prompt_in_the_board_gap() {
    assert!(!show_drop_prompt(true, false));
    assert!(show_drop_prompt(false, false));
}

#[test]
fn compact_drop_controls_do_not_put_a_prompt_on_the_board() {
    assert!(!show_drop_prompt(false, true));
}

#[test]
fn compact_header_instruction_stays_outside_the_title_lane() {
    idle_hands::testing::ui::with_compact_landscape_layout(|| {
        let layout = layout();
        let instruction = compact_instruction_position();
        assert!(instruction.x >= layout.board.right());
        assert!(instruction.y < layout.levels[0].y);
    });
}

#[test]
fn portrait_connect_four_title_uses_a_narrow_readable_size() {
    idle_hands::testing::ui::with_portrait_layout(|| assert_eq!(title_size(), 22.));
}

#[test]
fn moves_summary_clears_the_drop_row() {
    idle_hands::testing::ui::with_desktop_layout(|| {
        assert!(moves_summary_y(false, false) >= layout().drops.bottom() + 18.);
    });
    idle_hands::testing::ui::with_portrait_layout(|| {
        assert!(moves_summary_y(false, true) >= layout().drops.bottom() + 18.);
    });
}
