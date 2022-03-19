use crate::{Image, Rect, Vector};

pub struct View<'a> {
    image: &'a mut Image,
    rect: Rect,
}

impl<'a> View<'a> {
    pub fn new(image: &'a mut Image, rect: Rect) -> Self {
        Self { image, rect }
    }

    pub fn splat(&mut self, vector: Vector, opacity: f32) {
        let screen_space = self.to_screen(vector);
        let dist_x = screen_space.x.fract();
        let dist_y = screen_space.y.fract();
        let pixel_x = screen_space.x as usize;
        let pixel_y = screen_space.y as usize;
        if screen_space.x >= 0.0
            && pixel_x < self.image.width() - 1
            && screen_space.y >= 0.0
            && pixel_y < self.image.height() - 1
        {
            let flip_x = 1.0 - dist_x;
            let flip_y = 1.0 - dist_y;
            let next_x = pixel_x + 1;
            let next_y = pixel_y + 1;
            self.image.update(pixel_x, pixel_y, |v| {
                v * multiplier(opacity, dist_x, dist_y)
            });
            self.image
                .update(next_x, pixel_y, |v| v * multiplier(opacity, flip_x, dist_y));
            self.image
                .update(pixel_x, next_y, |v| v * multiplier(opacity, dist_x, flip_y));
            self.image
                .update(next_x, next_y, |v| v * multiplier(opacity, flip_x, flip_y));
        }
    }

    pub fn shade(&mut self, subsamples: usize, callback: impl Fn(f32, f32) -> f32) {
        let subsamples_f = subsamples as f32;
        for y in 0..self.image.height() {
            for x in 0..self.image.width() {
                let mut acc = 0.0;
                for y_subsample in 0..subsamples {
                    for x_subsample in 0..subsamples {
                        let p = Vector {
                            x: (x * subsamples + x_subsample) as f32 / subsamples_f,
                            y: (y * subsamples + y_subsample) as f32 / subsamples_f,
                        };
                        let p = self.from_screen(p);
                        acc += callback(p.x, p.y) / subsamples_f / subsamples_f;
                    }
                }
                self.image.set(x, y, acc);
            }
        }
    }

    fn to_view(&self, vector: Vector) -> Vector {
        self.rect.to_local(vector)
    }

    fn from_view(&self, vector: Vector) -> Vector {
        self.rect.from_local(vector)
    }

    fn to_screen(&self, vector: Vector) -> Vector {
        let vector = self.to_view(vector);
        Vector {
            x: vector.x * self.image.width() as f32,
            y: vector.y * self.image.height() as f32,
        }
    }

    fn from_screen(&self, vector: Vector) -> Vector {
        let vector = Vector {
            x: vector.x / self.image.width() as f32,
            y: vector.y / self.image.height() as f32,
        };
        self.from_view(vector)
    }
}

fn multiplier(opacity: f32, x: f32, y: f32) -> f32 {
    1.0 - opacity * (1.0 - x * x * y * y)
}
