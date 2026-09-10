use super::*;

#[test]
fn desktop_library_keeps_the_last_page_inside_the_viewport() {
    assert_eq!(library_page_start(60, 0), 0);
    assert_eq!(library_page_start(60, 44), 16);
    assert_eq!(library_page_start(60, 59), 16);
    assert_eq!(library_page_start(20, 19), 0);
}
