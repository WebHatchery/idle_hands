use super::*;

#[test]
fn archive_page_sizes_match_each_touch_layout() {
    crate::ui::with_desktop_layout(|| assert_eq!(page_size(), 10));
    crate::ui::with_compact_landscape_layout(|| assert_eq!(page_size(), 6));
    crate::ui::with_portrait_layout(|| assert_eq!(page_size(), 7));
}
