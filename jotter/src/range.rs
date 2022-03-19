#[derive(Clone, Copy, Debug, Default)]
pub struct Range {
    pub start: f32,
    pub length: f32,
}

impl Range {
    pub fn endpoints(a: f32, b: f32) -> Self {
        Self {
            start: a,
            length: b - a,
        }
    }

    pub fn new(start: f32, length: f32) -> Self {
        Self { start, length }
    }

    pub fn to_local(&self, world: f32) -> f32 {
        (world - self.start) / self.length
    }

    pub fn from_local(&self, local: f32) -> f32 {
        local * self.length + self.start
    }

    pub fn end(&self) -> f32 {
        self.start + self.length
    }
}
