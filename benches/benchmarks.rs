#[macro_use]
extern crate criterion;
extern crate simdnoise;
use simdnoise::*;
use criterion::Criterion;

const NOISE_TYPE: NoiseType = NoiseType::Fbm {
    freq: 0.04,
    lacunarity: 0.5,
    gain: 2.0,
    octaves: 3,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("2d sse2-1", |b| {
        b.iter(|| unsafe {sse2::get_2d_noise(0.0, 500,0.0,500, NOISE_TYPE)})
    });


}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
