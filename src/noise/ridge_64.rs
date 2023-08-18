use crate::noise::simplex_64::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn ridge_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::Vf64::set1(1.0);
    let mut result = S::sub_pd(S::Vf64::set1(1.0), simplex_1d::<S>(x, seed).abs());

    for _ in 1..octaves {
        x = x * lacunarity;
        amp = amp * gain;
        result = result + S::sub_pd(S::Vf64::set1(1.0), simplex_1d::<S>(x, seed).abs());
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::sub_pd(S::Vf64::set1(1.0), simplex_2d::<S>(x, y, seed).abs());
    let mut amp = S::Vf64::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        amp = amp * gain;
        result = result + S::fnmadd_pd(simplex_2d::<S>(x, y, seed).abs(), amp, S::Vf64::set1(1.0));
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::sub_pd(S::Vf64::set1(1.0), simplex_3d::<S>(x, y, z, seed).abs());
    let mut amp = S::Vf64::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        amp = amp * gain;
        result = result
            + S::fnmadd_pd(
                simplex_3d::<S>(x, y, z, seed).abs(),
                amp,
                S::Vf64::set1(1.0),
            );
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::sub_pd(S::Vf64::set1(1.0), simplex_4d::<S>(x, y, z, w, seed).abs());
    let mut amp = S::Vf64::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        w = w * lac;
        amp = amp * gain;
        result = result
            + S::sub_pd(
                S::Vf64::set1(1.0),
                (simplex_4d::<S>(x, y, z, w, seed) * amp).abs(),
            );
    }

    result
}
