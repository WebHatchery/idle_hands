use super::*;

#[test]
fn portrait_drop_controls_do_not_need_a_prompt_in_the_board_gap() {
    assert!(!show_drop_prompt(true, false));
    assert!(show_drop_prompt(false, false));
}

#[test]
fn compact_drop_controls_do_not_put_a_prompt_on_the_board() {
    assert!(!show_drop_prompt(false, true));
}
