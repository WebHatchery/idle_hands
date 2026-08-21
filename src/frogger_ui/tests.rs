use super::*;

#[test]
fn compact_header_and_board_leave_room_for_instruction_text() {
    let (title_x, title_y, status_x, status_y) = compact_header();
    let layout = compact_layout();
    let instruction_y = 356.;

    assert!(title_x >= 110.);
    assert_eq!(title_x, status_x);
    assert!(status_y > title_y + 12.);
    assert!(layout.board.y > status_y + 8.);
    assert!(instruction_y > layout.board.y + layout.board.h + 8.);
}
