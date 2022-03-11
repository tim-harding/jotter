use image::ImageBuffer;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

const RADIUS: i32 = 3;
const RADIUS_F: f32 = RADIUS as f32;

fn main() -> BoxResult<()> {
    let dims = (4096u32, 1024u32);
    let dims_f = (dims.0 as f32, dims.1 as f32);
    let mut img = ImageBuffer::from_pixel(
        dims.0 + RADIUS as u32 * 2 + 2,
        dims.1 + RADIUS as u32 * 2 + 2,
        image::Rgb([1f32; 3]),
    );
    let count = 1 << 10;
    for r_base in 0..count {
        let r = r_base as f32 / count as f32 * 4.0;
        let mut x = 0.5;
        for _ in 0..1024 {
            x = r * x * (1.0 - x);
            let r = r / 4.0;
            for r_offset in -RADIUS..RADIUS {
                for x_offset in -RADIUS..RADIUS {
                    // Still not quite right
                    let rf = r_offset as f32;
                    let xf = x_offset as f32;
                    let t = ((rf * rf + xf * xf) / RADIUS_F / RADIUS_F).min(1.0);
                    let t = 1.0 - (1.0 - t) * 0.25;
                    let r = ((r * dims_f.0) as i32 + r_offset + RADIUS + 1) as u32;
                    let x = ((x * dims_f.1) as i32 + x_offset + RADIUS + 1) as u32;
                    let a = img.get_pixel(r, x).0[0];
                    img.put_pixel(r, x, image::Rgb([a * t; 3]));
                }
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
