#[macro_use]
extern crate criterion;
extern crate simdeez;
extern crate simdnoise;
use criterion::*;
use criterion::Fun;
use simdnoise::*;
use std::time::Duration;

fn d4(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_4d(8, 8, 8, 8).wrap();
   
    c.bench(
        "fbm 4d",
        Benchmark::new(
            "scalar 4d", move |b| b.iter( || unsafe { scalar::get_4d_noise(&setting) }))
        .with_function(
            "sse2 4d", move |b| b.iter( || unsafe { sse2::get_4d_noise(&setting) }))
         .with_function(
            "sse41 4d", move |b| b.iter( || unsafe { sse41::get_4d_noise(&setting) }))
       .with_function(
            "avx2 4d", move |b| b.iter( || unsafe { avx2::get_4d_noise(&setting) }))
       .sample_size(10)
       .warm_up_time(Duration::from_millis(1))
       .measurement_time(Duration::from_secs(5)));

}
fn d3(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_3d(32, 32, 32).wrap();
  c.bench(
        "fbm 3d",
        Benchmark::new(
            "scalar 3d", move |b| b.iter( || unsafe { scalar::get_3d_noise(&setting) }))
        .with_function(
            "sse2 3d", move |b| b.iter( || unsafe { sse2::get_3d_noise(&setting) }))
         .with_function(
            "sse41 3d", move |b| b.iter( || unsafe { sse41::get_3d_noise(&setting) }))
       .with_function(
            "avx2 3d", move |b| b.iter( || unsafe { avx2::get_3d_noise(&setting) }))
       .sample_size(10)
       .warm_up_time(Duration::from_millis(1))
       .measurement_time(Duration::from_secs(5)));
}

fn d2(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_2d(256, 256).wrap();
  c.bench(
        "fbm 2d",
        Benchmark::new(
            "scalar 2d", move |b| b.iter( || unsafe { scalar::get_2d_noise(&setting) }))
        .with_function(
            "sse2 2d", move |b| b.iter( || unsafe { sse2::get_2d_noise(&setting) }))
         .with_function(
            "sse41 2d", move |b| b.iter( || unsafe { sse41::get_2d_noise(&setting) }))
       .with_function(
            "avx2 2d", move |b| b.iter( || unsafe { avx2::get_2d_noise(&setting) }))
       .sample_size(10)
       .warm_up_time(Duration::from_millis(1))
       .measurement_time(Duration::from_secs(5)));
}

fn d1(c: &mut Criterion) {
    let setting = NoiseBuilder::fbm_1d(1024).wrap();
  c.bench(
        "fbm 1d",
        Benchmark::new(
            "scalar 1d", move |b| b.iter( || unsafe { scalar::get_1d_noise(&setting) }))
        .with_function(
            "sse2 1d", move |b| b.iter( || unsafe { sse2::get_1d_noise(&setting) }))
         .with_function(
            "sse41 1d", move |b| b.iter( || unsafe { sse41::get_1d_noise(&setting) }))
       .with_function(
            "avx2 1d", move |b| b.iter( || unsafe { avx2::get_1d_noise(&setting) }))
       .sample_size(10)
       .warm_up_time(Duration::from_millis(1))
       .measurement_time(Duration::from_secs(5)));
}
fn d2_cell(c: &mut Criterion) {
    let setting = NoiseBuilder::cellular_2d(1024, 1024).wrap();
  c.bench(
        "cellular 2d",
        Benchmark::new(
            "scalar 2d", move |b| b.iter( || unsafe { scalar::get_2d_noise(&setting) }))
        .with_function(
            "sse2 2d", move |b| b.iter( || unsafe { sse2::get_2d_noise(&setting) }))
         .with_function(
            "sse41 2d", move |b| b.iter( || unsafe { sse41::get_2d_noise(&setting) }))
       .with_function(
            "avx2 2d", move |b| b.iter( || unsafe { avx2::get_2d_noise(&setting) }))
       .sample_size(10)
       .warm_up_time(Duration::from_millis(1))
       .measurement_time(Duration::from_secs(5)));
}
fn d3_cell(c: &mut Criterion) {
    let setting = NoiseBuilder::cellular_3d(32, 32, 32).wrap();
  c.bench(
        "cellular 3d",
        Benchmark::new(
            "scalar 3d", move |b| b.iter( || unsafe { scalar::get_3d_noise(&setting) }))
        .with_function(
            "sse2 3d", move |b| b.iter( || unsafe { sse2::get_3d_noise(&setting) }))
         .with_function(
            "sse41 3d", move |b| b.iter( || unsafe { sse41::get_3d_noise(&setting) }))
       .with_function(
            "avx2 3d", move |b| b.iter( || unsafe { avx2::get_3d_noise(&setting) }))
       .sample_size(10)
       .warm_up_time(Duration::from_millis(1))
       .measurement_time(Duration::from_secs(5)));
}
criterion_group!(benches, d4, d3, d2, d1, d2_cell, d3_cell);
criterion_main!(benches);
