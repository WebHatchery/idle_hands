//! Shared logical grid geometry for touch hit-testing and responsive drawing.

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridLayout {
    pub bounds: Rect,
    pub columns: usize,
    pub rows: usize,
    pub cell_width: f32,
    pub cell_height: f32,
}
impl GridLayout {
    pub fn new(bounds: Rect, columns: usize, rows: usize) -> Self {
        assert!(columns > 0 && rows > 0);
        Self {
            bounds,
            columns,
            rows,
            cell_width: bounds.w / columns as f32,
            cell_height: bounds.h / rows as f32,
        }
    }
    pub fn cell_rect(self, index: usize) -> Option<Rect> {
        if index >= self.columns * self.rows {
            return None;
        }
        Some(Rect::new(
            self.bounds.x + (index % self.columns) as f32 * self.cell_width,
            self.bounds.y + (index / self.columns) as f32 * self.cell_height,
            self.cell_width,
            self.cell_height,
        ))
    }
    pub fn index_at(self, point: Vec2) -> Option<usize> {
        if point.x < self.bounds.x
            || point.y < self.bounds.y
            || point.x >= self.bounds.right()
            || point.y >= self.bounds.bottom()
        {
            return None;
        }
        let column = ((point.x - self.bounds.x) / self.cell_width) as usize;
        let row = ((point.y - self.bounds.y) / self.cell_height) as usize;
        Some(row * self.columns + column)
    }
    pub fn coordinate_at(self, point: Vec2) -> Option<(usize, usize)> {
        self.index_at(point)
            .map(|index| (index % self.columns, index / self.columns))
    }
}

#[cfg(test)]
#[path = "../tests/legacy/grid/tests.rs"]
mod tests;
