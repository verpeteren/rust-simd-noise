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
const CELL_NOISE_TYPE: NoiseType = NoiseType::Cellular {
    freq: 0.02,
    distance_function: CellDistanceFunction::Euclidean,
    jitter:0.25
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cell scalar", |b| {
        b.iter(|| unsafe {scalar::get_2d_noise(0.0, 500,0.0,500, CELL_NOISE_TYPE)})
    });
     c.bench_function("cell sse2", |b| {
        b.iter(|| unsafe {sse2::get_2d_noise(0.0, 500,0.0,500, CELL_NOISE_TYPE)})
    });

   c.bench_function("cell sse41", |b| {
        b.iter(|| unsafe {sse41::get_2d_noise(0.0, 500,0.0,500, CELL_NOISE_TYPE)})
    });

   c.bench_function("cell avx2", |b| {
        b.iter(|| unsafe {avx2::get_2d_noise(0.0, 500,0.0,500, CELL_NOISE_TYPE)})
    });


}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
