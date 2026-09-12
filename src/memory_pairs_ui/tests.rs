//! Regression coverage for the tests module.

use super::*;

#[test]
fn portrait_memory_pairs_title_uses_a_narrow_header_size() {
    crate::ui::with_portrait_layout(|| assert_eq!(title_size(), 20.));
}
