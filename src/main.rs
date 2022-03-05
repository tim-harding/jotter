use image::{ImageBuffer};

fn main() -> anyhow::Result<()> {
    let img = ImageBuffer::from_fn(512, 512, |x,_| {
        if x % 2 == 0 {
            image::Rgb([0f32; 3])
        } else {
            image::Rgb([1f32; 3])
        }
    });
    img.save("render.exr")?;
    Ok(())
}
