use jotter::{Image, Rect, Vector, View};
use std::f32::consts::TAU;

#[derive(Debug, Default, Clone, Copy)]
struct ArchimedesSpiral {
    step: usize,
}

impl ArchimedesSpiral {
    pub fn next(&mut self) -> Vector {
        let t = self.step as f32;
        let t = t.sqrt();
        let trig_parameter = TAU * t;
        let x = t * trig_parameter.cos();
        let y = t * trig_parameter.sin();
        self.step += 1;
        Vector::new(x, y)
    }
}

struct Bits {
    data: Vec<u64>,
}

impl Bits {
    pub fn new(count: usize) -> Self {
        Self {
            data: vec![0; count / 64 + 1],
        }
    }

    pub fn set(&mut self, i: usize) {
        let n = i / 64;
        let bit = i % 64;
        self.data[n] |= 1 << bit;
    }

    pub fn get(&self, i: usize) -> bool {
        let n = i / 64;
        let bit = i % 64;
        (self.data[n] >> bit) & 1 == 1
    }
}

fn main() -> Result<(), jotter::Error> {
    const DIM: usize = 4096;
    const SQRT_PRIME_COUNT: usize = 1 << 12;
    const SQRT_PRIME_COUNT_F: f32 = SQRT_PRIME_COUNT as f32;
    let mut image = Image::new(DIM, DIM, 1.0);
    let rect = Rect::with_bounds(
        -SQRT_PRIME_COUNT_F,
        SQRT_PRIME_COUNT_F,
        -SQRT_PRIME_COUNT_F,
        SQRT_PRIME_COUNT_F,
    );
    let mut view = View::new(&mut image, rect);
    let mut spiral = ArchimedesSpiral::default();
    const ODDS_COUNT: usize = SQRT_PRIME_COUNT * SQRT_PRIME_COUNT / 2;
    let mut is_composite_vec = Bits::new(ODDS_COUNT);
    {
        spiral.next();
        let two = spiral.next();
        view.splat(two, 1.0);
    }
    for i in 1..ODDS_COUNT {
        let is_prime = !is_composite_vec.get(i);
        let p = spiral.next();
        spiral.next();
        if is_prime {
            let number = i * 2 + 1;
            let mut mask = i;
            while mask < ODDS_COUNT {
                is_composite_vec.set(mask);
                mask += number;
            }
            view.splat(p, 1.0);
        }
    }
    image.save("ulam_spiral.exr")?;
    Ok(())
}
