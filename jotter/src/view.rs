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
        let p = Vector {
            x: screen_space.x.fract(),
            y: screen_space.y.fract(),
        };
        let pixel_x = screen_space.x as usize;
        let pixel_y = screen_space.y as usize;
        if pixel_x < self.image.width() - 2 && pixel_y < self.image.height() - 2 {
            for offset_y in 0..3 {
                for offset_x in 0..3 {
                    let base = Vector::new(-0.5 + offset_x as f32, -0.5 + offset_y as f32);
                    self.image
                        .update(pixel_x + offset_x, pixel_y + offset_y, |v| {
                            v * multiplier(opacity, p, base)
                        })
                }
            }
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

fn multiplier(opacity: f32, p: Vector, base: Vector) -> f32 {
    1.0 - opacity * (1.0 - (p.square_distance(base) / 2.25).min(1.0))
}
