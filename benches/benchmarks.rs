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
    c.bench_function("4d sse2", |b| {
        b.iter(|| unsafe {sse2::get_4d_noise(0.0, 16,0.0,16,0.0,16,0.0,16, NOISE_TYPE)})
    });

    c.bench_function("4d scalar", |b| {
        b.iter(|| scalar::get_4d_noise(0.0, 16,0.0,16,0.0,16,0.0,16, NOISE_TYPE))
    });
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
