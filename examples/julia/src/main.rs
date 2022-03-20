use complex::Complex;
use jotter::{Image, Rect, Vector, View};

mod complex;

fn main() -> Result<(), jotter::Error> {
    const DIM_X: usize = 4096;
    const DIM_Y: usize = DIM_X / 2;
    let rect = Rect::with_bounds(-2.0, 2.0, -1.0, 1.0);
    let mut image = Image::new(DIM_X, DIM_Y, 1.0);
    let mut view = View::new(&mut image, rect);
    view.shade(1, |x, y| {
        let z = Vector::new(x, y);
        let mut z: Complex = z.into();
        let c: Complex = Complex {
            real: -1.33,
            imaginary: 0.02,
        };
        let mut n = 0;
        while n < 64 && z.abs() <= 2.0 {
            z = z * z + c;
            n += 1;
        }
        1.0 / n as f32
    });
    image.save("julia.exr")?;
    Ok(())
}