use image::ImageBuffer;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BoxResult<()> {
    let mut img = ImageBuffer::from_pixel(512, 512, image::Rgb([1f32; 3]));
    for p in 0..5 {
        let gap = 512 / 5;
        let center = p * gap;
        let r = 10;
        let start = if center < r { 0 } else { center - r };
        let end = if center > 512 - r { 512 } else { center + r };
        for x in start..end {
            img.put_pixel(x, 256, image::Rgb([0f32; 3]));
        }
    }
    img.save("render.exr")?;
    Ok(())
}
