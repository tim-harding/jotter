use image::ImageBuffer;
use rand::Rng;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

const RADIUS: i32 = 3;
const RADIUS_F: f32 = RADIUS as f32;

fn main() -> BoxResult<()> {
    let mut rng = rand::thread_rng();
    let dims = (4096u32, 1024u32);
    let dims_f = (dims.0 as f32, dims.1 as f32);
    let mut img = ImageBuffer::from_pixel(dims.0, dims.1, image::Rgb([1f32; 3]));
    let count = 1 << 15;
    for r_base in 0..count {
        let r = r_base as f32 / count as f32 * 4.0;
        let mut x = 0.5;
        for _ in 0..1024 {
            let r = r + rng.gen::<f32>() / count as f32;
            x = r * x * (1.0 - x);
            let screen_space = (r * dims_f.0 / 4.0, x * dims_f.1);
            let base_pixel = (screen_space.0 as i32, screen_space.1 as i32);
            for r_offset in -RADIUS..RADIUS {
                for x_offset in -RADIUS..RADIUS {
                    let pixel = (
                        (base_pixel.0 + r_offset) as u32,
                        (base_pixel.1 + x_offset) as u32,
                    );
                    if pixel.0 >= dims.0 || pixel.1 >= dims.1 {
                        continue;
                    }
                    let r_offset_f = pixel.0 as f32 - screen_space.0;
                    let x_offset_f = pixel.1 as f32 - screen_space.1;
                    let t = r_offset_f * r_offset_f + x_offset_f * x_offset_f;
                    let t = (t / RADIUS_F / RADIUS_F).min(1.0);
                    let t = 1.0 - (1.0 - t) / (1 << 14) as f32;
                    let a = img.get_pixel(pixel.0, pixel.1).0[0];
                    img.put_pixel(pixel.0, pixel.1, image::Rgb([a * t; 3]));
                }
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
