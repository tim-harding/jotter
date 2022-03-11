use image::ImageBuffer;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

const RADIUS: i32 = 2;
const RADIUS_F: f32 = RADIUS as f32;

fn main() -> BoxResult<()> {
    let dims = (4096u32, 1024u32);
    let dims_f = (dims.0 as f32, dims.1 as f32);
    let mut img = ImageBuffer::from_pixel(
        dims.0 + RADIUS as u32 * 2,
        dims.1 + RADIUS as u32 * 2,
        image::Rgb([1f32; 3]),
    );
    for r_base in 0..4096 {
        let r = r_base as f32 / 4096.0 * 4.0;
        let mut x = 0.5;
        for _ in 0..16 {
            x = r * x * (1.0 - x);
        }
        let r = r / 4.0;
        let cr = (r * dims_f.0) as i32;
        let cx = (x * dims_f.1) as i32;
        for r in (cr - RADIUS)..(cr + RADIUS) {
            for x in (cx - RADIUS)..(cx + RADIUS) {
                let xf = r as f32 - cr as f32;
                let yf = x as f32 - cx as f32;
                let t = (xf * xf + yf * yf) / RADIUS_F / RADIUS_F;
                let t = t.min(1.0);
                let t = 3.0 * t * t - 2.0 * t * t * t;
                let t = 1.0 - (1.0 - t) * 0.5;
                let r = (r + RADIUS) as u32;
                let x = (x + RADIUS) as u32;
                let a = img.get_pixel(r, x).0[0];
                img.put_pixel(r, x, image::Rgb([a * t; 3]));
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
