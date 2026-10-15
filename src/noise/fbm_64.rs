use crate::noise::simplex_64::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};

use simdeez::prelude::*;

#[inline(always)]
pub fn fbm_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::Vf64::set1(1.0);
    let mut result = simplex_1d::<S>(x, seed);

    for _ in 1..octaves {
        x *= lacunarity;
        amp *= gain;
        result = (simplex_1d::<S>(x, seed) * amp) + result;
    }

    result
}

#[inline(always)]
pub fn fbm_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_2d::<S>(x, y, seed);
    let mut amp = S::Vf64::set1(1.0);

    for _ in 1..octaves {
        x *= lac;
        y *= lac;
        amp *= gain;
        result = (simplex_2d::<S>(x, y, seed) * amp) + result;
    }

    result
}

#[inline(always)]
pub fn fbm_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_3d::<S>(x, y, z, seed);
    let mut amp = S::Vf64::set1(1.0);
    for _ in 1..octaves {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result = (simplex_3d::<S>(x, y, z, seed) * amp) + result;
    }
    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn fbm_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed);
    let mut amp = S::Vf64::set1(1.0);

    for _ in 1..octaves {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        result += simplex_4d::<S>(x, y, z, w, seed) * amp ;
    }

    result
}
