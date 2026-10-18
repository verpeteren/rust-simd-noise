use crate::noise::simplex_32::{
    simplex_1d, simplex_2d, simplex_3d, simplex_4d, simplex_5d, simplex_6d,
};

use simdeez::prelude::*;

#[inline(always)]
pub fn ridge_1d<S: Simd>(
    mut x: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut amp = S::Vf32::set1(1.0);
    let mut result = S::Vf32::set1(1.0) - simplex_1d::<S>(x, seed).abs();

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lacunarity;
        amp *= gain;
        result += S::Vf32::neg_mul_add(simplex_1d::<S>(x, seed).abs(), amp, S::Vf32::set1(1.0));
    }

    if frac > 0.0 {
        x *= lacunarity;
        amp *= gain;
        result += S::Vf32::neg_mul_add(simplex_1d::<S>(x, seed).abs(), amp, S::Vf32::set1(1.0))
            * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
pub fn ridge_2d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = S::Vf32::set1(1.0) - simplex_2d::<S>(x, y, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        amp *= gain;
        result += S::Vf32::neg_mul_add(simplex_2d::<S>(x, y, seed).abs(), amp, S::Vf32::set1(1.0));
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        amp *= gain;
        result += S::Vf32::neg_mul_add(simplex_2d::<S>(x, y, seed).abs(), amp, S::Vf32::set1(1.0))
            * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
pub fn ridge_3d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = S::Vf32::set1(1.0) - simplex_3d::<S>(x, y, z, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result += S::Vf32::neg_mul_add(
            simplex_3d::<S>(x, y, z, seed).abs(),
            amp,
            S::Vf32::set1(1.0),
        );
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result += S::Vf32::neg_mul_add(
            simplex_3d::<S>(x, y, z, seed).abs(),
            amp,
            S::Vf32::set1(1.0),
        ) * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn ridge_4d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    mut w: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = S::Vf32::set1(1.0) - simplex_4d::<S>(x, y, z, w, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        result = result + S::Vf32::set1(1.0) - (simplex_4d::<S>(x, y, z, w, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        result += (S::Vf32::set1(1.0) - (simplex_4d::<S>(x, y, z, w, seed) * amp).abs())
            * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn ridge_5d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    mut w: S::Vf32,
    mut u: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = S::Vf32::set1(1.0) - simplex_5d::<S>(x, y, z, w, u, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        result = result + S::Vf32::set1(1.0) - (simplex_5d::<S>(x, y, z, w, u, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        result += (S::Vf32::set1(1.0) - (simplex_5d::<S>(x, y, z, w, u, seed) * amp).abs())
            * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn ridge_6d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    mut w: S::Vf32,
    mut u: S::Vf32,
    mut t: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = S::Vf32::set1(1.0) - simplex_6d::<S>(x, y, z, w, u, t, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        result =
            result + S::Vf32::set1(1.0) - (simplex_6d::<S>(x, y, z, w, u, t, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        result += (S::Vf32::set1(1.0) - (simplex_6d::<S>(x, y, z, w, u, t, seed) * amp).abs())
            * S::Vf32::set1(frac);
    }

    result
}
