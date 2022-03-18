use jotter::Image;
use rand::{rngs::StdRng, Rng, SeedableRng};

const RADIUS: i32 = 2;
const RADIUS_F: f32 = RADIUS as f32;

fn main() -> Result<(), jotter::Error> {
    let mut rng = StdRng::seed_from_u64(7);
    let dims = (2048, 1024);
    let dims_f = (dims.0 as f32, dims.1 as f32);
    let mut img = Image::new(dims.0, dims.1, 1.0);
    let count = 1 << 15;
    let bound = (3.447, 4.0);
    for r_base in 0..count {
        let r = r_base as f32 / count as f32 * (bound.1 - bound.0) + bound.0;
        let mut x = rng.gen::<f32>();
        for _ in 0..1024 {
            let r = r + rng.gen::<f32>() / count as f32;
            x = r * x * (1.0 - x);
            let screen_space = ((r - bound.0) / (bound.1 - bound.0) * dims_f.0, x * dims_f.1);
            let base_pixel = (screen_space.0 as i32, screen_space.1 as i32);
            for pixel_x in ((base_pixel.1 - RADIUS - 1).max(0) as usize)
                ..((base_pixel.1 + RADIUS + 1) as usize).min(dims.1 - 1)
            {
                for pixel_r in ((base_pixel.0 - RADIUS - 1).max(0) as usize)
                    ..((base_pixel.0 + RADIUS + 1) as usize).min(dims.0 - 1)
                {
                    let r_offset_f = pixel_r as f32 - screen_space.0;
                    let x_offset_f = pixel_x as f32 - screen_space.1;
                    let t = r_offset_f * r_offset_f + x_offset_f * x_offset_f;
                    let t = (t / RADIUS_F / RADIUS_F).min(1.0);
                    let t = 1.0 - (1.0 - t) / (1 << 9) as f32;
                    let a = img.get(pixel_r, pixel_x);
                    img.set(pixel_r, pixel_x, a * t);
                }
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
