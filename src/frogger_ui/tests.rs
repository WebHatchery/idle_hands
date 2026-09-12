//! Regression coverage for the tests module.

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

#[test]
fn textured_traffic_waits_until_a_whole_car_fits_inside_the_board() {
    let fully_visible = Car {
        row: 2,
        x: WIDTH - 2,
        length: 2,
        direction: 1,
    };
    let wrapping = Car {
        row: 2,
        x: WIDTH - 1,
        length: 2,
        direction: 1,
    };

    assert!(car_fits_board(&fully_visible));
    assert!(!car_fits_board(&wrapping));
}
