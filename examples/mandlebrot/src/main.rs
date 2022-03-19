use complex::Complex;
use jotter::{Image, Rect, View};

mod complex;

fn main() -> Result<(), jotter::Error> {
    let rect = Rect::with_bounds(-1.44, 1.44, -1.44, 1.44);
    let mut image = Image::new(4096, 4096, 0.0);
    let mut view = View::new(&mut image, rect);
    view.shade(3, |x: f32, y: f32| {
        let point = Complex::new(x, y);
        let mut z: Complex = 0.0.into();
        let mut n = 1;
        while n < 256 && z.abs() <= 2.0 {
            z = z * z + point;
            n += 1;
        }
        1.0 / n as f32
    });
    image.save("mandelbrot.exr")?;
    Ok(())
}
