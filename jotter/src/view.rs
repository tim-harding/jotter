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

    fn to_view(&self, vector: &Vector) -> Vector {
        self.rect.to_local(vector)
    }

    fn to_screen(&self, vector: &Vector) -> Vector {
        let vector = self.to_view(vector);
        Vector {
            x: vector.x * self.image.width() as f32,
            y: vector.y * self.image.height() as f32,
        }
    }
}
