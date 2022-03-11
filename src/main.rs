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
    let points = [
        Point::new(0.25, 0.25),
        Point::new(0.25, 0.75),
        Point::new(0.75, 0.25),
        Point::new(0.75, 0.75),
        Point::new(0.5, 0.5),
    ];
    let wf = W as f32;
    for point in points.iter() {
        let cx = (point.x * wf) as i32;
        let cy = (point.y * wf) as i32;
        for x in (cx - R as i32)..(cx + R as i32) {
            for y in (cy - R as i32)..(cy + R as i32) {
                let xf = (x - cx) as f32;
                let yf = (y - cy) as f32;
                let r = (xf * xf + yf * yf) / R as f32;
                img.put_pixel((x + R) as u32, (y + R) as u32, image::Rgb([r; 3]));
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
