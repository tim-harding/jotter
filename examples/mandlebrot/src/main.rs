use complex::Complex;
use jotter::{Image, Rect, View};

mod complex;

fn main() -> Result<(), jotter::Error> {
    let mut image = Image::new(1024, 1024, 0.0);
    let rect = Rect::with_bounds(-1.0, 1.0, -1.0, 1.0);
    let mut view = View::new(&mut image, rect);
    view.shade(1, |x: f32, y: f32| {
        let point = Complex::new(x, y);
        let mut z: Complex = 0.0.into();
        let mut n = 1;
        while n < 64 && z.abs() <= 2.0 {
            z = z * z + point;
            n += 1;
        }
        1.0 / n as f32
    });
    image.save("mandelbrot.exr")?;
    Ok(())
}
