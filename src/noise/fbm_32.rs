use crate::noise::simplex_32::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn fbm_1d<S: Simd>(
    mut x: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut amp = S::set1_ps(1.0);
    let mut result = simplex_1d::<S>(x, seed);

    for _ in 1..octaves {
        x = S::mul_ps(x, lacunarity);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, simplex_1d::<S>(x, seed));
    }

    result
}

#[inline(always)]
pub unsafe fn fbm_2d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_2d::<S>(x, y, seed);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        x = S::mul_ps(x, lac);
        y = S::mul_ps(y, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(S::mul_ps(simplex_2d::<S>(x, y, seed), amp), result);
    }

    result
}

#[inline(always)]
pub unsafe fn fbm_3d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_3d::<S>(x, y, z, seed);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        x = S::mul_ps(x, lac);
        y = S::mul_ps(y, lac);
        z = S::mul_ps(z, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(S::mul_ps(simplex_3d::<S>(x, y, z, seed), amp), result);
    }

    result
}

#[inline(always)]
pub unsafe fn fbm_4d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    mut w: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        x = S::mul_ps(x, lac);
        y = S::mul_ps(y, lac);
        z = S::mul_ps(z, lac);
        w = S::mul_ps(w, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::mul_ps(simplex_4d::<S>(x, y, z, w, seed), amp));
    }

    result
}
