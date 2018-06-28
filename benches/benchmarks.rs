#[macro_use]
extern crate criterion;
extern crate simdeez;
extern crate simdnoise;
use criterion::Criterion;
use criterion::Fun;
use simdnoise::*;

const NOISE_TYPE: NoiseType = NoiseType::Fbm {
    freq: 0.04,
    lacunarity: 0.5,
    gain: 2.0,
    octaves: 3,
};

fn d4(c: &mut Criterion) {
    let scalar = Fun::new("Scalar 4D", |b, _i| {
        b.iter(|| scalar::get_4d_noise(0.0, 16, 0.0, 16, 0.0, 16, 0.0, 16, NOISE_TYPE))
    });
    let sse2 = Fun::new("SSE2 4D", |b, _i| {
        b.iter(|| unsafe { sse2::get_4d_noise(0.0, 16, 0.0, 16, 0.0, 16, 0.0, 16, NOISE_TYPE) })
    });
    let sse41 = Fun::new("SSE41 4D", |b, _i| {
        b.iter(|| unsafe { sse41::get_4d_noise(0.0, 16, 0.0, 16, 0.0, 16, 0.0, 16, NOISE_TYPE) })
    });
    let avx2 = Fun::new("AVX2 4D", |b, _i| {
        b.iter(|| unsafe { avx2::get_4d_noise(0.0, 16, 0.0, 16, 0.0, 16, 0.0, 16, NOISE_TYPE) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("4D", functions, 0);
}
fn d3(c: &mut Criterion) {
    let scalar = Fun::new("Scalar 3D", |b, _i| {
        b.iter(|| scalar::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, NOISE_TYPE))
    });
    let sse2 = Fun::new("SSE2 3D", |b, _i| {
        b.iter(|| unsafe { sse2::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, NOISE_TYPE) })
    });
    let sse41 = Fun::new("SSE41 3D", |b, _i| {
        b.iter(|| unsafe { sse41::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, NOISE_TYPE) })
    });
    let avx2 = Fun::new("AVX2 3D", |b, _i| {
        b.iter(|| unsafe { avx2::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, NOISE_TYPE) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("3D", functions, 0);
}

fn d2(c: &mut Criterion) {
    let scalar = Fun::new("Scalar 2D", |b, _i| {
        b.iter(|| scalar::get_2d_noise(0.0, 1024, 0.0, 1024, NOISE_TYPE))
    });
    let sse2 = Fun::new("SSE2 2D", |b, _i| {
        b.iter(|| unsafe { sse2::get_2d_noise(0.0, 1024, 0.0, 1024, NOISE_TYPE) })
    });
    let sse41 = Fun::new("SSE41 2D", |b, _i| {
        b.iter(|| unsafe { sse41::get_2d_noise(0.0, 1024, 0.0, 1024, NOISE_TYPE) })
    });
    let avx2 = Fun::new("AVX2 2D", |b, _i| {
        b.iter(|| unsafe { avx2::get_2d_noise(0.0, 1024, 0.0, 1024, NOISE_TYPE) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("2D", functions, 0);
}
#[target_feature(enable = "avx2")]
pub unsafe fn testsimd(startx: f32, w: usize, nt: NoiseType) -> (Vec<f32>,f32,f32) {
    simplex::get_1d_noise::<simdeez::avx2::Avx2>(startx, w, nt)
}


fn d1(c: &mut Criterion) {
    let scalar = Fun::new("Scalar 1D", |b, _i| {
        b.iter(|| scalar::get_1d_noise(0.0, 4096, NOISE_TYPE))
    });
    let sse2 = Fun::new("SSE2 1D", |b, _i| {
        b.iter(|| unsafe { sse2::get_1d_noise(0.0, 4096, NOISE_TYPE) })
    });
    let sse41 = Fun::new("SSE41 1D", |b, _i| {
        b.iter(|| unsafe { sse41::get_1d_noise(0.0, 4096, NOISE_TYPE) })
    });
    let avx2 = Fun::new("AVX2 1D", |b, _i| {
        b.iter(|| unsafe { avx2::get_1d_noise(0.0, 4096, NOISE_TYPE) })
    });
    let avx2new = Fun::new("AVX2 1D new", |b, _i| {
        b.iter(|| unsafe { testsimd(0.0, 4096, NOISE_TYPE) })
    });
    let functions = vec![scalar, sse2, sse41, avx2, avx2new];
    c.bench_functions("1D", functions, 0);
}
const CELL_NOISE_TYPE: NoiseType = NoiseType::Cellular {
    freq: 0.02,
    distance_function: CellDistanceFunction::Euclidean,
    return_type: CellReturnType::Distance,
    jitter: 0.25,
};
fn d2_cell(c: &mut Criterion) {
    let scalar = Fun::new("Scalar 1D", |b, _i| {
        b.iter(|| scalar::get_2d_noise(0.0, 1024, 0.0, 1024, CELL_NOISE_TYPE))
    });
    let sse2 = Fun::new("SSE2 1D", |b, _i| {
        b.iter(|| unsafe { sse2::get_2d_noise(0.0, 1024, 0.0, 1024, CELL_NOISE_TYPE) })
    });
    let sse41 = Fun::new("SSE41 1D", |b, _i| {
        b.iter(|| unsafe { sse41::get_2d_noise(0.0, 1024, 0.0, 1024, CELL_NOISE_TYPE) })
    });
    let avx2 = Fun::new("AVX2 1D", |b, _i| {
        b.iter(|| unsafe { avx2::get_2d_noise(0.0, 1024, 0.0, 1024, CELL_NOISE_TYPE) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("CELL 2D", functions, 0);
}
fn d3_cell(c: &mut Criterion) {
    let scalar = Fun::new("Scalar 3D", |b, _i| {
        b.iter(|| scalar::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, CELL_NOISE_TYPE))
    });
    let sse2 = Fun::new("SSE2 3D", |b, _i| {
        b.iter(|| unsafe { sse2::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, CELL_NOISE_TYPE) })
    });
    let sse41 = Fun::new("SSE41 3D", |b, _i| {
        b.iter(|| unsafe { sse41::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, CELL_NOISE_TYPE) })
    });
    let avx2 = Fun::new("AVX2 3D", |b, _i| {
        b.iter(|| unsafe { avx2::get_3d_noise(0.0, 64, 0.0, 64, 0.0, 64, CELL_NOISE_TYPE) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("CELL 3D", functions, 0);
}
criterion_group!(benches, d4, d3, d2, d1, d2_cell, d3_cell);
criterion_main!(benches);
