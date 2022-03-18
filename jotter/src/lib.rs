// Todo: Implement different layouts for better multithreading

pub struct Image {
    pixels: Vec<f32>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0f32; width * height],
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
}
