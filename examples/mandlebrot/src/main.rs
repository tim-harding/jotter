use complex::Complex;
use jotter::{Image, Rect, Vector};

mod complex;

fn main() -> Result<(), jotter::Error> {
    let mut image = Image::new(1024, 1024, 0.0);
    let rect = Rect::with_bounds(-1.0, 1.0, -1.0, 1.0);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let view = &Vector::new(
                x as f32 / image.width() as f32,
                y as f32 / image.height() as f32,
            );
            let point: Complex = rect.to_world(view).into();
            let mut z: Complex = 0.0.into();
            let mut n = 1;
            while n < 64 && z.abs() <= 2.0 {
                z = z * z + point;
                n += 1;
            }
            image.set(x, y, 1.0 / n as f32);
        }
    }
    image.save("mandelbrot.exr")?;
    Ok(())
}
