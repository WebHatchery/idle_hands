use super::*;

#[test]
fn grid_maps_cells_and_rejects_all_outer_edges() {
    let grid = GridLayout::new(Rect::new(10., 20., 300., 200.), 3, 2);
    assert_eq!(grid.cell_rect(4), Some(Rect::new(110., 120., 100., 100.)));
    assert_eq!(grid.index_at(Vec2::new(110., 120.)), Some(4));
    assert_eq!(grid.coordinate_at(Vec2::new(299.9, 219.9)), Some((2, 1)));
    assert_eq!(grid.index_at(Vec2::new(310., 100.)), None);
    assert_eq!(grid.index_at(Vec2::new(100., 220.)), None);
}

#[test]
fn non_square_grids_keep_rows_and_columns_independent() {
    let grid = GridLayout::new(Rect::new(0., 0., 240., 100.), 4, 2);
    assert_eq!(grid.index_at(Vec2::new(130., 75.)), Some(6));
    assert_eq!(grid.coordinate_at(Vec2::new(70., 25.)), Some((1, 0)));
}
