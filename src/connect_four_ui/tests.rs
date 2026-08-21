use super::*;

#[test]
fn portrait_drop_controls_do_not_need_a_prompt_in_the_board_gap() {
    assert!(!show_drop_prompt(true));
    assert!(show_drop_prompt(false));
}
