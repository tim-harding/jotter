use criterion::{criterion_group, criterion_main, Criterion};
use jotter::{Image, Rect, Vector, View};

fn exponentiation() {
    let mut image = Image::new(1024, 1024, 1.0);
    let mut view = View::new(&mut image, Rect::SQUARE);
    let modulus = 1_048_391;
    let base = 6173;
    let mut n = 1;
    for _ in 0..100_000 {
        n = (n * base) % modulus;
        let x = (n % 1024) as f32 / 1024.0;
        let y = (n / 1024) as f32 / 1024.0;
        let p = Vector::new(x, y);
        view.splat(p, 0.5);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("exponentiation", |b| b.iter(|| exponentiation()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
