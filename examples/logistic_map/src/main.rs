use jotter::{Image, Rect, Vector, View};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() -> Result<(), jotter::Error> {
    let mut rng = StdRng::seed_from_u64(7);
    let mut image = Image::new(2048, 1024, 1.0);
    let rect = Rect::with_bounds(3.447, 4.0, 0.0, 1.0);
    let mut view = View::new(&mut image, rect);
    let count = 1 << 15;
    for r in 0..count {
        let r = rect.horizontal.to_world(r as f32 / count as f32);
        let mut x = rng.gen::<f32>();
        for _ in 0..1024 {
            let r = r + rng.gen::<f32>() / count as f32;
            x = r * x * (1.0 - x);
            view.splat(&Vector::new(r, x), 2, 1.0 / (1 << 9) as f32);
        }
    }
    image.save("render.exr")?;
    Ok(())
}
