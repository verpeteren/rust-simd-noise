<img align="left" width="120" src="https://raw.githubusercontent.com/jackmott/rust-simd-noise/master/logo.jpg"/>

# SIMDNoise
[![](https://img.shields.io/crates/v/simdnoise.svg)](https://crates.io/crates/simdnoise) [![](https://docs.rs/simdnoise/badge.svg)](https://docs.rs/simdnoise)

Super fast SIMD noise library for Rust. PRs welcome!
This library was created by Jack Mott.
The current maintainer is Peter Reijnders.

## Features

* Gradient Noise (Simplex aka Perlin) 1D,2D,3D,4D, and **5D/6D (experimental)**
* Fractal Brownian Motion, Ridge, and Turbulence
* Cellular Noise (aka Voroni) 1D,2D,3D,4D, and **5D/6D (experimental)**
* `f32` and `f64` output for every algorithm and dimension
* SSE2, SSE41, AVX2, and AVX-512 instruction sets, plus NEON and a non-SIMD fallback
* AVX2 version also leverages FMA3
* Runtime detection picks the best available instruction set
* Provide a seed value to randomize your noise result

### Capability matrix

What is implemented, per algorithm and dimension. `v` = implemented & working,
blank = not applicable. Entries marked `v*` are **experimental** (see below).

| Algorithm | knobs | 1D | 2D | 3D | 4D | 5D | 6D |
|---|---|:--:|:--:|:--:|:--:|:--:|:--:|
| Gradient (raw Simplex) | seed, freq | v | v | v | v | v* | v* |
| FBM | + lacunarity, gain, octaves | v | v | v | v | v* | v* |
| Ridge | + lacunarity, gain, octaves | v | v | v | v | v* | v* |
| Turbulence | + lacunarity, gain, octaves | v | v | v | v | v* | v* |
| Cellular (Voronoi F1) | seed, freq, distance_fn, return_type, jitter | v | v | v | v | v* | v* |
| Cellular2 (Voronoi F2) | + return_type2, index0, index1 | v | v | v | v | v* | v* |

Every `v`/`v*` cell is available for both `f32` (`generate()`, …) and `f64`
(`generate_64()`, …).

#### Engines

| Scalar | SSE2 | SSE4.1 | AVX2 | AVX-512 | NEON (aarch64) | WASM simd128 |
|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| v | v | v | v | v | v | v |

Selection is automatic at runtime (x86: AVX-512 → AVX2 → SSE4.1 → SSE2 → scalar;
aarch64: NEON), or compile-time via the `scalar` feature / the per-engine
`intrinsics::{engine}` modules. NEON `f64` cellular is excluded (its `rsqrt`
approximation produces NaN).

#### ⚠️ Experimental: 5D and 6D

5D and 6D noise has **no canonical reference implementation** (FastNoiseSIMD and
friends stop at 4D), so these dimensions are experimental:

* The simplex 5D/6D kernels are classic simplex extended by hand; the
  `[-1, 1]` normalization constant is **empirically calibrated**, not derived.
* Classic simplex develops visible surflet artifacts past 4D.

**There are no golden (stored-fixture) tests for 5D/6D yet.** Because the
output values are still expected to change (normalization is being refined and
the kernels have no reference to match), pinning them byte-for-byte would be
premature. Correctness is instead verified by cross-engine agreement,
`f32`↔`f64` agreement, and statistical range/finiteness checks. Golden fixtures
(and the per-architecture NEON regen) will be added once 5D/6D stabilizes.

The API and output values at 5D/6D may change in a future release. 1D–4D are
stable and keep their full per-engine golden-fixture coverage.

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

* Stabilize the experimental 5D/6D noise (golden fixtures, normalization review)
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
