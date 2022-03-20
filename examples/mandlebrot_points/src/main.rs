use complex::Complex;
use jotter::{Image, Rect, View, Vector};

mod complex;

fn main() -> Result<(), jotter::Error> {
    const DIM: usize = 4096;
    const DIM_F: f32 = DIM as f32;
    let rect = Rect::with_bounds(-2.0, 1.0, -1.5, 1.5);
    let mut image = Image::new(DIM, DIM, 1.0);
    let mut view = View::new(&mut image, rect);
    for y in 0..DIM {
        for x in 0..DIM {
            let point = Vector::new(x as f32 / DIM_F, y as f32 / DIM_F);
            let point = rect.from_local(point);
            let point: Complex = point.into();
            let mut z: Complex = 0.0.into();
            let mut n = 0;
            while n < 64 && z.abs() <= 2.0 {
                z = z * z + point;
                let v: Vector = z.into();
                view.splat(v, 0.005);
                n += 1;
            }
        }
    }
    image.save("mandelbrot_points.exr")?;
    Ok(())
}
