use super::*;

#[test]
fn portrait_sliding_puzzle_title_uses_a_narrow_header_size() {
    crate::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}
