# Migrating to simdnoise 4.0

This guide is written for upgrading from **3.1.6** — the last version published
on crates.io — to **4.0.0**. 4.0 is a clean-break major: it adds a lot (f64
output, analytic gradients, a fill-into-your-own-buffer API, 5D/6D, more
engines) and settles the public-API naming ([#84]) with **no deprecated
aliases**. The compiler will point at every call site; this guide explains each
change.

> Tracking the unpublished 4.0 git branch (sometimes called *sdn4*) rather than
> 3.1.6? The naming settled late in that cycle — see
> [Appendix: 4.0 pre-release renames](#appendix-40-pre-release-git-renames).

[#84]: https://github.com/verpeteren/rust-simd-noise/issues/84

## TL;DR of breaking changes

1. **`with_octaves` takes `f32`, not `u8`** — pass `5.0`, not `5` (fractional
   octaves are now supported, [#14]).
2. **Import the builder traits.** The settings methods moved onto the `Settings`
   and `SimplexSettings` traits: add `use simdnoise::{Settings, SimplexSettings};`.
3. **The low-level engine modules moved under `intrinsics`** and their block
   functions were renamed `get_*` → `generate_*`:
   `simdnoise::sse41::get_2d_noise` → `simdnoise::intrinsics::sse41::generate_2d_noise`.
4. **The raw `simplex` / `cellular` modules are gone** from the public API; use
   `simdnoise::intrinsics` instead.

Everything else (`NoiseBuilder`, the `*Settings` structs, `NoiseType`, the
`Cell*` enums) keeps its name.

[#14]: https://github.com/verpeteren/rust-simd-noise/issues/14

## High-level builder API

The `NoiseBuilder` entry point is unchanged, but two things around it changed.

### `Settings` / `SimplexSettings` are now traits

In 3.1.6 the builder methods were inherent on each `*Settings` struct, so they
worked with no imports. In 4.0 they live on two traits that must be in scope:

- `Settings`: `with_seed`, `with_frequency`, `wrap`, `generate`, `generate_scaled`.
- `SimplexSettings`: `with_lacunarity`, `with_gain`, `with_octaves`.

```rust
use simdnoise::{NoiseBuilder, Settings, SimplexSettings};
```

The cellular-only setters (`with_distance_function`, `with_return_type`,
`with_jitter`, `with_index0`, `with_index1`) stay inherent — no trait needed.

### `with_octaves` is now `f32`

```rust
// 3.1.6
let (noise, min, max) = NoiseBuilder::ridge_3d(32, 32, 32)
    .with_octaves(5)        // u8
    .generate();

// 4.0
use simdnoise::{NoiseBuilder, Settings, SimplexSettings};
let (noise, min, max) = NoiseBuilder::ridge_3d(32, 32, 32)
    .with_octaves(5.0)      // f32 — 5.5 blends halfway into the 6th octave (#14)
    .generate();
```

`generate()` still returns `(Vec<f32>, f32, f32)` and `generate_scaled(min, max)`
still returns `Vec<f32>`; the builder constructors (`fbm_2d`, `ridge_3d`, …) are
unchanged.

## Low-level / direct API

### Engine modules moved under `intrinsics`, block fns renamed

The per-engine modules are now `simdnoise::intrinsics::{engine}`, and their
`get_*` block functions became `generate_*`. They are generic over the engine
type (`S: simdeez::Simd`), as in 3.1.7+.

```rust
// 3.1.6
let (noise, min, max) = unsafe { simdnoise::sse41::get_2d_noise(&noise_setting) };

// 4.0
use simdeez::sse41::Sse41;
let (noise, min, max) =
    unsafe { simdnoise::intrinsics::sse41::generate_2d_noise::<Sse41>(&noise_setting) };
```

| 3.1.6 | 4.0 |
|---|---|
| `simdnoise::{avx2,sse2,sse41,scalar}` | `simdnoise::intrinsics::{avx2,sse2,sse41,scalar}` (plus new `avx512`, `neon`) |
| `{engine}::get_Nd_noise` | `intrinsics::{engine}::generate_Nd_noise::<S>` |
| `{engine}::get_Nd_scaled_noise` | `intrinsics::{engine}::generate_Nd_scaled_noise::<S>` |

### Per-sample kernels: generic + seed, moved under `intrinsics`

The raw per-sample functions (`simplex_2d`, `turbulence_2d`, …) also moved under
`intrinsics::{engine}`, are generic over `S: Simd` (operating on `S::Vf32` lanes
instead of a concrete `__m128`/`__m256`), and take the seed as an argument.

```rust
// 3.1.6
let f: __m128 = unsafe { simdnoise::sse2::simplex_2d(x, y) };

// 4.0
use simdeez::prelude::*;
use simdeez::sse2::Sse2;
let f = unsafe { simdnoise::intrinsics::sse2::simplex_2d::<Sse2>(x, y, seed) };
```

See the crate-level docs for a full worked example.

### The `simplex` and `cellular` modules were removed

3.1.6 exposed `pub mod simplex` and `pub mod cellular` with the raw kernels.
These are now crate-internal. Reach the equivalent functionality through
`simdnoise::intrinsics` (the supported low-level entry point). The configuration
enums are unchanged and still re-exported at the crate root:
`CellDistanceFunction`, `CellReturnType`, `Cell2ReturnType` (plus the new
`cell_strategy` types).

## `NoiseDimensions`

`NoiseDimensions` is normally filled in by `NoiseBuilder`, so most code never
touches it. Two differences if you do:

- Its axis fields were **private** in 3.1.6; in 4.0 they are **public**, named
  uniformly `offset_x..offset_v` (start position) and `extent_x..extent_v`
  (sample count along each axis).
- The supported dimension range grew from **1–4** to **1–6**.

## New in 4.0 (nothing to change; adopt if useful)

- **f64 output** — every entry point has an `_f64` twin
  (`generate_2d_noise_f64`, `Settings::generate_f64`, …).
- **Analytic gradients** ([#24]) — `Settings::generate_with_gradient[_f64]`
  returns a `NoiseAndGradient`/`NoiseAndGradientF64` (values + per-sample
  gradient), with matching free functions `generate_Nd_gradient[_f64]`.
- **Fill-into-your-own-buffer** ([#5]) — `Settings::fill` / `fill_scaled` /
  `fill_range::<T>` and the free `fill_Nd_noise*` functions write into a slice
  you already own instead of allocating a `Vec`.
- **5D and 6D** noise (experimental — the 5D/6D output may still change).
- **Per-axis frequency** — `with_frequency_2d`…`with_frequency_6d` and `frequency_x()`…`frequency_v()`.
- **More engines & the `scalar` feature** — AVX-512 and NEON (aarch64) engines,
  and a `scalar` cargo feature that forces the non-SIMD backend for the whole
  public API.
- **Cellular strategy types** ([#48]) — the `cell_strategy` module.

[#24]: https://github.com/verpeteren/rust-simd-noise/issues/24
[#5]: https://github.com/verpeteren/rust-simd-noise/issues/5
[#48]: https://github.com/verpeteren/rust-simd-noise/issues/48

## Other 4.0 changes

- Crate `edition` is now **2021**, MSRV (`rust-version`) is **1.85**, and the
  SPDX `license` is `MIT OR Apache-2.0`.

---

## Appendix: 4.0 pre-release (git) renames

If you were tracking the 4.0 development branch rather than 3.1.6, these are the
renames that landed near the end of that cycle ([#84]). (3.1.6 users never saw
the old names, so they can skip this section.)

| pre-release | 4.0 |
|---|---|
| `_64` suffix (e.g. `get_2d_noise_64`, `generate_64`, `NoiseAndGradient64`) | `_f64` (`generate_2d_noise_f64`, `generate_f64`, `NoiseAndGradientF64`) |
| `*_into` buffer-fill (e.g. `get_2d_noise_into`, `Settings::generate_into`) | `fill_*` (`fill_2d_noise`, `Settings::fill`) |
| `get_Nd_noise` / `get_Nd_scaled_noise` (free fns & intrinsics) | `generate_Nd_noise` / `generate_Nd_scaled_noise` |
| `get_Nd_gradient` / `get_Nd_fbm_gradient` / … | `generate_Nd_gradient` / `generate_Nd_fbm_gradient` / … |
| `get_gradient_64` / `get_fbm_gradient_64` / `get_ridge_gradient_64` / `get_turbulence_gradient_64` (one dimensionless fn each) | `generate_Nd_gradient_f64` / `generate_Nd_fbm_gradient_f64` / … (now per-dimension, 1–6) |
| `Settings::generate_with_derivative[_64]` | `Settings::generate_with_gradient[_f64]` |
| `Settings::get_freq_x` … `get_freq_v` | `frequency_x()` … `frequency_v()` |
| `Settings::with_freq` / `with_freq_2d` … `with_freq_6d` | `with_frequency` / `with_frequency_2d` … `with_frequency_6d` |
| `NoiseDimensions` field `dim` | `dimension` |
| `NoiseDimensions` fields `x/y/z/w`, `offset5/6`, `width/height/depth/time`, `extent5/6` | `offset_x..offset_v`, `extent_x..extent_v` |
| `simdnoise::noise::{simplex,fbm,cell,…}_{32,64}` kernels | crate-internal; use `simdnoise::intrinsics` |
