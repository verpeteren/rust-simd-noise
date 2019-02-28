#[macro_use]
extern crate criterion;
extern crate simdeez;
extern crate simdnoise;
use criterion::Criterion;
use criterion::Fun;
use simdnoise::*;

fn d4(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_4d(8,8,8,8).wrap();
    let scalar = Fun::new("Scalar 4D", move |b, _i| {
        b.iter(|| unsafe {scalar::get_1d_noise(&setting)})
    });
    let sse2 = Fun::new("SSE2 4D", move |b, _i| {
        b.iter(|| unsafe { sse2::get_4d_noise(&setting) })
    });
    let sse41 = Fun::new("SSE41 4D", move |b, _i| {
        b.iter(|| unsafe { sse41::get_4d_noise(&setting) })
    });
    let avx2 = Fun::new("AVX2 4D", move |b, _i| {
        b.iter(|| unsafe { avx2::get_4d_noise(&setting) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("4D", functions, 0);
}
fn d3(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_3d(32,32,32).wrap();
    let scalar = Fun::new("Scalar 3D", move |b, _i| {
        b.iter(|| unsafe {scalar::get_3d_noise(&setting)})
    });
    let sse2 = Fun::new("SSE2 3D", move |b, _i| {
        b.iter(|| unsafe { sse2::get_3d_noise(&setting) })
    });
    let sse41 = Fun::new("SSE41 3D", move |b, _i| {
        b.iter(|| unsafe { sse41::get_3d_noise(&setting) })
    });
    let avx2 = Fun::new("AVX2 3D", move |b, _i| {
        b.iter(|| unsafe { avx2::get_3d_noise(&setting) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("3D", functions, 0);
}

fn d2(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_2d(256,256).wrap();
    let scalar = Fun::new("Scalar 2D", move |b, _i| {
        b.iter(|| unsafe { scalar::get_2d_noise(&setting)})
    });

    let sse2 = Fun::new("SSE2 2D", move |b, _i| {
        b.iter(|| unsafe { sse2::get_2d_noise(&setting) })
    });
    let sse41 = Fun::new("SSE41 2D", move |b, _i| {
        b.iter(|| unsafe { sse41::get_2d_noise(&setting) })
    });
    let avx2 = Fun::new("AVX2 2D", move |b, _i| {
        b.iter(|| unsafe { avx2::get_2d_noise(&setting) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("2D", functions, 0);
}

fn d1(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_1d(1024).wrap();
    let scalar = Fun::new("Scalar 1D", move |b, _i| {
        b.iter(|| unsafe {scalar::get_1d_noise(&setting)})
    });
    let sse2 = Fun::new("SSE2 1D", move |b, _i| {
        b.iter(|| unsafe { sse2::get_1d_noise(&setting) })
    });
    let sse41 = Fun::new("SSE41 1D", move |b, _i| {
        b.iter(|| unsafe { sse41::get_1d_noise(&setting) })
    });
    let avx2 = Fun::new("AVX2 1D", move |b, _i| {
        b.iter(|| unsafe { avx2::get_1d_noise(&setting) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("1D", functions, 0);
}
fn d2_cell(c: &mut Criterion) {
    let setting = NoiseBuilder::cellular_2d(1024,1024).wrap();
/*    let scalar = Fun::new("Scalar", move |b, _i| {
        b.iter(|| unsafe {scalar::get_2d_noise(&setting)})
    });*/
    let sse2 = Fun::new("SSE2", move |b, _i| {
        b.iter(|| unsafe { sse2::get_2d_noise(&setting) })
    });
    let sse41 = Fun::new("SSE41", move |b, _i| {
        b.iter(|| unsafe { sse41::get_2d_noise(&setting) })
    });
    let avx2 = Fun::new("AVX2", move |b, _i| {
        b.iter(|| unsafe { avx2::get_2d_noise(&setting) })
    });
    let functions = vec![sse2, sse41, avx2];
    c.bench_functions("CELL 2D", functions, 0);
}
fn d3_cell(c: &mut Criterion) {
    let setting = NoiseBuilder::cellular_3d(32,32,32).wrap();
    let scalar = Fun::new("Scalar 3D", move |b, _i| {
        b.iter(|| unsafe {scalar::get_3d_noise(&setting)})
    });
    let sse2 = Fun::new("SSE2 3D", move |b, _i| {
        b.iter(|| unsafe { sse2::get_3d_noise(&setting) })
    });
    let sse41 = Fun::new("SSE41 3D", move |b, _i| {
        b.iter(|| unsafe { sse41::get_3d_noise(&setting) })
    });
    let avx2 = Fun::new("AVX2 3D", move |b, _i| {
        b.iter(|| unsafe { avx2::get_3d_noise(&setting) })
    });
    let functions = vec![scalar, sse2, sse41, avx2];
    c.bench_functions("CELL 3D", functions, 0);
}
criterion_group!(benches, d4, d3, d2, d1, d2_cell, d3_cell);
criterion_main!(benches);
