use jotter::{Image, Rect, Vector, View};

fn main() -> Result<(), jotter::Error> {
    let mut image = Image::new(2048, 1024, 1.0);
    let rect = Rect::with_bounds(3.447, 4.0, 0.0, 1.0);
    let mut view = View::new(&mut image, rect);
    let count = 1 << 15;
    for r in 0..count {
        let r = rect.horizontal.from_local(r as f32 / count as f32);
        let mut x = 0.5;
        for _ in 0..1024 {
            x = r * x * (1.0 - x);
            let opacity = 1.0 / (1 << 9) as f32;
            view.splat(Vector::new(r, x), opacity);
        }
    }
    image.save("render.exr")?;
    Ok(())
}
