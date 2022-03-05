use image::{ImageBuffer};

fn main() -> anyhow::Result<()> {
    let img = ImageBuffer::from_pixel(512, 512, image::Rgb([1f32;3]));
    img.save("render.exr")?;
    Ok(())
}
