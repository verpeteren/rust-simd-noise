use crate::noise::simplex_32::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn turbulence_1d<S: Simd>(
    mut x: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut amp = S::Vf32::set1(1.0);
    let mut result = simplex_1d::<S>(x, seed).abs();

    for _ in 1..octaves {
        x = x * lacunarity;
        amp = amp * gain;
        result = result + simplex_1d::<S>(x, seed).abs();
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_2d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_2d::<S>(x, y, seed).abs();

    let mut amp = S::Vf32::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        amp = amp * gain;
        result = result + (simplex_2d::<S>(x, y, seed) * amp).abs();
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_3d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_3d::<S>(x, y, z, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        amp = amp * gain;
        result = result + (simplex_3d::<S>(x, y, z, seed) * amp).abs();
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_4d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    mut w: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    for _ in 1..octaves {
        x = x * lac;
        y = y * lac;
        z = z * lac;
        w = w * lac;
        amp = amp * gain;
        result = result + (simplex_4d::<S>(x, y, z, w, seed) * amp).abs();
    }

    result
}
