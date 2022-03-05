use image::{ImageBuffer};

fn main() -> anyhow::Result<()> {
    let d = 512;
    let img = ImageBuffer::from_fn(d, d, |x,y| {
        let r = (d / 2) as f32;
        let x = x as f32 - r;
        let y = y as f32 - r;
        let r = (x * x + y * y) / (r*r);
        image::Rgb([r; 3])
    });
    img.save("render.exr")?;
    Ok(())
}
