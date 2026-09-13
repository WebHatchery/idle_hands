//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_mastermind_title_uses_a_narrow_readable_size() {
    crate::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}
