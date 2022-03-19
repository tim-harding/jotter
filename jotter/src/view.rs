use crate::{Image, Rect, Vector};

pub struct View<'a> {
    image: &'a mut Image,
    rect: Rect,
}

impl<'a> View<'a> {
    pub fn new(image: &'a mut Image, rect: Rect) -> Self {
        Self { image, rect }
    }

    pub fn splat(&mut self, vector: &Vector, pixel_radius: usize, opacity: f32) {
        let pixel_radius_f = pixel_radius as f32;
        let pixel_radius = pixel_radius as i32;
        let screen_space = self.to_screen(vector);
        let base_pixel = (screen_space.x as i32, screen_space.y as i32);
        for pixel_y in ((base_pixel.1 - pixel_radius - 1).max(0) as usize)
            ..((base_pixel.1 + pixel_radius + 1) as usize).min(self.image.height() - 1)
        {
            for pixel_x in ((base_pixel.0 - pixel_radius - 1).max(0) as usize)
                ..((base_pixel.0 + pixel_radius + 1) as usize).min(self.image.width() - 1)
            {
                let x_offset_f = pixel_x as f32 - screen_space.x;
                let y_offset_f = pixel_y as f32 - screen_space.y;
                let t = x_offset_f * x_offset_f + y_offset_f * y_offset_f;
                let t = (t / pixel_radius_f / pixel_radius_f).min(1.0);
                let t = 1.0 - (1.0 - t) * opacity;
                self.image.update(pixel_x, pixel_y, |old: f32| old * t);
            }
        }
    }

    pub fn shade(&mut self, subsamples: usize, callback: impl Fn(f32, f32) -> f32) {
        let subsamples_f = subsamples as f32;
        for y in 0..self.image.width() {
            for x in 0..self.image.height() {
                let mut acc = 0.0;
                for y_subsample in 0..subsamples {
                    for x_subsample in 0..subsamples {
                        let p = Vector::new(
                            (x * subsamples + x_subsample) as f32 / subsamples_f,
                            (y * subsamples + y_subsample) as f32 / subsamples_f,
                        );
                        let p = self.from_screen(&p);
                        acc += callback(p.x, p.y) / subsamples_f / subsamples_f;
                    }
                }
                self.image.set(x, y, acc);
            }
        }
    }

    fn to_view(&self, vector: &Vector) -> Vector {
        self.rect.to_local(vector)
    }

    fn from_view(&self, vector: &Vector) -> Vector {
        self.rect.from_view(vector)
    }

    fn to_screen(&self, vector: &Vector) -> Vector {
        let vector = self.to_view(vector);
        Vector {
            x: vector.x * self.image.width() as f32,
            y: vector.y * self.image.height() as f32,
        }
    }

    fn from_screen(&self, vector: &Vector) -> Vector {
        let vector = Vector {
            x: vector.x / self.image.width()  as f32,
            y: vector.y / self.image.height() as f32,
        };
        self.from_view(&vector)
    }
}
