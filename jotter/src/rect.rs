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

    pub fn to_local(&self, vector: &Vector) -> Vector {
        Vector {
            x: self.horizontal.to_local(vector.x),
            y: self.vertical.to_local(vector.y),
        }
    }

    pub fn from_view(&self, vector: &Vector) -> Vector {
        Vector {
            x: self.horizontal.to_world(vector.x),
            y: self.vertical.to_world(vector.y),
        }
    }
}
