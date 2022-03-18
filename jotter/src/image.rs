use crate::Error;
use exr::prelude::{
    ChannelDescription, Encoding, Image as ExrImage, Layer, LayerAttributes, SampleType,
    SpecificChannels, Vec2, WritableImage,
};
use std::path::Path;
use std::result::Result;

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
        let i = self.index(x, y);
        self.pixels[i] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.pixels[self.index(x, y)]
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

    fn save_inner(&self, path: &Path) -> Result<(), Error> {
        let channels = SpecificChannels::build()
            .with_channel_details(ChannelDescription::named("L", SampleType::F32))
            .with_pixels(|position: Vec2<usize>| (self.get(position.x(), position.y()),));
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
