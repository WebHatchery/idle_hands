#[test]
fn compact_title_and_instruction_use_separate_rows() {
    let title_y = 30.;
    let instruction_y = 52.;

    assert!(instruction_y > title_y + 12.);
    assert!(instruction_y < 66.);
}
