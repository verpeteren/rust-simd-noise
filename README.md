<img align="left" width="120" src="https://raw.githubusercontent.com/jackmott/rust-simd-noise/master/logo.jpg"/>

# SIMDNoise
[![](https://img.shields.io/crates/v/simdnoise.svg)](https://crates.io/crates/simdnoise) [![](https://docs.rs/simdnoise/badge.svg)](https://docs.rs/simdnoise)

Super fast SIMD noise library for Rust. PRs welcome!

## Features

* Gradient Noise (Simplex aka Perlin) 1D,2D,3D,4D
* Fractal Brownian Motion, Ridge, and Turbulence
* Cellular Noise (aka Voroni) 2D, 3D
* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
* AVX2 version also leverages FMA3
* Runtime detection picks the best available instruction set

## Benchmarks
*Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz*
*Single Threaded*
*using Criterion.rs*

### 2D 4k (3840 × 2160)  Fbm Noise, 3 Octaves

| SIMD Set | Time |
|----------|------|
| scalar|  888 ms|
| sse2  |  225 ms|
| sse41 |  186 ms|
| avx2  |  108 ms|

### 3D 128 × 128 x 128  Cell Noise

| SIMD Set | Time |
|----------|------|
| scalar|  1,400 ms|
| sse2  |    128 ms|
| sse41 |     94 ms|
| avx2  |     47 ms|


## Todo

* AVX512 support
* ARM NEON support
* Other noise types

## Get a block of noise with runtime SIMD detection

The library will, at runtime, pick the fastest available options between SSE2, SSE41, and AVX2.

```rust
// Get a block of 2d fbm noise with default settings, 100 x 100, with values scaled to the range [0,1]
let noise = NoiseBuilder::fbm_2d(100, 100).generate_scaled(0.0, 1.0);

// Get a block of 3d ridge noise, custom settings, 32x32x32 unscaled
let (noise, min, max) = NoiseBuilder::ridge_3d(32, 32, 32)
    .with_freq(0.05)
    .with_octaves(5)
    .with_gain(2.0)
    .with_lacunarity(0.5)
    .generate();
```

## Call noise functions directly
Sometimes you need something other than a block, like the points on the surface of a sphere.
Sometimes you may want to use SSE41 even with AVX2 is available.

```rust
let noise_setting = NoiseBuilder::ridge_3d(32, 32, 32)
    .with_freq(0.05)
    .with_octaves(5)
    .with_gain(2.0)
    .with_lacunarity(0.5)
    .wrap();

// get a block of noise with the sse41 version, using the above settings
let (noise, min, max) = unsafe { simdnoise::sse41::get_2d_noise(&noise_setting) };

// send your own SIMD x,y values to the noise functions directly
unsafe {
    // sse2 simplex noise
    let x = _mm_set1_ps(5.0);
    let y = _mm_set1_ps(10.0);
    let f: __m128 = simdnoise::sse2::simplex_2d(x, y);

    // avx2 turbulence
    let x = _mm256_set1_ps(5.0);
    let y = _mm256_set1_ps(10.0);
    let freq = _mm256_set1_ps(1.0);
    let lacunarity = _mm256_set1_ps(0.5);
    let gain = _mm256_set1_ps(2.0);
    let octaves = 3;
    let f_turbulence: __m256 = simdnoise::avx2::turbulence_2d(x, y, lacunarity, gain, octaves);
};

```
