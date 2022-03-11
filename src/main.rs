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
    let mut x = 7;
    let mut y = 11;
    for _ in 0..4096 {
        x = (7 * x) % 503;
        y = (11 * y) % 509;
        let point = Point::new(x as f32 / wf, y as f32 / wf);
        let cx = (point.x * wf) as i32;
        let cy = (point.y * wf) as i32;
        for x in (cx - R)..(cx + R) {
            for y in (cy - R)..(cy + R) {
                let xf = (x - cx) as f32;
                let yf = (y - cy) as f32;
                let r = (xf * xf + yf * yf) / (R as f32) / (R as f32);
                img.put_pixel((x + R) as u32, (y + R) as u32, image::Rgb([r; 3]));
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
