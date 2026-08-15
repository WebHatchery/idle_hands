//! Shared logical viewport and cancellable pointer gesture normalization.

use macroquad::prelude::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub scale: f32,
    pub offset: Vec2,
    pub logical_width: f32,
    pub logical_height: f32,
}

impl Viewport {
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        logical_width: f32,
        logical_height: f32,
    ) -> Self {
        let scale = (screen_width / logical_width).min(screen_height / logical_height);
        Self {
            scale,
            offset: vec2(
                (screen_width - logical_width * scale) / 2.,
                (screen_height - logical_height * scale) / 2.,
            ),
            logical_width,
            logical_height,
        }
    }
    pub fn screen_to_logical(self, position: Vec2) -> Option<Vec2> {
        if !self.screen_bounds().contains(position) {
            return None;
        }
        Some((position - self.offset) / self.scale)
    }
    #[allow(dead_code)]
    pub fn logical_to_screen(self, position: Vec2) -> Vec2 {
        position * self.scale + self.offset
    }
    pub fn screen_bounds(self) -> Rect {
        Rect::new(
            self.offset.x,
            self.offset.y,
            self.logical_width * self.scale,
            self.logical_height * self.scale,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
    Tap(Vec2),
    Drag { start: Vec2, end: Vec2 },
}

#[derive(Debug, Default)]
pub struct PointerTracker {
    start: Option<Vec2>,
    cancelled: bool,
}

impl PointerTracker {
    pub fn press(&mut self, position: Option<Vec2>) {
        self.start = position;
        self.cancelled = position.is_none();
    }
    pub fn cancel(&mut self) {
        self.start = None;
        self.cancelled = true;
    }
    pub fn release(&mut self, position: Option<Vec2>) -> Option<Gesture> {
        let start = self.start.take()?;
        if self.cancelled {
            return None;
        }
        let end = position?;
        if (end - start).length() > 16. {
            Some(Gesture::Drag { start, end })
        } else {
            Some(Gesture::Tap(end))
        }
    }
}

fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}
use macroquad::prelude::Rect;

#[cfg(test)]
mod tests;
