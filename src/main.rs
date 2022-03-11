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

fn main() -> BoxResult<()> {
    let r = 5;
    let w = 512;
    let dim = 512 + 2 * r;
    let mut img = ImageBuffer::from_pixel(dim, dim, image::Rgb([1f32; 3]));
    let points = [
        Point::new(0.25, 0.25),
        Point::new(0.25, 0.75),
        Point::new(0.75, 0.25),
        Point::new(0.75, 0.75),
        Point::new(0.5, 0.5),
    ];
    let wf = w as f32;
    for point in points.iter() {
        let cx = (point.x * wf) as u32;
        let cy = (point.y * wf) as u32;
        for x in (r + cx)..(3 * r + cx) {
            for y in (r + cy)..(3*r + cy) {
                img.put_pixel(x, y, image::Rgb([0f32; 3]));
            }
        }
    }
    img.save("render.exr")?;
    Ok(())
}
