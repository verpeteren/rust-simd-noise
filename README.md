<img align="left" width="120" src="https://raw.githubusercontent.com/jackmott/rust-simd-noise/master/logo.jpg"/>

# SIMDNoise
[![](https://img.shields.io/crates/v/simdnoise.svg)](https://crates.io/crates/simdnoise) [![](https://docs.rs/simdnoise/badge.svg)](https://docs.rs/simdnoise)

Super fast SIMD noise library for Rust. PRs welcome!
This library was created by Jack Mott.
The current maintainer is Peter Reijnders.

## Features

* Gradient Noise (Simplex aka Perlin) 1D,2D,3D,4D
* Fractal Brownian Motion, Ridge, and Turbulence
* Cellular Noise (aka Voroni) 2D, 3D
* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
* AVX2 version also leverages FMA3
* Runtime detection picks the best available instruction set
* Provide a seed value to randomize your noise result

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

## Disabling SIMD (the `scalar` feature)

By default the public API uses runtime CPU feature detection and executes the
fastest available SIMD engine (SSE2/SSE41/AVX2 on x86, NEON on arm64). Some
toolchains cannot compile or run those vendor SIMD intrinsics — most notably the
Cranelift rustc backend (`rustc_codegen_cranelift`), popular for fast
hot-reloaded debug builds, which only partially implements the `std::arch`
intrinsics and *panics at runtime* if an unimplemented one is executed.

Enabling the `scalar` feature routes the entire public API through the scalar
backend, so no CPU SIMD intrinsic is ever executed:

```toml
[dependencies]
simdnoise = { version = "3", features = ["scalar"] }
```

This makes the crate usable under such toolchains (e.g. with Cranelift for
hot-reloaded debug builds). The output is the scalar engine's output, which
differs numerically from the SIMD engines only within normal floating-point
tolerance.

**Caveat:** `simdeez` always *compiles* its architecture-gated SIMD engines
(they are selected with `#[cfg(target_arch = ...)]` and cannot be disabled by a
feature). Under a backend with incomplete intrinsic support the compile-time
warnings for that unused SIMD code therefore remain; the `scalar` feature only
guarantees that scalar code is what actually *runs*, which is what avoids the
runtime panic. On a normal LLVM build the feature compiles cleanly.

## WebAssembly

`simdnoise` works on `wasm32` with no code changes: on that target `simdeez`
dispatches to its wasm32 **SIMD128** engine automatically, and the ordinary API
(`NoiseBuilder`, `get_*_noise`) just works.

### The gotcha: no runtime SIMD detection on wasm

On x86 and ARM, `simdnoise` detects CPU features **at runtime** and transparently
falls back to the best available engine (down to scalar) — so a single binary
runs anywhere. **WebAssembly has no such runtime detection.** On `wasm32`,
`simdeez` therefore *always* emits its SIMD128 engine; there is no automatic
fallback.

The consequence: a default `wasm32` build uses SIMD128 unconditionally, so it
**only runs in a SIMD128-capable runtime**. In a runtime without SIMD128 the
module fails to *instantiate* (a `WebAssembly.CompileError` / validation error) —
it does not silently run slower, it does not load at all.

You have two options, and you must pick one up front:

1. **Target SIMD128 (recommended).** Build with the feature enabled and run in a
   SIMD128-capable runtime — every current browser, Node ≥ 16, Deno, wasmtime:

   ```sh
   RUSTFLAGS="-C target-feature=+simd128" cargo build --target wasm32-unknown-unknown
   ```

2. **Avoid SIMD128 entirely.** Build against the [`scalar`](#disabling-simd-the-scalar-feature)
   backend (and *without* `+simd128`) so no SIMD128 instruction is emitted; the
   module then runs in any wasm runtime:

   ```sh
   cargo build --target wasm32-unknown-unknown --features scalar
   ```

A runnable, headless `wasm-bindgen` example — covering both paths — lives in
[`wasm-example/`](wasm-example/).

## Get a block of noise with runtime SIMD detection

The library will, at runtime, pick the fastest available options between SSE2, SSE41, and AVX2.

```rust
// Get a block of 2d fbm noise with default settings, 100 x 100, with values scaled to the range [0,1]
let noise = NoiseBuilder::fbm_2d(100, 100).generate_scaled(0.0, 1.0);

// Get a block of 3d ridge noise, custom settings, 32x32x32 unscaled
let (noise, min, max) = NoiseBuilder::ridge_3d(32, 32, 32)
    .with_freq_3d(0.05, 0.04, 0.01)
    .with_octaves(5)
    .with_gain(2.0)
    .with_seed(1337)
    .with_lacunarity(0.5)
    .generate();
```

## Call noise functions directly
Sometimes you need something other than a block, like the points on the surface of a sphere.
Sometimes you may want to use SSE41 even with AVX2 is available.

```rust
let noise_setting = NoiseBuilder::ridge_3d(32, 32, 32)
    .with_freq_3d(0.05, 0.04, 0.01)
    .with_octaves(5)
    .with_gain(2.0)
    .with_lacunarity(0.5)
    .wrap();

// get a block of noise with the sse41 version, using the above settings
let (noise, min, max) = unsafe { simdnoise::intrinsics::sse41::get_2d_noise(&noise_setting) };

// send your own SIMD x,y values to the noise functions directly
unsafe {
    // sse2 simplex noise
    let x = _mm_set1_ps(5.0);
    let y = _mm_set1_ps(10.0);
    let f: __m128 = simdnoise::intrinsics::sse2::simplex_2d(x, y);

    // avx2 turbulence
    let x = _mm256_set1_ps(5.0);
    let y = _mm256_set1_ps(10.0);
    let freq = _mm256_set1_ps(1.0);
    let lacunarity = _mm256_set1_ps(0.5);
    let gain = _mm256_set1_ps(2.0);
    let octaves = 3;
    let seed = 1337;
    let f_turbulence: __m256 = simdnoise::intrinsics::avx2::turbulence_2d(x, y, lacunarity, gain, octaves, seed);
};

```
