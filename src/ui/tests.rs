use super::*;

#[test]
fn compact_header_title_starts_after_the_cabinet_breadcrumb() {
    let breadcrumb_budget = 76.;
    let title_x = std::hint::black_box(COMPACT_HEADER_TITLE_X);
    assert!(title_x > breadcrumb_budget);
    assert!(title_x < 494.);
    let status_x = std::hint::black_box(COMPACT_HEADER_STATUS_X);
    assert!(status_x + 190. < 494.);
}
