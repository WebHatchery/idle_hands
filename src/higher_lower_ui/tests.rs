use super::*;

#[test]
fn portrait_higher_lower_title_uses_a_narrow_readable_size() {
    crate::ui::with_portrait_layout(|| {
        assert_eq!(title_size(), 20.);
        assert_eq!(title_text(), "HIGHER / LOWER");
    });
}
