use image::ImageBuffer;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

const R: i32 = 5;
const W: i32 = 512;

fn main() -> BoxResult<()> {
    let dim = (W + 2 * R) as u32;
    let mut img = ImageBuffer::from_pixel(dim, dim, image::Rgb([1f32; 3]));
    let wf = W as f32;
    let mut x = 1489;
    let mut y = 653;
    for _ in 0..4096 {
        x = (1489 * x) % 503;
        y = (653 * y) % 509;
        let point = Point::new(x as f32 / wf, y as f32 / wf);
        let cx = (point.x * wf) as i32;
        let cy = (point.y * wf) as i32;
        for x in (cx - R)..(cx + R) {
            for y in (cy - R)..(cy + R) {
                let xf = x as f32 - cx as f32;
                let yf = y as f32 - cy as f32;
                let r = (xf * xf + yf * yf) / (R as f32) / (R as f32);
                let r = r.min(1.0);
                let r = 1.0 - (1.0 - r) * (1.0 - r);
                let r = 3.0 * r * r - 2.0 * r * r * r;
                let r = 1.0 - (1.0 - r) * 0.75;
                let x = (x + R) as u32;
                let y = (y + R) as u32;
                let a = img.get_pixel(x, y).0[0];
                img.put_pixel(x, y, image::Rgb([a * r; 3]));
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
