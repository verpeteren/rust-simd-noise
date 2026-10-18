use crate::noise::simplex_32::{
    simplex_1d, simplex_1d_deriv, simplex_2d, simplex_2d_deriv, simplex_3d, simplex_3d_deriv,
    simplex_4d, simplex_5d, simplex_6d,
};

use simdeez::prelude::*;

#[inline(always)]
pub fn fbm_1d<S: Simd>(
    mut x: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut amp = S::Vf32::set1(1.0);
    let mut result = simplex_1d::<S>(x, seed);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lacunarity;
        amp *= gain;
        result = (simplex_1d::<S>(x, seed) * amp) + result;
    }

    if frac > 0.0 {
        x *= lacunarity;
        amp *= gain;
        result = (simplex_1d::<S>(x, seed) * amp * S::Vf32::set1(frac)) + result;
    }

    result
}

#[inline(always)]
pub fn fbm_2d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_2d::<S>(x, y, seed);
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        amp *= gain;
        result = (simplex_2d::<S>(x, y, seed) * amp) + result;
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        amp *= gain;
        result = (simplex_2d::<S>(x, y, seed) * amp * S::Vf32::set1(frac)) + result;
    }

    result
}

#[inline(always)]
pub fn fbm_3d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_3d::<S>(x, y, z, seed);
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result = (simplex_3d::<S>(x, y, z, seed) * amp) + result;
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result = (simplex_3d::<S>(x, y, z, seed) * amp * S::Vf32::set1(frac)) + result;
    }

    result
}

// ---------------------------------------------------------------------------
// FBM value + analytic gradient (issue #24).
//
// These mirror `fbm_{1,2,3}d` above octave for octave so the value they return
// is bit-identical to the value-only kernels, but they also accumulate the
// gradient with respect to the *input* coordinate. FBM sums octaves of
// `simplex(lacunarity^k * x)`, so by the chain rule each octave contributes an
// extra `lacunarity^k` factor to the derivative on top of its `gain^k`
// amplitude (`lac_pow` tracks `lacunarity^k`). The caller is responsible for
// the outer `frequency` factor, exactly as for the plain-gradient helpers.
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn fbm_1d_deriv<S: Simd>(
    mut x: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> (S::Vf32, S::Vf32) {
    let (mut result, mut deriv) = simplex_1d_deriv::<S>(x, seed);
    let mut amp = S::Vf32::set1(1.0);
    let mut lac_pow = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lacunarity;
        amp *= gain;
        lac_pow *= lacunarity;
        let (v, d) = simplex_1d_deriv::<S>(x, seed);
        result = (v * amp) + result;
        deriv = (d * amp * lac_pow) + deriv;
    }

    if frac > 0.0 {
        x *= lacunarity;
        amp *= gain;
        lac_pow *= lacunarity;
        let fracv = S::Vf32::set1(frac);
        let (v, d) = simplex_1d_deriv::<S>(x, seed);
        result = (v * amp * fracv) + result;
        deriv = (d * amp * lac_pow * fracv) + deriv;
    }

    (result, deriv)
}

#[inline(always)]
pub fn fbm_2d_deriv<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> (S::Vf32, [S::Vf32; 2]) {
    let (mut result, d0) = simplex_2d_deriv::<S>(x, y, seed);
    let mut dx = d0[0];
    let mut dy = d0[1];
    let mut amp = S::Vf32::set1(1.0);
    let mut lac_pow = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        amp *= gain;
        lac_pow *= lac;
        let (v, d) = simplex_2d_deriv::<S>(x, y, seed);
        let w = amp * lac_pow;
        result = (v * amp) + result;
        dx = (d[0] * w) + dx;
        dy = (d[1] * w) + dy;
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf32::set1(frac);
        let (v, d) = simplex_2d_deriv::<S>(x, y, seed);
        let w = amp * lac_pow * fracv;
        result = (v * amp * fracv) + result;
        dx = (d[0] * w) + dx;
        dy = (d[1] * w) + dy;
    }

    (result, [dx, dy])
}

#[inline(always)]
pub fn fbm_3d_deriv<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> (S::Vf32, [S::Vf32; 3]) {
    let (mut result, d0) = simplex_3d_deriv::<S>(x, y, z, seed);
    let mut dx = d0[0];
    let mut dy = d0[1];
    let mut dz = d0[2];
    let mut amp = S::Vf32::set1(1.0);
    let mut lac_pow = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        lac_pow *= lac;
        let (v, d) = simplex_3d_deriv::<S>(x, y, z, seed);
        let w = amp * lac_pow;
        result = (v * amp) + result;
        dx = (d[0] * w) + dx;
        dy = (d[1] * w) + dy;
        dz = (d[2] * w) + dz;
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf32::set1(frac);
        let (v, d) = simplex_3d_deriv::<S>(x, y, z, seed);
        let w = amp * lac_pow * fracv;
        result = (v * amp * fracv) + result;
        dx = (d[0] * w) + dx;
        dy = (d[1] * w) + dy;
        dz = (d[2] * w) + dz;
    }

    (result, [dx, dy, dz])
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn fbm_4d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    mut w: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed);
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
        result += simplex_4d::<S>(x, y, z, w, seed) * amp;
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        result += simplex_4d::<S>(x, y, z, w, seed) * amp * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn fbm_5d<S: Simd>(
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
    let mut result = simplex_5d::<S>(x, y, z, w, u, seed);
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
        result += simplex_5d::<S>(x, y, z, w, u, seed) * amp;
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        result += simplex_5d::<S>(x, y, z, w, u, seed) * amp * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn fbm_6d<S: Simd>(
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
    let mut result = simplex_6d::<S>(x, y, z, w, u, t, seed);
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
        result += simplex_6d::<S>(x, y, z, w, u, t, seed) * amp;
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        result += simplex_6d::<S>(x, y, z, w, u, t, seed) * amp * S::Vf32::set1(frac);
    }

    result
}
