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
    let mut result = S::Vf64::set1(1.0) - simplex_1d::<S>(x, seed).abs();

    for _ in 1..octaves {
        x = x * lacunarity;
        amp = amp * gain;
        result = result + (S::Vf64::set1(1.0) - simplex_1d::<S>(x, seed).abs());
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::Vf64::set1(1.0);
    let mut result = S::Vf64::set1(1.0) - simplex_2d::<S>(x, y, seed).abs();

    for _ in 1..octaves {
        x = x * lacunarity;
        y = y * lacunarity;
        amp = amp * gain;
        result = result + (S::Vf64::set1(1.0) - simplex_2d::<S>(x, y, seed).abs());
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::Vf64::set1(1.0);
    let mut result = S::Vf64::set1(1.0) - simplex_3d::<S>(x, y, z, seed).abs();

    for _ in 1..octaves {
        x = x * lacunarity;
        y = y * lacunarity;
        z = z * lacunarity;
        amp = amp * gain;
        result = result + (S::Vf64::set1(1.0) - simplex_3d::<S>(x, y, z, seed).abs());
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::Vf64::set1(1.0);
    let mut result = S::Vf64::set1(1.0) - simplex_4d::<S>(x, y, z, w, seed).abs();

    for _ in 1..octaves {
        x = x * lacunarity;
        y = y * lacunarity;
        z = z * lacunarity;
        w = w * lacunarity;
        amp = amp * gain;
        result = result + (S::Vf64::set1(1.0) - simplex_4d::<S>(x, y, z, w, seed).abs());
    }

    result
}
