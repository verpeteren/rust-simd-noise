use crate::noise::simplex_32::{
    simplex_1d, simplex_1d_deriv, simplex_2d, simplex_2d_deriv, simplex_3d, simplex_3d_deriv,
    simplex_4d, simplex_5d, simplex_6d,
};

use simdeez::prelude::*;

#[inline(always)]
pub fn turbulence_1d<S: Simd>(
    mut x: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut amp = S::Vf32::set1(1.0);
    let mut result = simplex_1d::<S>(x, seed).abs();

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lacunarity;
        amp *= gain;
        result += (simplex_1d::<S>(x, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lacunarity;
        amp *= gain;
        result += (simplex_1d::<S>(x, seed) * amp).abs() * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
pub fn turbulence_2d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_2d::<S>(x, y, seed).abs();

    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        amp *= gain;
        result += (simplex_2d::<S>(x, y, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        amp *= gain;
        result += (simplex_2d::<S>(x, y, seed) * amp).abs() * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
pub fn turbulence_3d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_3d::<S>(x, y, z, seed).abs();
    let mut amp = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result += (simplex_3d::<S>(x, y, z, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result += (simplex_3d::<S>(x, y, z, seed) * amp).abs() * S::Vf32::set1(frac);
    }

    result
}

// ---------------------------------------------------------------------------
// Turbulence value + analytic gradient (issue #24).
//
// These mirror `turbulence_{1,2,3}d` octave for octave, so the value is
// bit-identical. Turbulence sums `|amp_k * simplex(lacunarity^k * x)|`, and
// d|g|/dx = sign(g) * g'. Each octave's inner quantity is `q = amp_k * simplex`,
// whose derivative w.r.t. the input coordinate is `amp_k * lacunarity^k * s'`
// (the `lacunarity^k` chain factor is tracked by `lac_pow`). We select the sign
// of `q` with a branchless `cmp_lt(0).blendv(+d, -d)`. The caller applies the
// outer `frequency` factor, as for the other gradient helpers.
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn turbulence_1d_deriv<S: Simd>(
    mut x: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> (S::Vf32, S::Vf32) {
    let zero = S::Vf32::zeroes();
    let (s0, d0) = simplex_1d_deriv::<S>(x, seed);
    let mut result = s0.abs();
    let mut deriv = s0.cmp_lt(zero).blendv(d0, zero - d0);
    let mut amp = S::Vf32::set1(1.0);
    let mut lac_pow = S::Vf32::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lacunarity;
        amp *= gain;
        lac_pow *= lacunarity;
        let (s, d) = simplex_1d_deriv::<S>(x, seed);
        let q = s * amp;
        result += q.abs();
        let base = d * amp * lac_pow;
        deriv += q.cmp_lt(zero).blendv(base, zero - base);
    }

    if frac > 0.0 {
        x *= lacunarity;
        amp *= gain;
        lac_pow *= lacunarity;
        let fracv = S::Vf32::set1(frac);
        let (s, d) = simplex_1d_deriv::<S>(x, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let base = d * amp * lac_pow;
        deriv += q.cmp_lt(zero).blendv(base, zero - base) * fracv;
    }

    (result, deriv)
}

#[inline(always)]
pub fn turbulence_2d_deriv<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> (S::Vf32, [S::Vf32; 2]) {
    let zero = S::Vf32::zeroes();
    let (s0, d0) = simplex_2d_deriv::<S>(x, y, seed);
    let mut result = s0.abs();
    let neg0 = s0.cmp_lt(zero);
    let mut dx = neg0.blendv(d0[0], zero - d0[0]);
    let mut dy = neg0.blendv(d0[1], zero - d0[1]);
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
        let (s, d) = simplex_2d_deriv::<S>(x, y, seed);
        let q = s * amp;
        result += q.abs();
        let w = amp * lac_pow;
        let bx = d[0] * w;
        let by = d[1] * w;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx);
        dy += neg.blendv(by, zero - by);
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf32::set1(frac);
        let (s, d) = simplex_2d_deriv::<S>(x, y, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let w = amp * lac_pow;
        let bx = d[0] * w;
        let by = d[1] * w;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx) * fracv;
        dy += neg.blendv(by, zero - by) * fracv;
    }

    (result, [dx, dy])
}

#[inline(always)]
pub fn turbulence_3d_deriv<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> (S::Vf32, [S::Vf32; 3]) {
    let zero = S::Vf32::zeroes();
    let (s0, d0) = simplex_3d_deriv::<S>(x, y, z, seed);
    let mut result = s0.abs();
    let neg0 = s0.cmp_lt(zero);
    let mut dx = neg0.blendv(d0[0], zero - d0[0]);
    let mut dy = neg0.blendv(d0[1], zero - d0[1]);
    let mut dz = neg0.blendv(d0[2], zero - d0[2]);
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
        let (s, d) = simplex_3d_deriv::<S>(x, y, z, seed);
        let q = s * amp;
        result += q.abs();
        let w = amp * lac_pow;
        let bx = d[0] * w;
        let by = d[1] * w;
        let bz = d[2] * w;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx);
        dy += neg.blendv(by, zero - by);
        dz += neg.blendv(bz, zero - bz);
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf32::set1(frac);
        let (s, d) = simplex_3d_deriv::<S>(x, y, z, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let w = amp * lac_pow;
        let bx = d[0] * w;
        let by = d[1] * w;
        let bz = d[2] * w;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx) * fracv;
        dy += neg.blendv(by, zero - by) * fracv;
        dz += neg.blendv(bz, zero - bz) * fracv;
    }

    (result, [dx, dy, dz])
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_4d<S: Simd>(
    mut x: S::Vf32,
    mut y: S::Vf32,
    mut z: S::Vf32,
    mut w: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: f32,
    seed: i32,
) -> S::Vf32 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed).abs();
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
        result += (simplex_4d::<S>(x, y, z, w, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        result += (simplex_4d::<S>(x, y, z, w, seed) * amp).abs() * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_5d<S: Simd>(
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
    let mut result = simplex_5d::<S>(x, y, z, w, u, seed).abs();
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
        result += (simplex_5d::<S>(x, y, z, w, u, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        result += (simplex_5d::<S>(x, y, z, w, u, seed) * amp).abs() * S::Vf32::set1(frac);
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_6d<S: Simd>(
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
    let mut result = simplex_6d::<S>(x, y, z, w, u, t, seed).abs();
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
        result += (simplex_6d::<S>(x, y, z, w, u, t, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        result += (simplex_6d::<S>(x, y, z, w, u, t, seed) * amp).abs() * S::Vf32::set1(frac);
    }

    result
}
