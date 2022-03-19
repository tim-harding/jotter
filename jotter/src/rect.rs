use crate::{Range, Vector};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub horizontal: Range,
    pub vertical: Range,
}

impl Rect {
    pub const SQUARE: Rect = Rect {
        horizontal: Range {
            start: 0.0,
            length: 1.0,
        },
        vertical: Range {
            start: 0.0,
            length: 1.0,
        },
    };

    pub fn new(horizontal: Range, vertical: Range) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn with_bounds(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        Self {
            horizontal: Range::endpoints(left, right),
            vertical: Range::endpoints(bottom, top),
        }
    }

    pub fn to_local(&self, vector: Vector) -> Vector {
        Vector {
            x: self.horizontal.to_local(vector.x),
            y: self.vertical.to_local(vector.y),
        }
    }

    pub fn from_local(&self, vector: Vector) -> Vector {
        Vector {
            x: self.horizontal.from_local(vector.x),
            y: self.vertical.from_local(vector.y),
        }
    }

    pub fn in_bounds(&self, vector: Vector) -> bool {
        vector.x > self.horizontal.start
            && vector.x < self.horizontal.end()
            && vector.y > self.vertical.start
            && vector.y < self.vertical.end()
    }
}
