use super::*;

#[test]
fn portrait_word_ladder_title_uses_a_narrow_header_size() {
    assert_eq!(title_size(false, true), 20.);
    assert_eq!(title_size(true, false), 22.);
    assert_eq!(title_size(false, false), 28.);
}
