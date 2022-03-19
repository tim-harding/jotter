use bitvec::prelude::*;
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

fn main() -> Result<(), jotter::Error> {
    const DIM: usize = 4096;
    let mut image = Image::new(DIM, DIM, 1.0);
    let rect = Rect::with_bounds(-4096.0, 4096.0, -4096.0, 4096.0);
    let mut view = View::new(&mut image, rect);
    let mut spiral = ArchimedesSpiral::default();
    const ODDS_COUNT: usize = DIM * DIM / 2;
    let mut is_composite_vec: BitArr!(for ODDS_COUNT) = BitArray::ZERO;
    {
        spiral.next();
        let two = spiral.next();
        view.splat(two, 0.5);
    }
    for i in 1..ODDS_COUNT {
        let is_prime = !is_composite_vec[i];
        let p = spiral.next();
        spiral.next();
        if is_prime {
            let number = i * 2 + 1;
            let mut mask = i;
            while mask < ODDS_COUNT {
                is_composite_vec.set(mask, true);
                mask += number;
            }
            view.splat(p, 0.5);
        }
    }
    image.save("ulam_spiral.exr")?;
    Ok(())
}
