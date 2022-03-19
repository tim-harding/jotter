use crate::Error;
use exr::prelude::{
    ChannelDescription, Encoding, Image as ExrImage, Layer, LayerAttributes, SampleType,
    SpecificChannels, Vec2, WritableImage,
};
use std::path::Path;
use std::result::Result;

// Todo:
// - Multithreaded access
// - Block-based storage
// - Guarantee 64-byte aligned rows for shading

pub struct Image {
    pixels: Vec<f32>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize, pixel: f32) -> Self {
        Self {
            pixels: vec![pixel; width * height],
            width,
            height,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        if self.in_bounds(x, y) {
            let i = self.index(x, y);
            // Todo: Fix slow checked access
            self.pixels[i] = value;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<f32> {
        if self.in_bounds(x, y) {
            Some(self.pixels[self.index(x, y)])
        } else {
            None
        }
    }

    pub fn update(&mut self, x: usize, y: usize, callback: impl Fn(f32) -> f32) {
        if self.in_bounds(x, y) {
            let i = self.index(x, y);
            let old = self.pixels[i];
            self.pixels[i] = callback(old);
        }
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        self.save_inner(path.as_ref())
    }

    #[inline(never)]
    fn save_inner(&self, path: &Path) -> Result<(), Error> {
        let channels = SpecificChannels::build()
            .with_channel_details(ChannelDescription::named("L", SampleType::F32))
            .with_pixels(|position: Vec2<usize>| (self.get(position.x(), position.y()).unwrap(),));
        let layer = Layer::new(
            (self.width, self.height),
            LayerAttributes::named("beauty"),
            Encoding::FAST_LOSSLESS,
            channels,
        );
        let image = ExrImage::from_layer(layer);
        image.write().to_file(path)?;
        Ok(())
    }
}
